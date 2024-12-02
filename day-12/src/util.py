from itertools import combinations


def parse_records(lines):
    return list(map(parse_line, lines))


def parse_line(line):
    springs, groups = line.strip().split()
    groups = list(map(int, groups.split(",")))
    return springs, groups


def get_possible_arrangements(record):
    springs, groups = record
    blocks = len(groups)
    spaces = len(springs) - sum(groups) + blocks
    combis = combinations(range(spaces-blocks+1), blocks)
    arr = map(lambda c: combi_to_springs(c, groups, len(springs)), combis)
    arr = list(filter(lambda c: matches(c, list(springs)), arr))
    return arr


def combi_to_springs(combi, groups, length):
    springs = []
    for i in range(len(combi)):
        for j in range(combi[i]-(0 if i == 0 else combi[i-1])):
            springs.append(".")
        for j in range(groups[i]):
            springs.append("#")
    for i in range(length-len(springs)):
        springs.append(".")
    return springs


def matches(springs, blueprint):
    for c1, c2 in zip(springs, blueprint):
        if c1 != c2 and c2 != "?":
            return False
    return True
