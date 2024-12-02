
def parse_grid(lines):
    return list(map(lambda l: list(l.strip()), lines))


def roll_north(grid, width, height):
    for i in range(height):
        for j in range(width):
            if grid[i][j] == 'O':
                grid[i][j] = '.'
                ni = i
                while ni > 0 and grid[ni-1][j] == '.':
                    ni -= 1
                grid[ni][j] = 'O'
    return grid


def roll_east(grid, width, height):
    for j in range(width)[::-1]:
        for i in range(height):
            if grid[i][j] == 'O':
                grid[i][j] = '.'
                nj = j
                while nj < width-1 and grid[i][nj+1] == '.':
                    nj += 1
                grid[i][nj] = 'O'
    return grid


def roll_south(grid, width, height):
    for i in range(height)[::-1]:
        for j in range(width):
            if grid[i][j] == 'O':
                grid[i][j] = '.'
                ni = i
                while ni < height-1 and grid[ni+1][j] == '.':
                    ni += 1
                grid[ni][j] = 'O'
    return grid


def roll_west(grid, width, height):
    for j in range(width):
        for i in range(height):
            if grid[i][j] == 'O':
                grid[i][j] = '.'
                nj = j
                while nj > 0 and grid[i][nj-1] == '.':
                    nj -= 1
                grid[i][nj] = 'O'
    return grid


def count_load(grid):
    load = 0
    for i in range(len(grid)):
        rocks = 0
        for j in range(len(grid[i])):
            if grid[i][j] == 'O':
                rocks += 1
        load += (len(grid)-i) * rocks
    return load
