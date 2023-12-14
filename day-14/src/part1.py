import util
import sys


def process(grid):
    grid = util.roll_north(grid, len(grid[0]), len(grid))
    return util.count_load(grid)


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        grid = util.parse_grid(file.readlines())
        print("Part 1:", process(grid))
