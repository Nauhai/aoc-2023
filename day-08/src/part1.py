import sys
import util


def process(instr, mapp):
    current = mapp["AAA"]
    return util.follow_until(lambda x: x.val == "ZZZ", current, instr, mapp)


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        lines = file.readlines()
        instr, mapp = util.parse_map(lines)
        print("Part 1:", process(instr, mapp))
