use day_05::*;

fn main() {
    let input = include_str!("../input.txt");
    let (ranges, maps) = parse_input(input);
    let mut ranges = ranges.into_iter();

    let mut values = Vec::new();
    while let Some(start) = ranges.next() {
        let length = ranges.next().unwrap();
        println!("{start} {length}");
        for x in start..start+length {
            values.push(x);
        }
    }

    println!("Part 2: {}", process(&values, &maps));
}
