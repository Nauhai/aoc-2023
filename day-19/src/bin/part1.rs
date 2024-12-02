use day_19::*;
use day_19::Action::*;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/input.txt");
    let mut split = input.split("\n\n");
    let workflows = parse_workflows(split.next().unwrap());
    let parts = parse_parts(split.next().unwrap());
    println!("Part 1: {}", process(workflows, parts));
}

fn process(workflows: HashMap<String, Workflow>, parts: Vec<Part>) -> u32 {
    let mut accepted = Vec::new();

    for part in parts {
        let mut current = workflows.get("in").unwrap();
        
        loop {
            match current.treat(&part) {
                Accept => break accepted.push(part),
                Reject => break,
                Send(workflow) => current = workflows.get(workflow).unwrap(),
            }
        }
    }

    accepted
        .into_iter()
        .map(|p| p.rating_number())
        .sum()
}

#[test]
fn test() {
    let input = include_str!("../input/testinput.txt");
    let mut split = input.split("\n\n");
    let workflows = parse_workflows(split.next().unwrap());
    let parts = parse_parts(split.next().unwrap());
    assert_eq!(process(workflows, parts), 19114);
}
