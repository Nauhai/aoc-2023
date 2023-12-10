
def parse_grid(lines):
    grid = list(map(lambda l: l.strip(), lines))

    pos = (0, 0)
    for i in range(len(grid)):
        if "S" in grid[i]:
            pos = (i, grid[i].index("S"))
    
    return grid, pos


def find_next(grid, prev, node):
    (ci, cj) = node
    (pi, pj) = prev

    match grid[ci][cj]:
        case "|" if pj == cj:
            return (ci+1, cj) if pi < ci else (ci-1, cj)
        case "-" if pi == ci:
            return (ci, cj+1) if pj < cj else (ci, cj-1)
        case "L" if pi <= ci and pj >= cj:
            return (ci, cj+1) if pi < ci else (ci-1, cj)
        case "J" if pi <= ci and pj <= cj:
            return (ci, cj-1) if pi < ci else (ci-1, cj)
        case "7" if pi >= ci and pj <= cj:
            return (ci+1, cj) if pj < cj else (ci, cj-1)
        case "F" if pi >= ci and pj >= cj:
            return (ci+1, cj) if pj > cj else (ci, cj+1)
        case other:
            return None


class Path:
    def __init__(self, grid, start):
        self.grid = grid
        self.nodes = [start]
        self.current = 1
        self.finished = False

        pi, pj = start
        for i, j in [(pi-1, pj), (pi+1, pj), (pi, pj-1), (pi, pj+1)]:   
            if find_next(self.grid, start, (i, j)):
                self.nodes.append((i, j))
                break
        
        if len(self.nodes) == 1:
            self.finished = True
    
    def next(self):
        if not self.finished:
            node = find_next(self.grid, self.nodes[self.current-1], self.nodes[self.current])
            if node in self.nodes:
                self.finished = True
            else:
                self.nodes.append(node)
                self.current += 1
    
    def complete(self):
        while not self.finished:
            self.next()
