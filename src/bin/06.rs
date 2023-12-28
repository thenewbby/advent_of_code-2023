use std::iter::zip;

use itertools::Itertools;

advent_of_code::solution!(6);

fn to_distance((prep, run): (&u64, &u64)) -> u64 {
    let mut mi = None;
    let mut ma = None;
    for i in 0..*prep {
        let dist: u64 = i * (prep - i);
        // println!("\t{:?}  {:?}", i, dist);
        // if dist > *run {
        //     println!("\t\tOK {:?}", i);
        // }

        // if dist < *run {
        //     println!("\t\tLOWER {:?}", i);
        // }

        if dist > *run && mi.is_none() {
            // println!("\t\tmin val {:?}", i);
            mi = Some(i);
        } else if dist <= *run && mi.is_some() {
            ma = Some(i);
            // println!("\t\tmax val {:?}", i);
            break;
        }
    }
    ma.unwrap_or(*prep) - mi.unwrap_or(0)
}

pub fn part_one(input: &str) -> Option<u32> {
    let data: Vec<Vec<u64>> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect()
        })
        .collect();

    // println!("{:?}", data);
    let mut nb = 1;
    for i in zip(&data[0], &data[1]) {
        println!("{:?}", i);
        nb *= to_distance(i);
    }

    Some(nb as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data: Vec<u64> = input
        .lines()
        .map(|l| l.split_ascii_whitespace().skip(1).collect::<String>())
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    // println!("{:?}", data);
    let nb = to_distance((&data[0], &data[1]));

    Some(nb as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
