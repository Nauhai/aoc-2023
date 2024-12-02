
def parse_patterns(input):
    return list(map(lambda p: p.split("\n"), input.split("\n\n")))
