
#[derive(Debug)]
struct Range {
    start: u64,
    length: u64,
}

#[derive(Debug, PartialEq)]
struct MapRange {
    start_end: u64,
    start_from: u64,
    length: u64,
    
}

impl<'a> From <&'a str> for MapRange {
    fn from(value: &'a str) -> MapRange {
        let mut split = value.split(' ');

        MapRange {
            start_end: split.next().unwrap().parse().unwrap(),
            start_from: split.next().unwrap().parse().unwrap(),
            length: split.next().unwrap().parse().unwrap(),
        }
    }
}

impl MapRange {
    fn get_value(&self, val: u64) -> Option<u64> {
        if self.start_from <= val && val < self.start_from+self.length {
            Some(self.start_end+val-self.start_from)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Map {
    ranges: Vec<MapRange>,
}

impl<'a> From<&'a str> for Map {
    fn from(value: &'a str) -> Map {
        Map {
            ranges: value
                .lines()
                .map(MapRange::from)
                .collect(),
        }
    }
}

impl Map {
    pub fn map(&self, val: u64) -> u64 {
        self.ranges
            .iter()
            .filter_map(|r| r.get_value(val))
            .next()
            .unwrap_or(val)
    }

    pub fn map_range(&self, range: Range) -> Vec<Range> {
        // TODO
        vec![]
    }
}

pub fn parse_input(input: &str) -> (Vec<u64>, Vec<Map>) {
    let mut chunks = input.split("\n\n");
    let values = chunks
        .next().unwrap()
        .trim()
        .strip_prefix("seeds: ").unwrap()
        .split(' ')
        .map(|v| v.parse().unwrap())
        .collect();

    let maps = chunks
        .map(|c| c.lines())
        .map(|mut l| {
            l.next();
            let ranges = l.map(MapRange::from).collect();
            Map { ranges }
        })
        .collect();

    (values, maps)
}

pub fn process(values: &[u64], maps: &[Map]) -> u64 {
    let mut values = values.to_vec();

    for map in maps {
        for i in 0..values.len() {
            values[i] = map.map(values[i]);
        }
    }

    *values.iter().min().unwrap()
}

#[test]
fn test_map_from_str() {
    let map = Map::from("0 15 37
37 52 2
39 0 15");

    let should_be = Map {
        ranges: vec![
            MapRange {
                start_from: 15,
                start_end: 0,
                length: 37,
            },
            MapRange {
                start_from: 52,
                start_end: 37,
                length: 2,
            },
            MapRange {
                start_from: 0,
                start_end: 39,
                length: 15,
            },
        ]
    };

    assert_eq!(map, should_be);
}

#[test]
fn test_map() {
    let map = Map::from("50 98 2
52 50 48");

    assert_eq!(map.map(98), 50);
    assert_eq!(map.map(100), 100);
    assert_eq!(map.map(51), 53);
}

#[test]
fn test_parse_input() {
    let (values, maps) = parse_input("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15");

    assert_eq!(values, vec![79, 14, 55, 13]);
    assert_eq!(maps, vec![
        Map::from("50 98 2
52 50 48"),
        Map::from("0 15 37
37 52 2
39 0 15"),
    ]);
}

#[test]
fn test_process() {
    let input = include_str!("../testinput.txt");
    let (values, maps) = parse_input(input);
    assert_eq!(process(&values, &maps), 35);
}
