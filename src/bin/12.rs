use slice_group_by::StrGroupBy;
use std::fmt::DebugMap;
advent_of_code::solution!(12);

#[derive(Debug)]
enum SpringGroup {
    OP(usize),
    KO(usize),
    UNKNOWN(usize),
}

impl SpringGroup {
    fn from_str(s: &str) -> Self {
        match s.chars().nth(0).unwrap() {
            '.' => Self::OP(s.chars().count()),
            '#' => Self::KO(s.chars().count()),
            '?' => Self::UNKNOWN(s.chars().count()),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Spring<'a> {
    raw: &'a str,
    raw_map: &'a str,
    raw_records: &'a str,
    map: Vec<SpringGroup>,
    records: Vec<usize>,
}

impl<'a> Spring<'a> {
    fn new(
        raw: &'a str,
        raw_map: &'a str,
        raw_records: &'a str,
        map: Vec<SpringGroup>,
        records: Vec<usize>,
    ) -> Self {
        Self {
            raw,
            raw_map,
            raw_records,
            map,
            records,
        }
    }

    fn from_input(line: &'a str) -> Self {
        let part: Vec<&str> = line.split_whitespace().collect();

        let records: Vec<usize> = part[1].split(",").filter_map(|s| s.parse().ok()).collect();

        let map: Vec<SpringGroup> = part[0]
            .linear_group_by(|a, b| a == b)
            .map(SpringGroup::from_str)
            // .filter(|v| !matches!(v, SpringGroup::OP(_)))
            .collect();
        Self::new(line, part[0], part[1], map, records)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut springs: Vec<Spring> = input.lines().map(Spring::from_input).collect();
    dbg!(springs);

    todo!()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
