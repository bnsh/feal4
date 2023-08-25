#! /usr/bin/env python3
# vim: expandtab shiftwidth=4 tabstop=4

"""Build a _graph_ of the FEAL-8 network so that we can make an actual plot of the algorithm."""

# import matplotlib.pyplot as plt
# import networkx as nx

#pylint: disable=too-few-public-methods
class Node:
    counter = 0
    def __init__(self, name, bitsz):
        self.name = name
        self.bitsz = bitsz
        self.inputs = []
        self.idx = Node.counter
        self.value = None
        Node.counter += 1
#pylint: enable=too-few-public-methods

class Input(Node):
#     def __init__(self, name, bitsz):
#         super().__init__(name, bitsz)

    def set(self, value):
        self.value = value

    def eval(self):
        return self.value

#pylint: disable=too-few-public-methods
class XOR(Node):
    def __init__(self, nodea, nodeb):
        assert nodea.bitsz == nodeb.bitsz
        super().__init__("xor", nodea.bitsz)
        self.nodea = nodea
        self.nodeb = nodeb

    def eval(self):
        return self.nodea.eval() ^ self.nodeb.eval()
#pylint: enable=too-few-public-methods

#pylint: disable=too-few-public-methods
class Left(Node):
    def __init__(self, node):
        assert node.bitsz % 2 == 0
        super().__init__("left", node.bitsz // 2)
        self.node = node
        self.bitmask = 0
        for idx in range(0, node.bitsz):
            self.bitmask = self.bitmask << 1
            if idx < node.bitsz // 2:
                self.bitmask = self.bitmask | 0x01

    def eval(self):
        return (self.node.eval() & self.bitmask) >> (self.node.bitsz // 2)
#pylint: enable=too-few-public-methods

#pylint: disable=too-few-public-methods
class Right(Node):
    def __init__(self, node):
        assert node.bitsz % 2 == 0
        super().__init__("right", node.bitsz // 2)
        self.node = node
        self.bitmask = 0
        for idx in range(0, node.bitsz):
            self.bitmask = self.bitmask << 1
            if idx >= node.bitsz // 2:
                self.bitmask = self.bitmask | 0x01

    def eval(self):
        return self.node.eval() & self.bitmask
#pylint: enable=too-few-public-methods

#pylint: disable=too-few-public-methods
class Concatenate(Node):
    def __init__(self, nodeleft, noderight):
        super().__init__("concatenate", nodeleft.bitsz + noderight.bitsz)
        self.nodeleft, self.noderight = nodeleft, noderight

    def eval(self):
        return (self.nodeleft.eval() << self.noderight.bitsz) | self.noderight.eval()
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
#pylint: enable=invalid-name,too-few-public-methods

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
