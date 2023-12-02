use std::collections::HashMap;
use std::convert::From;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cube {
    Red,
    Green,
    Blue,
}

use Cube::*;

impl<'a> From<&'a str> for Cube {
    fn from(value: &'a str) -> Cube {
        match value {
            "red" => Red,
            "green" => Green,
            "blue" => Blue,
            _ => panic!(),
        }
    }
}

pub type Set = HashMap<Cube, u32>;

fn set_from_str(value: &str) -> Set {
    let mut set = Set::new();

    for elem in value.trim().split(",") {
        let mut split = elem.trim().split(" ");
        let (num, cube) = (split.next().unwrap(), split.next().unwrap());
        let num = num.parse().unwrap();
        let cube = Cube::from(cube);

        set.insert(cube, num);
    }

    set
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

impl<'a> From<&'a str> for Game {
    fn from(value: &'a str) -> Game {
        let mut split = value.split(':');
        let (id, sets) = (split.next().unwrap(), split.next().unwrap());
        let id = id.strip_prefix("Game ").unwrap().parse().unwrap();
        let sets = sets.split(';').map(|s| set_from_str(s)).collect();

        Game { id, sets }
    }
}

pub fn is_set_possible(set: &Set, config: &Set) -> bool {
    set.iter().all(|(c, n)| config.get(c).unwrap() >= n)
}

pub fn is_game_possible(game: &Game, config: &Set) -> bool {
    game.sets.iter().all(|s| is_set_possible(s, config))
}

pub fn input_to_games(input: &str) -> Vec<Game> {
    input.lines().map(Game::from).collect()
}

#[test]
fn test_parse_game() {
    let game = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    let should_be = Game {
        id: 1,
        sets: vec![
            Set::from([(Blue, 3), (Red, 4)]),
            Set::from([(Red, 1), (Green, 2), (Blue, 6)]),
            Set::from([(Green, 2)]),
        ],
    };

    assert_eq!(game, should_be);
}

#[test]
fn test_game_possible() {
    let game1 = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    let game3 =
        Game::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");

    let config = Set::from([(Red, 12), (Green, 13), (Blue, 14)]);

    assert!(is_game_possible(&game1, &config));
    assert!(!is_game_possible(&game3, &config));
}
