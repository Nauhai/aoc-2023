from collections import namedtuple
from itertools import cycle


Node = namedtuple("Node", ["val", "left", "right"])


def parse_line(line):
    val, lr = line.strip().split(" = ")
    left, right = lr.strip("()").split(", ")
    return val, Node(val, left, right)


def parse_map(lines):
    instr = lines[0].strip()
    mapp = dict(map(parse_line, lines[2:]))
    return instr, mapp


def follow_until(pred, node, instr, mapp):
    instr = cycle(instr)
    steps = 0

    while not pred(node):
        direction = next(instr)

        if direction == "L":
            node = mapp[node.left]
        else:
            node = mapp[node.right]
        
        steps += 1
    
    return steps
