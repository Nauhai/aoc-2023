use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

use Direction::*;

impl Direction {
    pub fn displacement(&self) -> (isize, isize) {
        match self {
            Up => (-1, 0),
            Right => (0, 1),
            Down => (1, 0),
            Left => (0, -1),
        }
    }
}

impl<'a> From<&'a str> for Direction {
    fn from(val: &'a str) -> Self {
        match val {
            "U" => Up,
            "R" => Right,
            "D" => Down,
            "L" => Left,
            _ => panic!("{}", val),
        }
    }
}

pub type Command = (Direction, u32, String);

pub fn parse_commands(input: &str) -> Vec<Command> {
    let re = Regex::new(r"(U|R|D|L) (\d+) \(#(\w{6})\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let (_, [dir, len, col]) = cap.extract();
            (Direction::from(dir), len.parse().unwrap(), col.to_string())
        })
        .collect()
}

pub fn get_corners(commands: &[Command]) -> Vec<(isize, isize)> {
    let mut corners = vec![(0, 0)];
    let (mut i, mut j) = (0, 0);

    for (dir, len, _) in commands {
        let (di, dj) = dir.displacement();
        i += di * (*len as isize);
        j += dj * (*len as isize);
        corners.push((i, j));
    }

    corners
}

pub fn compute_area(corners: &[(isize, isize)]) -> isize {
    let mut area = 0;
    let mut walls = 0;

    for k in 0..(corners.len() - 1) {
        let (i1, j1) = corners[k];
        let (i2, j2) = corners[k + 1];

        area += (i1 + i2) * (j1 - j2);
        walls += (j2 - j1).abs();
        walls += (i2 - i1).abs();
    }

    area /= 2;
    area += walls / 2 + 1;
    area
}
