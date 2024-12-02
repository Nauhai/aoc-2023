import util
import sys
import copy


def process(grid):
    width = len(grid[0])
    height = len(grid)

    grids = []
    for i in range(1_000_000_000):
        grid = cycle(grid, width, height)

        if grid in grids:
            index = grids.index(grid)
            period = i-index
            rest = (1_000_000_000-i-1)%period
            for j in range(rest):
                grid = cycle(grid, width, height)
            break
        else:
            grids.append(copy.deepcopy(grid))
    
    return util.count_load(grid)


def cycle(grid, width, height):
    grid = util.roll_north(grid, width, height)
    grid = util.roll_west(grid, width, height)
    grid = util.roll_south(grid, width, height)
    grid = util.roll_east(grid, width, height)
    return grid


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        grid = util.parse_grid(file.readlines())
        print("Part 2:", process(grid))
