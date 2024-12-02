import sys
import util


def process(records):
    records = map(unfold, records)
    return sum(map(len, map(util.get_possible_arrangements, records)))


def unfold(record):
    springs, groups = record
    return springs*5, groups*5


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        records = util.parse_records(file.readlines())
        print("Part 2:", process(records))
