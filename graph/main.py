#! /usr/bin/env python3
# vim: expandtab shiftwidth=4 tabstop=4

"""Build a _graph_ of the FEAL-8 network so that we can make an actual plot of the algorithm."""

import re

def parse_rgbstr(rgbstr):
    mtch = re.match(r'^#([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})$', rgbstr)
    assert mtch is not None, rgbstr
    red, green, blue = int(mtch.group(1), 16), int(mtch.group(2), 16), int(mtch.group(3), 16)
    return red, green, blue

#pylint: disable=too-few-public-methods,too-many-instance-attributes,too-many-arguments
class Node:
    counter = 0
    def __init__(self, name, bitsz, xpos, ypos, size=None):
        self.name = name
        self.bitsz = bitsz
        self.inputs = []
        self.idx = Node.counter
        self.value = None
        self.red, self.green, self.blue = parse_rgbstr("#ffffff")
        self.xpos = xpos
        self.ypos = ypos
        self.size = size if size is not None else 0.4
        Node.counter += 1

    def populate_nodes(self, nodes):
        assert isinstance(nodes, dict)
        nodes[self.idx] = {
            "label": self.name,
            "r": self.red, "g": self.green, "b": self.blue,
            "x": 120 * float(self.xpos),
            "y": -120 * float(self.ypos),
            "size": 120 * self.size,
            "bitsize": self.bitsz
        }
#pylint: enable=too-few-public-methods,too-many-arguments

class Input(Node):
    def __init__(self, name, bitsz, **kwargs):
        super().__init__(name, bitsz, **kwargs)
        self.red, self.green, self.blue = parse_rgbstr("#87ceeb") # SkyBlue
        if name.startswith("key"):
            self.red, self.green, self.blue = parse_rgbstr("#ff8c00") # DarkOrange

    def set(self, value):
        self.value = value

    def eval(self):
        return self.value

    def populate_edges(self, edges):
        assert isinstance(edges, list)
#pylint: disable=too-few-public-methods
class XOR(Node):
    def __init__(self, nodea, nodeb, **kwargs):
        assert nodea.bitsz == nodeb.bitsz
        super().__init__("xor", nodea.bitsz, **kwargs)
        self.nodea = nodea
        self.nodeb = nodeb
        self.red, self.green, self.blue = parse_rgbstr("#90ee90") # LightGreen

    def eval(self):
        return self.nodea.eval() ^ self.nodeb.eval()

    def populate_nodes(self, nodes):
        super().populate_nodes(nodes)
        self.nodea.populate_nodes(nodes)
        self.nodeb.populate_nodes(nodes)

    def populate_edges(self, edges):
        assert isinstance(edges, list)
        edges.append((self.nodea.idx, self.idx, "a"))
        edges.append((self.nodeb.idx, self.idx, "b"))
        self.nodea.populate_edges(edges)
        self.nodeb.populate_edges(edges)
#pylint: enable=too-few-public-methods

