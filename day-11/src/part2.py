import sys
import util
from itertools import combinations


def process(grid):
    empty_lines = list(filter(lambda i: "#" not in grid[i], range(len(grid))))

    empty_cols = list(range(len(grid[0])))
    for i in range(len(grid)):
        for j in empty_cols:
            if grid[i][j] == "#":
                empty_cols.remove(j)
    
    print(empty_lines, empty_cols)
    
    galaxies = util.find_galaxies(grid)
    pairs = combinations(galaxies, 2)
    distances = map(lambda p: distance(*p, empty_lines, empty_cols, 1000000), pairs)
    return sum(distances)


def distance(p1, p2, empty_lines, empty_cols, factor):
    i1, j1 = p1
    i2, j2 = p2

    di = i2-i1
    dj = j2-j1

    sign_di = sign(di)
    sign_dj = sign(dj)

    dist = 0
    if di:
        for i in range(i1+sign_di, i1+di+sign_di, sign_di):
            dist += factor if i in empty_lines else 1
    
    if dj:
        for j in range(j1+sign_dj, j1+dj+sign_dj, sign_dj):
            dist += factor if j in empty_cols else 1

    return dist


def sign(x):
    return 0 if not x else x//abs(x)


if __name__ == "__main__":
    with open(sys.argv[1]) as file:
        grid = util.parse_grid(file.readlines())
        print("Part 2:", process(grid))
