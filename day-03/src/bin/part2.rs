use std::collections::HashMap;

use day_03::*;

fn get_adjacent_gears(number: &Number, schematic: &Schematic) -> Vec<(usize, usize)> {
    number
        .get_neighbour_cells()
        .into_iter()
        .filter(|(i, j)| matches!(schematic.access(*i, *j), Some('*')))
        .collect()
}

fn process(schematic: &Schematic) -> u32 {
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for number in schematic.read_numbers() {
        for (i, j) in get_adjacent_gears(&number, schematic) {
            gears
                .entry((i, j))
                .and_modify(|v| v.push(number.value))
                .or_insert(vec![number.value]);
        }
    }

    gears
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let schematic = Schematic::from(input);
    println!("Part 2: {}", process(&schematic));
}
