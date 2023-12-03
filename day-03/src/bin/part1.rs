use day_03::*;

pub fn is_part_number(number: &Number, schematic: &Schematic) -> bool {
    number
        .get_neighbour_cells()
        .iter()
        .filter_map(|(i, j)| schematic.access(*i, *j))
        .any(|c| c != '.')
}

fn process(schematic: &Schematic) -> u32 {
    schematic
        .read_numbers()
        .iter()
        .filter(|n| is_part_number(n, schematic))
        .map(|n| n.value)
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let schematic = Schematic::from(input);
    println!("Part 1: {}", process(&schematic));
}
