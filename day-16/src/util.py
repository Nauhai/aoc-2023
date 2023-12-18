from collections import namedtuple, defaultdict


def parse_grid(lines):
    return list(map(lambda l: l.strip(), lines))


Beam = namedtuple("Beam", ["pos", "dir"])


def count_energized_cells(grid, starting_beam):
    beams = [starting_beam]
    energized = defaultdict(set)

    while beams:
        beam = beams.pop(0)
        i, j = beam.pos

        if 0 <= i < len(grid) and 0 <= j < len(grid[0]) and beam.dir not in energized[beam.pos]:
            energized[beam.pos].add(beam.dir)
            new_beams = move(grid, beam)
            beams.extend(new_beams)
    
    return len(energized)


def move(grid, beam):
    i, j = beam.pos
    di, dj = beam.dir

    match grid[i][j]:
        case "|" if di == 0:
            return [Beam((i-1, j), (-1, 0)), Beam((i+1, j), (1, 0))]
        case "-" if dj == 0:
            return [Beam((i, j-1), (0, -1)), Beam((i, j+1), (0, 1))]
        case "/":
            if di == -1:
                return [Beam((i, j+1), (0, 1))]
            elif di == 1:
                return [Beam((i, j-1), (0, -1))]
            elif dj == -1:
                return [Beam((i+1, j), (1, 0))]
            else:
                return [Beam((i-1, j), (-1, 0))]
        case "\\":
            if di == -1:
                return [Beam((i, j-1), (0, -1))]
            elif di == 1:
                return [Beam((i, j+1), (0, 1))]
            elif dj == -1:
                return [Beam((i-1, j), (-1, 0))]
            else:
                return [Beam((i+1, j), (1, 0))]
        case _:
            return [Beam((i+di, j+dj), (di, dj))]
