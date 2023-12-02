use day_02::*;

fn process(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|g| get_min_config(g))
        .map(|c| get_set_power(&c))
        .sum()
}

fn get_min_config(game: &Game) -> Set {
    let mut min_config = Set::new();

    for set in &game.sets {
        for (&cube, &num) in set {
            min_config
                .entry(cube)
                .and_modify(|n| *n = num.max(*n))
                .or_insert(num);
        }
    }

    min_config
}

fn get_set_power(set: &Set) -> u32 {
    let mut acc = 1;
    for (_, num) in set {
        acc *= num;
    }
    acc
}

fn main() {
    let input = include_str!("../input.txt");
    let games = input_to_games(input);
    println!("Part 2: {}", process(&games));
}
