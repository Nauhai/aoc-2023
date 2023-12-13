import sys
import util
from itertools import combinations


def process(grid):
    grid = expand_grid(grid)
    galaxies = util.find_galaxies(grid)
    pairs = combinations(galaxies, 2)
    distances = map(lambda p: distance(*p), pairs)
    return sum(distances)


def expand_grid(grid):
    expanded = []

    for line in grid:
        if "#" not in line:
            expanded.append(line)
        expanded.append(line)
    
    empty_cols = list(range(len(grid[0])))
    for i in range(len(grid)):
        for j in empty_cols:
            if grid[i][j] == "#":
                empty_cols.remove(j)
    
    for i in range(len(grid)):
        for j in empty_cols[::-1]:
            grid[i].insert(j, ".")
    
    return expanded


def distance(p1, p2):
    i1, j1 = p1
    i2, j2 = p2

    return abs(i1-i2) + abs(j1-j2)


if __name__ == "__main__":
    with open(sys.argv[1]) as file:
        grid = util.parse_grid(file.readlines())
        print("Part 1:", process(grid))
