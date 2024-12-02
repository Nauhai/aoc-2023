import util


def process(input):
    pattern = r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))"
    return sum(map(lambda l: util.get_calibration_value(l, pattern), input))


if __name__ == "__main__":
    with open(sys.argv[1], "r") as input:
        print("Part 2:", process(input))
