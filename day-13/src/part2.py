import util
import sys


def process(patterns):
    summary = 0
    for pattern in patterns:
        if (line := find_vertical_reflection_line(pattern)) is not None:
            summary += line+1
        else:
            summary += 100*(find_horizontal_reflection_line(pattern)+1)
    return summary


def find_vertical_reflection_line(pattern):
    for j in range(len(pattern[0])-1):
        if vertical_diff(pattern, j) == 1:
            return j
    

def find_horizontal_reflection_line(pattern):
    for i in range(len(pattern)-1):
        if horizontal_diff(pattern, i) == 1:
            return i


def vertical_diff(pattern, j):
    diff = 0
    d = 0
    while 0 <= j-d and j+1+d < len(pattern[0]):
        for i in range(len(pattern)):
            if pattern[i][j-d] != pattern[i][j+1+d]:
                diff += 1
        d += 1
    return diff


def horizontal_diff(pattern, i):
    diff = 0
    d = 0
    while 0 <= i-d and i+1+d < len(pattern):
        for j in range(len(pattern[i])):
            if pattern[i-d][j] != pattern[i+1+d][j]:
                diff += 1
        d += 1
    return diff


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        patterns = util.parse_patterns(file.read())
        print("Part 2:", process(patterns))
