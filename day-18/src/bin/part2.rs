use day_18::Direction::*;
use day_18::*;

fn main() {
    let input = include_str!("../input/input.txt");
    let commands = parse_commands(input);
    println!("Part 2: {}", process(&commands));
}

fn process(commands: &[Command]) -> isize {
    let cmds: Vec<Command> = commands
        .iter()
        .map(|(_, _, col)| {
            let mut col = col.clone();

            let dir = match col.pop().unwrap() {
                '0' => Right,
                '1' => Down,
                '2' => Left,
                '3' => Up,
                other => panic!("{}", other),
            };

            let len = u32::from_str_radix(&col, 16).unwrap();

            (dir, len, "".to_string())
        })
        .collect();

    let corners = get_corners(&cmds);
    compute_area(&corners)
}

#[test]
fn test() {
    let input = include_str!("../input/testinput.txt");
    assert_eq!(process(&parse_commands(input)), 952408144115);
}
