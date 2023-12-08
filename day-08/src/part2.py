import sys
import util
import math


def node_next(node, direction):
    return node.left if direction == "L" else node.right


def process(instr, mapp):
    currents = list(filter(lambda n: n.val[2] == "A", mapp.values()))
    steps = map(lambda n: util.follow_until(lambda n: n.val[2] == "Z", n, instr, mapp), currents)    
    return math.lcm(*steps)


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        lines = file.readlines()
        instr, mapp = util.parse_map(lines)
        print("Part 2:", process(instr, mapp))
