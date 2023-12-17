
def parse_steps(input):
    return input.strip().split(",")


def hash(string):
    val = 0
    for c in string:
        val += ord(c)
        val *= 17
        val %= 256
    return val
