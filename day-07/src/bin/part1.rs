use day_07::*;

fn process(mut hands: Vec<Hand>) -> u32 {
    hands.sort_by(Hand::cmp);
    hands
        .iter()
        .map(|h| h.bid)
        .enumerate()
        .map(|(i, b)| (i as u32 + 1) * b)
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let hands = parse_hands(input);
    println!("Part 1: {}", process(hands));
}
