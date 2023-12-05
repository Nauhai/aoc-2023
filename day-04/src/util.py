
class Card:
    def __init__(self, winning, numbers):
        self.winning = winning
        self.numbers = numbers

    def winning_numbers(self):
        return list(filter(lambda n: n in self.winning, self.numbers))


def str_to_card(line):
    win, num = line.strip().split(" | ")
    win = list(map(int, win.split()[2:]))
    num = list(map(int, num.split()))
    return Card(win, num)
