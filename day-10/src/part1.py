import sys
import util


def process(grid, pos):
    path = util.Path(grid, pos)
    path.complete()
    return len(path.nodes)//2


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        grid, pos = util.parse_grid(file.readlines())
        print("Part 1:", process(grid, pos))
