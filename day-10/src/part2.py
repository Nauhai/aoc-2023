import sys
import util


def process(grid, pos):
    path = util.Path(grid, pos)
    path.complete()
    # TODO: flood fill
                

if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        grid, pos = util.parse_grid(file.readlines())
        print("Part 2:", process(grid, pos))
