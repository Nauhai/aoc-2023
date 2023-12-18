import util
import sys


def process(grid):
    starting_beam = util.Beam((0, 0), (0, 1))
    return util.count_energized_cells(grid, starting_beam)


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        grid = util.parse_grid(file.readlines())
        print("Part 1:", process(grid))
