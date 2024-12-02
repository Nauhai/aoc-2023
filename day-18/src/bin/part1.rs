use day_18::*;

fn main() {
    let input = include_str!("../input/input.txt");
    let commands = parse_commands(input);
    println!("Part 1: {}", process(&commands));
}

fn process(commands: &[Command]) -> isize {
    let corners = get_corners(commands);
    compute_area(&corners)
}

#[test]
fn test() {
    let input = include_str!("../input/testinput.txt");
    assert_eq!(process(&parse_commands(input)), 62);
}
