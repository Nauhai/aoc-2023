import util
import sys
from collections import defaultdict


def process(cards):
    cards_count = defaultdict(lambda: 1)
    cards_count[0]
    
    for i, card in enumerate(cards):
        l = len(card.winning_numbers())
        
        for j in range(i+1, i+l+1):
            cards_count[j] += cards_count[i]
    
    return sum(cards_count.values())


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        cards = list(map(util.str_to_card, file.readlines()))
        print("Part 2:", process(cards))
