import util


def process(input):
    return sum(map(lambda l: util.get_calibration_value(l, r"\d"), input))


if __name__ == "__main__":
    with open(sys.argv[1], "r") as input:
        print("Part 1:", process(input))
