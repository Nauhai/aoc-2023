
def parse_sequence(line):
    return list(map(int, line.strip().split()))


def parse_input(lines):
    return list(map(parse_sequence, lines))


def seq_is_zero(seq):
    for e in seq:
        if e != 0:
            return False
    return True


def get_diff_seq(seq):
    res = []
    for i in range(len(seq)-1):
        res.append(seq[i+1] - seq[i])
    return res


def get_all_diff(seq):
    diff_sequences = [seq]
    i = 0
    while not seq_is_zero(diff_sequences[i]):
        diff_sequences.append(get_diff_seq(diff_sequences[i]))
        i += 1
    return diff_sequences
