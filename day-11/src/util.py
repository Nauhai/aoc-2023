
def parse_grid(lines):
    return list(map(lambda l: list(l.strip()), lines))


def find_galaxies(grid):
    galaxies = []
    for i in range(len(grid)):
        for j in range(len(grid[i])):
            if grid[i][j] == "#":
                galaxies.append((i, j))
    return galaxies