#pylint: disable=too-few-public-methods
class Left(Node):
    def __init__(self, node, **kwargs):
        assert node.bitsz % 2 == 0
        super().__init__("left", node.bitsz // 2, **kwargs)
        self.node = node
        self.red, self.green, self.blue = parse_rgbstr("#800080") # Purple
        self.bitmask = 0
        for idx in range(0, node.bitsz):
            self.bitmask = self.bitmask << 1
            if idx < node.bitsz // 2:
                self.bitmask = self.bitmask | 0x01

    def eval(self):
        return (self.node.eval() & self.bitmask) >> (self.node.bitsz // 2)

    def populate_nodes(self, nodes):
        super().populate_nodes(nodes)
        self.node.populate_nodes(nodes)

    def populate_edges(self, edges):
        assert isinstance(edges, list)
        edges.append((self.node.idx, self.idx, "src"))
        self.node.populate_edges(edges)
#pylint: enable=too-few-public-methods

#pylint: disable=too-few-public-methods
class Right(Node):
    def __init__(self, node, **kwargs):
        assert node.bitsz % 2 == 0
        super().__init__("right", node.bitsz // 2, **kwargs)
        self.node = node
        self.red, self.green, self.blue = parse_rgbstr("#ee82ee") # Violet
        self.bitmask = 0
        for idx in range(0, node.bitsz):
            self.bitmask = self.bitmask << 1
            if idx >= node.bitsz // 2:
                self.bitmask = self.bitmask | 0x01

    def eval(self):
        return self.node.eval() & self.bitmask

    def populate_nodes(self, nodes):
        super().populate_nodes(nodes)
        self.node.populate_nodes(nodes)

    def populate_edges(self, edges):
        assert isinstance(edges, list)
        edges.append((self.node.idx, self.idx, "src"))
        self.node.populate_edges(edges)
#pylint: enable=too-few-public-methods

class Copy(Node):
    def __init__(self, node, name=None, color=None, size=None, **kwargs):
        super().__init__(name or ".", node.bitsz, size=0.2 if size is None else size, **kwargs)
        self.node = node
        self.red, self.green, self.blue = parse_rgbstr(color or "#d3d3d3") # LightGray

    def eval(self):
        return self.node.eval()

    def populate_nodes(self, nodes):
        super().populate_nodes(nodes)
        self.node.populate_nodes(nodes)

    def populate_edges(self, edges):
        assert isinstance(edges, list)
        edges.append((self.node.idx, self.idx, "src"))
        self.node.populate_edges(edges)

#pylint: disable=too-few-public-methods
class Concatenate(Node):
    def __init__(self, nodeleft, noderight, **kwargs):
        super().__init__("concatenate", nodeleft.bitsz + noderight.bitsz, **kwargs)
        self.nodeleft, self.noderight = nodeleft, noderight
        self.red, self.green, self.blue = parse_rgbstr("#d3d3d3") # LightGray

    def eval(self):
        return (self.nodeleft.eval() << self.noderight.bitsz) | self.noderight.eval()

    def populate_nodes(self, nodes):
        super().populate_nodes(nodes)
        self.nodeleft.populate_nodes(nodes)
        self.noderight.populate_nodes(nodes)

    def populate_edges(self, edges):
        assert isinstance(edges, list)
        edges.append((self.nodeleft.idx, self.idx, "left"))
        edges.append((self.noderight.idx, self.idx, "right"))
        self.nodeleft.populate_edges(edges)
        self.noderight.populate_edges(edges)
#pylint: enable=too-few-public-methods

#pylint: disable=too-few-public-methods
class Swap(Node):
    def __init__(self, nodeleft, noderight, **kwargs):
        super().__init__("swap", nodeleft.bitsz + noderight.bitsz, **kwargs)
        self.nodeleft, self.noderight = nodeleft, noderight
        self.red, self.green, self.blue = parse_rgbstr("#d3d3d3") # LightGray

    def eval(self):
        return (self.noderight.eval() << self.nodeleft.bitsz) | self.nodeleft.eval()

    def populate_nodes(self, nodes):
        super().populate_nodes(nodes)
        self.nodeleft.populate_nodes(nodes)
        self.noderight.populate_nodes(nodes)

    def populate_edges(self, edges):
        assert isinstance(edges, list)
        edges.append((self.nodeleft.idx, self.idx, "left"))
        edges.append((self.noderight.idx, self.idx, "right"))
        self.nodeleft.populate_edges(edges)
        self.noderight.populate_edges(edges)
#pylint: enable=too-few-public-methods

#pylint: disable=invalid-name,too-few-public-methods
class F(Node):
    """Binesh - 2023-08-25 have tested this against the rust version, and verified that this works."""
    def __init__(self, subkey, value, **kwargs):
        assert subkey.bitsz == 16
        assert value.bitsz == 32
        super().__init__("F", 32, **kwargs)
        self.subkey = subkey
        self.value = value
        self.red, self.green, self.blue = parse_rgbstr("#f08080") # LightCoral

    @staticmethod
    def gx(x, a, b):
        int_ = (a + b + x) & 0x00ff
        rot = ((int_ << 2) & 0x00fc) | ((int_ & 0x00c0) >> 6)
        return rot

    def g0(self, inp1, inp2):
        return self.gx(0, inp1, inp2)

    def g1(self, inp1, inp2):
        return self.gx(1, inp1, inp2)

    def fyoutube(self, a, b, c, d):
        """This is a translation of the function directly from ../src/feal.rs"""
        v1 = a ^ b
        v2 = c ^ d
        v3 = self.g1(v1, v2)
        v4 = self.g0(v2, v3)
        v5 = self.g0(a, v3)
        v6 = self.g1(d, v4)
        ap = v5
        bp = v3
        cp = v4
        dp = v6
        return ap, bp, cp, dp

    def eval(self):
        subkey = self.subkey.eval()
        b0 = ((subkey & 0xff00) >> 8) & 0x00ff
        b1 = subkey & 0x00ff

        value = self.value.eval()
        a0 = ((value & 0xff000000) >> 24) & 0x00ff
        a1 = ((value & 0x00ff0000) >> 16) & 0x00ff
        a2 = ((value & 0x0000ff00) >>  8) & 0x00ff
        a3 = value & 0x000000ff

        ap, bp, cp, dp = self.fyoutube(a0, b0 ^ a1, b1 ^ a2, a3)

        combined = (ap << 24) | (bp << 16) | (cp << 8) | dp

        return combined

    def populate_nodes(self, nodes):
        super().populate_nodes(nodes)
        self.subkey.populate_nodes(nodes)
        self.value.populate_nodes(nodes)

    def populate_edges(self, edges):
        assert isinstance(edges, list)
        edges.append((self.subkey.idx, self.idx, "subkey"))
        edges.append((self.value.idx, self.idx, "value"))
        self.subkey.populate_edges(edges)
        self.value.populate_edges(edges)
#pylint: enable=invalid-name,too-few-public-methods

#pylint: disable=invalid-name,too-many-locals
def encrypt():
    """Binesh - 2023-08-26 have tested this against the rust version, and verified that this works."""
    # This is basically just translated from feal4_raw from ../src/feal.rs
    plaintext = Input("plaintext", 64, xpos=0, ypos=0)
    keys = [Input(f"key{idx:d}", 16, xpos=3, ypos=5+(idx*5)+0) for idx in range(0, 8)]
    key0 = keys[0]
    key1 = keys[1]
    key2 = keys[2]
    key3 = keys[3]
    key4 = keys[4]
    key5 = keys[5]
    key6 = keys[6]
    key7 = keys[7]
    key8_11 = Input("key8_11", 64, xpos=3, ypos=1)
    key12_15 = Input("key12_15", 64, xpos=3, ypos=47)

    v1 = XOR(plaintext, key8_11, xpos=0, ypos=1)
    copy = Copy(v1, xpos=0, ypos=2)

    left = Left(copy, xpos=-1, ypos=3)
    right = Right(copy, xpos=1, ypos=3)

    left = Copy(left, xpos=-1, ypos=4)
    right = XOR(left, right, xpos=1, ypos=4)

    starty = 5
    for idx in range(0, 8):
        right = Copy(right, xpos=1, ypos=starty+(idx*5)+1)
        intermediate = F(Copy(keys[idx], xpos=0, ypos=starty+(idx*5)+0), right, xpos=0, ypos=starty+(idx*5)+1)
        concatenated = Swap(Copy(XOR(left, intermediate, xpos=-1, ypos=starty+(idx*5)+1), xpos=-1, ypos=starty+(idx*5)+2), Copy(right, xpos=1, ypos=starty+(idx*5)+2), xpos=0, ypos=starty+(idx*5)+3)
        left, right = Left(concatenated, xpos=-1, ypos=starty+(idx*5)+4), Right(concatenated, xpos=1, ypos=starty+(idx*5)+4)

    right = Copy(right, xpos=1, ypos=45)
    left = XOR(left, right, xpos=-1, ypos=45)
    combined = Swap(left, right, xpos=0, ypos=46)
    ciphertext = Copy(XOR(combined, key12_15, xpos=0, ypos=47), name="ciphertext", color="#00008b", xpos=0, ypos=48, size=0.4) # DarkBlue

    return plaintext, key0, key1, key2, key3, key4, key5, key6, key7, key8_11, key12_15, ciphertext
#pylint: enable=invalid-name,too-many-locals

def main():
    inp = Input("H", 16)
    left = Left(inp)
    right = Right(inp)
    concat = Concatenate(left, right)

    for val in range(0, 65536):
        inp.set(val)
        assert val == concat.eval()
        print(f"{val:04x}: {concat.eval():04x}")

if __name__ == "__main__":
    main()
