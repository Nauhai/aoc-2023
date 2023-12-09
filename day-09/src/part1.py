import sys
import util


def extrapolate(seq):
    diff_sequences = util.get_all_diff(seq)
    diff_sequences[-1].append(0)
    
    for i in range(len(diff_sequences)-1, 0, -1):
        diff_sequences[i-1].append(diff_sequences[i][-1] + diff_sequences[i-1][-1])
    
    return diff_sequences[0][-1]


def process(sequences):
    return sum(map(extrapolate, sequences))


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        sequences = util.parse_input(file.readlines())
        print("Part 1:", process(sequences))
