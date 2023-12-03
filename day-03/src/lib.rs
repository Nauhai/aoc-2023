use std::collections::HashSet;

use regex::Regex;

#[derive(Debug)]
pub struct Number {
    pub value: u32,
    line: usize,
    start: usize,
    end: usize,
}

impl Number {
    pub fn get_neighbour_cells(&self) -> HashSet<(usize, usize)> {
        let mut neigh = HashSet::new();
        let l = self.line as isize;
        let s = self.start as isize;
        let e = self.end as isize;

        neigh.insert((l, s - 1));
        neigh.insert((l, e));

        for i in s - 1..=e {
            neigh.insert((l - 1, i));
            neigh.insert((l + 1, i));
        }

        neigh
            .iter()
            .filter(|(i, j)| *i >= 0 && *j >= 0)
            .map(|(i, j)| (*i as usize, *j as usize))
            .collect()
    }
}

#[derive(Debug)]
pub struct Schematic {
    matrix: Vec<String>,
}

impl<'a> From<&'a str> for Schematic {
    fn from(value: &'a str) -> Schematic {
        let matrix = value.lines().map(|l| l.to_string()).collect();

        Schematic { matrix }
    }
}

impl Schematic {
    pub fn read_numbers(&self) -> Vec<Number> {
        let re = Regex::new(r"\d+").unwrap();

        self.matrix
            .iter()
            .enumerate()
            .flat_map(|(i, l)| {
                re.find_iter(l).map(move |m| Number {
                    value: m.as_str().parse().unwrap(),
                    line: i,
                    start: m.start(),
                    end: m.end(),
                })
            })
            .collect()
    }

    pub fn access(&self, i: usize, j: usize) -> Option<char> {
        self.matrix.get(i).and_then(|l| l.chars().nth(j))
    }
}

#[test]
fn test_neighbour_cells() {
    let num = Number {
        value: 234,
        line: 0,
        start: 0,
        end: 3,
    };

    let should_be = HashSet::from([(0, 3), (1, 0), (1, 1), (1, 2), (1, 3)]);

    assert_eq!(num.get_neighbour_cells(), should_be);
}
