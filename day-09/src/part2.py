import sys
import util


def extrapolate_back(seq):
    diff_sequences = util.get_all_diff(seq)
    diff_sequences[-1].insert(0, 0)
    
    for i in range(len(diff_sequences)-1, 0, -1):
        diff_sequences[i-1].insert(0, (diff_sequences[i-1][0] - diff_sequences[i][0]))
    
    return diff_sequences[0][0]


def process(sequences):
    return sum(map(extrapolate_back, sequences))


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        sequences = util.parse_input(file.readlines())
        print("Part 2:", process(sequences))
