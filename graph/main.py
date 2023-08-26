#! /usr/bin/env python3
# vim: expandtab shiftwidth=4 tabstop=4

"""Build a _graph_ of the FEAL-8 network so that we can make an actual plot of the algorithm."""

import re

def parse_rgbstr(rgbstr):
    mtch = re.match(r'^#([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})$', rgbstr)
    assert mtch is not None, rgbstr
    red, green, blue = int(mtch.group(1), 16), int(mtch.group(2), 16), int(mtch.group(3), 16)
    return red, green, blue

#pylint: disable=too-few-public-methods,too-many-instance-attributes
class Node:
    counter = 0
    def __init__(self, name, bitsz):
        self.name = name
        self.bitsz = bitsz
        self.inputs = []
        self.idx = Node.counter
        self.value = None
        self.red, self.green, self.blue = parse_rgbstr("#ffffff")
        Node.counter += 1
#pylint: enable=too-few-public-methods

class Input(Node):
    def __init__(self, name, bitsz):
        super().__init__(name, bitsz)
        self.red, self.green, self.blue = parse_rgbstr("#87ceeb") # SkyBlue
        if name.startswith("key"):
            self.red, self.green, self.blue = parse_rgbstr("#ff8c00") # DarkOrange

    def set(self, value):
        self.value = value

    def eval(self):
        return self.value

    def populate_nodes(self, depth, xpos, nodes):
        assert isinstance(depth, int)
        assert isinstance(xpos, int)
        assert isinstance(nodes, dict)
        nodes[self.idx] = {
            "label": self.name,
            "r": self.red, "g": self.green, "b": self.blue,
            "x": float(xpos),
            "y": float(depth),
        }

    def populate_edges(self, edges):
        assert isinstance(edges, list)
#pylint: disable=too-few-public-methods
class XOR(Node):
    def __init__(self, nodea, nodeb):
        assert nodea.bitsz == nodeb.bitsz
        super().__init__("xor", nodea.bitsz)
        self.nodea = nodea
        self.nodeb = nodeb
        self.red, self.green, self.blue = parse_rgbstr("#90ee90") # LightGreen

    def eval(self):
        return self.nodea.eval() ^ self.nodeb.eval()

    def populate_nodes(self, depth, xpos, nodes):
        assert isinstance(nodes, dict)
        nodes[self.idx] = {
            "label": self.name,
            "r": self.red, "g": self.green, "b": self.blue,
            "x": float(xpos),
            "y": float(depth),
        }
        self.nodea.populate_nodes(1+depth, xpos-1, nodes)
        self.nodeb.populate_nodes(1+depth, xpos+1, nodes)

    def populate_edges(self, edges):
        assert isinstance(edges, list)
        edges.append((self.nodea.idx, self.idx, None))
        edges.append((self.nodeb.idx, self.idx, None))
        self.nodea.populate_edges(edges)
        self.nodeb.populate_edges(edges)
#pylint: enable=too-few-public-methods

