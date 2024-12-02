import util
import sys
import math


def parse_input(lines):
    times, dists = map(parse_line, lines)
    return list(zip(times, dists))


def parse_line(line):
    return list(map(int, line.split()[1:]))


def process(races):
    return math.prod(map(util.number_of_ways_to_win, races))


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        races = parse_input(file.readlines())
        print("Part 1:", process(races))
