import util
import sys


def process(steps):
    return sum(map(util.hash, steps))


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        steps = util.parse_steps(file.read())
        print("Part 1:", process(steps))
