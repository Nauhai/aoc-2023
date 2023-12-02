use std::collections::HashMap;

use day_02::Cube::*;
use day_02::*;

fn process(games: &[Game], config: &Set) -> u32 {
    games
        .iter()
        .filter(|g| is_game_possible(g, config))
        .map(|g| g.id)
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let games = input_to_games(input);
    let config = HashMap::from([(Red, 12), (Green, 13), (Blue, 14)]);
    println!("Part 1: {}", process(&games, &config));
}
