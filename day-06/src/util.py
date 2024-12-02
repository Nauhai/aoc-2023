
def number_of_ways_to_win(race):
    time, record = race
    dists = get_all_distances(time)
    winning = list(filter(lambda x: x > record, dists))
    return len(winning)


def get_all_distances(time):
    return list(map(lambda x: x*(time-x), list(range(0, time+1))))
