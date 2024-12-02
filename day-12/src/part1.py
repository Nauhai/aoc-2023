import sys
import util


def process(records):
    return sum(map(len, map(util.get_possible_arrangements, records)))


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        records = util.parse_records(file.readlines())
        print("Part 1:", process(records))
