use std::cmp::{Ordering};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Part(u32, u32, u32, u32);

impl Part {
    pub fn rating_number(&self) -> u32 {
        let Part(x, m, a, s) = &self;
        x + m + a + s
    }
}

impl<'a> From<&'a str> for Part {
    fn from(value: &str) -> Part {
        let mut values = value.strip_prefix('{').unwrap().strip_suffix('}').unwrap().split(',')
            .map(|v| v.chars().skip(2).collect::<String>().parse().unwrap());

        Part(values.next().unwrap(), values.next().unwrap(), values.next().unwrap(), values.next().unwrap())
    }
}

#[derive(Debug)]
pub enum Action {
    Send(String),
    Accept,
    Reject,
}

use Action::*;

impl<'a> From<&'a str> for Action {
    fn from(value: &str) -> Action {
        match value {
            "A" => Accept,
            "R" => Reject,
            name => Send(name.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Condition(pub char, pub Ordering, pub u32);

impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0, match self.1 {
            Ordering::Less => '<',
            Ordering::Greater => '>',
            _ => panic!(),
        }, self.2)
    }
}

#[derive(Debug)]
pub enum Rule {
    If(Condition, Action),
    Else(Action)
}

use Rule::*;

impl<'a> From<&'a str> for Rule {
    fn from(value: &str) -> Rule {
        if value.contains(':') {
            let mut split = value.split(':');

            let mut chars = split.next().unwrap().chars();
            let c = chars.next().unwrap();
            let ord = match chars.next().unwrap() {
                '<' => Ordering::Less,
                '>' => Ordering::Greater,
                o => panic!("{}", o),
            };
            let val = chars.as_str().parse().unwrap();

            let action = Action::from(split.next().unwrap());

            If(Condition(c, ord, val), action)
        } else {
            let action = Action::from(value);

            Else(action)
        }
    }
}

#[derive(Debug)]
pub struct Workflow(pub Vec<Rule>);

impl Workflow {
    pub fn treat(&self, &Part(x, m, a, s): &Part) -> &Action {
        for rule in &self.0 {
            match rule {
                If(Condition(c, ord, val), action) => {
                    let cmp = match c {
                        'x' => x.cmp(&val),
                        'm' => m.cmp(&val),
                        'a' => a.cmp(&val),
                        's' => s.cmp(&val),
                        o => panic!("{}", o),
                    };

                    if cmp == *ord {
                        return action;
                    }
                }
                Else(action) => return action,
            }
        }
        panic!();
    }

}

impl<'a> From<&'a str> for Workflow {
    fn from(value: &str) -> Workflow {
        let rules = value
            .split(',')
            .map(Rule::from)
            .collect();
        
        Workflow(rules)
    }
}

pub fn parse_workflows(input: &str) -> HashMap<String, Workflow> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split("{");
            let name = split.next().unwrap().to_string();
            let workflow = Workflow::from(split.next().unwrap().strip_suffix("}").unwrap());
            (name, workflow)
        })
        .collect()
}

pub fn parse_parts(input: &str) -> Vec<Part> {
    input
        .lines()
        .map(Part::from)
        .collect()
}
