advent_of_code::solution!(2);
use regex::Regex;

use std::cmp;

const TARGET: [u32; 3] = [12, 13, 14]; // r,g,b

fn parse_line(line: &str) -> [u32; 3] {
    let re = Regex::new(r"(\d+) (blue|red|green)").unwrap();
    re.captures_iter(line)
        .map(|c| {
            let (_, [nb, color]) = c.extract();
            (nb.parse::<u32>().unwrap(), color)
        })
        .fold([0, 0, 0], |sum: [u32; 3], (nb, color)| match color {
            // "red" => [sum[0] + nb, sum[1], sum[2]],
            // "green" => [sum[0], sum[1] + nb, sum[2]],
            // "blue" => [sum[0], sum[1], sum[2] + nb],
            "red" => [cmp::max(sum[0], nb), sum[1], sum[2]],
            "green" => [sum[0], cmp::max(sum[1], nb), sum[2]],
            "blue" => [sum[0], sum[1], cmp::max(sum[2], nb)],
            _ => panic!("not a color"),
        })
}

// fn parse_line(line: &str) -> [u32; 3] {
//     let re = Regex::new(r"(\d+) (blue|red|green)").unwrap();
//     re.captures_iter(line)
//         .map(|c| {
//             let (_, [nb, color]) = c.extract();
//             (nb.parse::<u32>().unwrap(), color)
//         })
//         .fold([0, 0, 0], |sum: [u32; 3], (nb, color)| match color {
//             // "red" => [sum[0] + nb, sum[1], sum[2]],
//             // "green" => [sum[0], sum[1] + nb, sum[2]],
//             // "blue" => [sum[0], sum[1], sum[2] + nb],
//             "red" => [cmp::max(sum[0], nb), sum[1], sum[2]],
//             "green" => [sum[0], cmp::max(sum[1], nb), sum[2]],
//             "blue" => [sum[0], sum[1], cmp::max(sum[2], nb)],
//             _ => panic!("not a color"),
//         })
// }

pub fn part_one(input: &str) -> Option<u32> {
    let v: u32 = input
        .lines()
        .map(parse_line)
        .enumerate()
        .filter_map(|(i, val)| {
            if val[0] <= TARGET[0] && val[1] <= TARGET[1] && val[2] <= TARGET[2] {
                Some((i + 1) as u32)
            } else {
                None
            }
        })
        .sum();
    Some(v)
}

pub fn part_two(input: &str) -> Option<u32> {
    let v: u32 = input
        .lines()
        .map(parse_line)
        .fold(0, |pow, arr| pow + arr[0] * arr[1] * arr[2]);
    Some(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
