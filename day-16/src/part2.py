import util
import sys


def process(grid):
    width = len(grid[0])
    height = len(grid)

    starting_beams = []
    
    for j in range(width):
        starting_beams.append(util.Beam((0, j), (1, 0)))
        starting_beams.append(util.Beam((height-1, j), (-1, 0)))
    
    for i in range(height):
        starting_beams.append(util.Beam((i, 0), (0, 1)))
        starting_beams.append(util.Beam((width-1, 0), (0, -1)))
    
    return max(map(lambda b: util.count_energized_cells(grid, b), starting_beams))


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        grid = util.parse_grid(file.readlines())
        print("Part 2:", process(grid))
