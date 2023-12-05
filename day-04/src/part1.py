import util
import sys


def compute_points(card):
    l = len(card.winning_numbers())
    if l == 0:
        return 0
    return 2**(l-1)


def process(cards):
    return sum(map(compute_points, cards))


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        cards = map(util.str_to_card, file.readlines())
        print("Part 1:", process(cards))
