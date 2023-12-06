import util
import sys


def parse_number(line):
    return int(line.replace(" ", "").split(":")[1])


def parse_race(lines):
    time, record = map(parse_number, lines)
    return (time, record)


def process(race):
    return util.number_of_ways_to_win(race)


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        race = parse_race(file.readlines())
        print("Part 2:", process(race))
