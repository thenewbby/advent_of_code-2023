advent_of_code::solution!(1);

#[macro_use]
extern crate icecream;

pub fn part_one(input: &str) -> Option<u32> {
    // ic!(input);

    // let mut v: u32 = 0;
    // let mut s_first: Option<char> = None;
    // let mut s_last: Option<char> = None;

    // for c in input.chars() {
    //     if c.is_ascii_digit() {
    //         match s_first {
    //             None => s_first = Some(c),
    //             Some(_) => s_last = Some(c),
    //         }
    //     } else if c.is_ascii_control() {
    //         let d = s_first.unwrap().to_digit(10).unwrap();

    //         match s_last {
    //             None => {
    //                 v += d * 10 + d;
    //             }
    //             Some(val) => {
    //                 let u: u32 = val.to_digit(10).unwrap();
    //                 v += d * 10 + u;
    //             }
    //         }
    //         s_first = None;
    //         s_last = None;
    //     }
    // }

    let var: u32 = input
        .lines()
        .map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            10 * first + last
        })
        .sum();
    Some(var)
}

const MATCHES: [(&str, u32); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn find_num_in_line(line: &str) -> u32 {
    // println!("{line}");
    let mut v: Vec<(usize, u32)> = MATCHES
        .into_iter()
        .flat_map(|(s, val)| line.match_indices(s).map(move |(i, _)| (i, val)))
        .collect();

    // dbg!(v);
    v.sort_by(|a, b| a.0.cmp(&b.0));
    // println!("{:?}   {:?}", v, 10 * v[0].1 + v.last().unwrap().1);
    10 * v[0].1 + v.last().unwrap().1
}

pub fn part_two(input: &str) -> Option<u32> {
    let var: u32 = input.lines().map(|line| find_num_in_line(line)).sum();
    Some(var)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(280));
    }
}
