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
    def __init__(self, subkey, value):
        assert subkey.bitsz == 16
        assert value.bitsz == 32
        super().__init__("F", 32)
        self.subkey = subkey
        self.value = value

    def eval(self):
        # TODO: Here we just want to implement F directly. the purpose here isn't to fully create the whole algorithm this way,
        #       Simply to eventually output a graph of the computation (akin to Fig 13.3 in "Applied Cryptography" by Bruce Schneier.
        raise RuntimeError("Unimplemented")
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
