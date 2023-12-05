use day_05::*;

fn main() {
    let input = include_str!("../input.txt");
    let (values, maps) = parse_input(input);
    println!("Part 1: {}", process(&values, &maps));
}
