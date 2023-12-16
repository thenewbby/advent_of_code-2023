advent_of_code::solution!(4);

use regex::{Matches, Regex};

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let re_line = Regex::new(r"Card +\d+: +(.+) \| +(.+)").unwrap();
    let f = re_line.captures(line).unwrap();
    let winning_num: Vec<u32> = f
        .get(1)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let my_num: Vec<u32> = f
        .get(2)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    (winning_num, my_num)
}

// fn count_card_point(tup: (Vec<u32>, Vec<u32>)) -> u32 {
fn count_card_point((winning_num, my_num): (Vec<u32>, Vec<u32>)) -> u32 {
    let nb_num_match = winning_num
        .iter()
        .map(|num| my_num.contains(num))
        .filter(|x| *x)
        .count();
    if nb_num_match == 0 {
        0
    } else {
        1 << (nb_num_match - 1)
    }
}

fn count_card_match((winning_num, my_num): (Vec<u32>, Vec<u32>)) -> usize {
    winning_num
        .iter()
        .map(|num| my_num.contains(num))
        .filter(|x| *x)
        .count()
}

pub fn part_one(input: &str) -> Option<u32> {
    // println!("{}", input);
    let v: u32 = input.lines().map(parse_line).map(count_card_point).sum();
    Some(v)
}

pub fn part_two(input: &str) -> Option<u32> {
    let v: Vec<usize> = input
        .lines()
        .map(parse_line)
        .map(count_card_match)
        .collect();

    let mut arr = vec![1; v.len()];
    for i in 0..v.len() {
        for j in i + 1..i + 1 + v[i] {
            arr[j] += arr[i]
        }
    }
    // println!("{:?}", arr);
    let s: u32 = arr.iter().sum();
    // dbg!(s);

    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
