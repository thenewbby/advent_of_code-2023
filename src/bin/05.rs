advent_of_code::solution!(5);

use pathfinding::directed::bfs::bfs;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
type Link = (u32, u32);

fn page_data_to_vec_link_from(table: &str) -> Vec<Link> {
    table
        .lines()
        .flat_map(|s| {
            let v: Vec<_> = s
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            // dbg!(v);
            let mut end_v: Vec<(u32, u32)> = Vec::new();

            for i in 0..v[2] {
                end_v.push((v[0] + i, v[1] + i));
            }
            end_v
        })
        .collect()
}

fn page_data_to_vec_link_to(table: &str) -> Vec<Link> {
    table
        .lines()
        .flat_map(|s| {
            let v: Vec<_> = s
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            // dbg!(v);
            let mut end_v: Vec<(u32, u32)> = Vec::new();

            for i in 0..v[2] {
                end_v.push((v[1] + i, v[0] + i));
            }
            end_v
        })
        .collect()
}

fn find_path<'a>(conv_table: &'a HashMap<&'a str, Node<'a>>) -> std::vec::Vec<&'a Node<'a>> {
    // let a: Option<Vec<&Node<'_>>> = bfs(
    //     &conv_table.get("location").unwrap(),
    //     |n| {
    //         n.conversion_from
    //             .keys()
    //             .map(|s| conv_table.get(s).unwrap())
    //             .collect::<Vec<&'a Node<'static>>>()
    //     },
    //     |n| n.name == "seed",
    // );

    let a: Option<Vec<&Node<'_>>> = bfs(
        &conv_table.get("location").unwrap(),
        |n| {
            n.conversion_from
                .keys()
                .map(|s| conv_table.get(s).unwrap())
                .collect::<Vec<_>>()
        },
        |n| n.name == "seed",
    );
    a.unwrap()
}

#[derive(Debug, Clone)]
struct Node<'a> {
    name: &'a str,
    conversion_from: HashMap<&'a str, Vec<Link>>,
    conversion_to: HashMap<&'a str, Vec<Link>>,
}

impl<'a> Hash for Node<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl<'a> Eq for Node<'a> {}

impl<'a> Node<'a> {
    fn new(nm: &'a str) -> Self {
        Node {
            name: nm,
            conversion_from: HashMap::new(),
            conversion_to: HashMap::new(),
        }
    }

    fn add_convertion_from(&mut self, from: &'a str, table: &'a str) {
        self.conversion_from
            .insert(from, page_data_to_vec_link_from(table));
    }

    fn add_convertion_to(&mut self, to: &'a str, table: &'a str) {
        self.conversion_to
            .insert(to, page_data_to_vec_link_to(table));
    }
}

fn transform_seed(seed: u32, transformations: &Vec<&Node<'_>>) -> u32 {
    let mut val = seed;
    for (i, trans) in transformations.iter().enumerate() {
        if trans.name == "location" {
        } else {
            let conversion_table = trans
                .conversion_to
                .get(transformations[i + 1].name)
                .unwrap();
            for conv in conversion_table {
                if conv.0 == val {
                    // println!("{} {}", conv.0, conv.1);
                    val = conv.1;
                    break;
                }
            }
            // println!("{}  {}", transformations[i + 1].name, val);
        }
    }
    val
}

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let seed_regex = Regex::new(r"seeds: (.+)").unwrap();

    let conversion_regex = Regex::new(r"(\w+)-to-(\w+) map:\n((?:.+\n)+)").unwrap();

    let mut conversions: HashMap<&str, Node> = HashMap::new();

    let seed_cap: regex::Captures<'_> = seed_regex.captures(input).unwrap();
    let seeds: Vec<u32> = seed_cap
        .get(1)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let conv_iter = conversion_regex.captures_iter(input);
    println!("conv_iter");

    for conv in conv_iter {
        let (_, [from, to, table]) = conv.extract();
        println!("from {}  to {} ", from, to);

        if !conversions.contains_key(from) {
            conversions.insert(from, Node::new(from));
        }
        if !conversions.contains_key(to) {
            conversions.insert(to, Node::new(to));
        }

        conversions
            .get_mut(to)
            .unwrap()
            .add_convertion_from(from, table);

        conversions
            .get_mut(from)
            .unwrap()
            .add_convertion_to(to, table);
    }
    println!("Find path");
    // dbg!(conversions);
    let mut path: Vec<&Node<'_>> = find_path(&conversions);
    println!("Finded path");
    // dbg!(path);

    // for p in &path {
    //     println!("{}", p.name);
    // }
    path.reverse();
    // for p in &path {
    //     println!("{}", p.name);
    // }
    let mut position: Vec<u32> = Vec::new();
    for seed in seeds {
        let pos = transform_seed(seed, &path);
        position.push(pos);
    }
    println!("{:?}", position);
    Some(position.iter().min().unwrap().to_owned())
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
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