#pylint: disable=too-few-public-methods
class Left(Node):
    def __init__(self, node):
        assert node.bitsz % 2 == 0
        super().__init__("left", node.bitsz // 2)
        self.node = node
        self.red, self.green, self.blue = parse_rgbstr("#800080") # Purple
        self.bitmask = 0
        for idx in range(0, node.bitsz):
            self.bitmask = self.bitmask << 1
            if idx < node.bitsz // 2:
                self.bitmask = self.bitmask | 0x01

    def eval(self):
        return (self.node.eval() & self.bitmask) >> (self.node.bitsz // 2)

    def populate_nodes(self, depth, xpos, nodes):
        assert isinstance(nodes, dict)
        nodes[self.idx] = {
            "label": self.name,
            "r": self.red, "g": self.green, "b": self.blue,
            "x": float(xpos),
            "y": float(depth),
        }
        self.node.populate_nodes(1+depth, xpos-1, nodes)

    def populate_edges(self, edges):
        assert isinstance(edges, list)
        edges.append((self.node.idx, self.idx, None))
        self.node.populate_edges(edges)
#pylint: enable=too-few-public-methods

#pylint: disable=too-few-public-methods
class Right(Node):
    def __init__(self, node):
        assert node.bitsz % 2 == 0
        super().__init__("right", node.bitsz // 2)
        self.node = node
        self.red, self.green, self.blue = parse_rgbstr("#ee82ee") # Violet
        self.bitmask = 0
        for idx in range(0, node.bitsz):
            self.bitmask = self.bitmask << 1
            if idx >= node.bitsz // 2:
                self.bitmask = self.bitmask | 0x01

    def eval(self):
        return self.node.eval() & self.bitmask

    def populate_nodes(self, depth, xpos, nodes):
        assert isinstance(nodes, dict)
        nodes[self.idx] = {
            "label": self.name,
            "r": self.red, "g": self.green, "b": self.blue,
            "x": float(xpos),
            "y": float(depth),
        }
        self.node.populate_nodes(1+depth, xpos+1, nodes)

    def populate_edges(self, edges):
        assert isinstance(edges, list)
        edges.append((self.node.idx, self.idx, None))
        self.node.populate_edges(edges)
#pylint: enable=too-few-public-methods

#pylint: disable=too-few-public-methods
class Concatenate(Node):
    def __init__(self, nodeleft, noderight):
        super().__init__("concatenate", nodeleft.bitsz + noderight.bitsz)
        self.nodeleft, self.noderight = nodeleft, noderight
        self.red, self.green, self.blue = parse_rgbstr("#d3d3d3") # LightGray

    def eval(self):
        return (self.nodeleft.eval() << self.noderight.bitsz) | self.noderight.eval()

    def populate_nodes(self, depth, xpos, nodes):
        assert isinstance(nodes, dict)
        nodes[self.idx] = {
            "label": self.name,
            "r": self.red, "g": self.green, "b": self.blue,
            "x": float(xpos),
            "y": float(depth),
        }
        self.nodeleft.populate_nodes(1+depth, xpos-1, nodes)
        self.noderight.populate_nodes(1+depth, xpos+1, nodes)

    def populate_edges(self, edges):
        assert isinstance(edges, list)
        edges.append((self.nodeleft.idx, self.idx, None))
        edges.append((self.noderight.idx, self.idx, None))
        self.nodeleft.populate_edges(edges)
        self.noderight.populate_edges(edges)
#pylint: enable=too-few-public-methods

#pylint: disable=invalid-name,too-few-public-methods
class F(Node):
    """Binesh - 2023-08-25 have tested this against the rust version, and verified that this works."""
    def __init__(self, subkey, value):
        assert subkey.bitsz == 16
        assert value.bitsz == 32
        super().__init__("F", 32)
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

    def populate_nodes(self, depth, xpos, nodes):
        assert isinstance(nodes, dict)
        nodes[self.idx] = {
            "label": self.name,
            "r": self.red, "g": self.green, "b": self.blue,
            "x": float(xpos),
            "y": float(depth),
        }
        self.subkey.populate_nodes(1+depth, xpos+1, nodes)
        self.value.populate_nodes(1+depth, xpos-1, nodes)

    def populate_edges(self, edges):
        assert isinstance(edges, list)
        edges.append((self.subkey.idx, self.idx, None))
        edges.append((self.value.idx, self.idx, None))
        self.subkey.populate_edges(edges)
        self.value.populate_edges(edges)
#pylint: enable=invalid-name,too-few-public-methods

#pylint: disable=invalid-name,too-many-locals
def encrypt():
    """Binesh - 2023-08-26 have tested this against the rust version, and verified that this works."""
    # This is basically just translated from feal4_raw from ../src/feal.rs
    plaintext = Input("plaintext", 64)
    keys = [Input(f"key{idx:d}", 16) for idx in range(0, 8)]
    key0 = keys[0]
    key1 = keys[1]
    key2 = keys[2]
    key3 = keys[3]
    key4 = keys[4]
    key5 = keys[5]
    key6 = keys[6]
    key7 = keys[7]
    key8_11 = Input("key8_11", 64)
    key12_15 = Input("key12_15", 64)

    v1 = XOR(plaintext, key8_11)
    left = Left(v1)
    right = Right(v1)

    right = XOR(left, right)

    for idx in range(0, 8):
        intermediate = F(keys[idx], right)
        left, right = right, XOR(left, intermediate)

    left = XOR(left, right)
    combined = Concatenate(right, left) # right, left is deliberate, the algorithm _calls_ for this swap.
    ciphertext = XOR(combined, key12_15)

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
