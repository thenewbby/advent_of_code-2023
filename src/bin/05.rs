advent_of_code::solution!(5);

use pathfinding::directed::bfs::bfs;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
type Link = (u32, u32);

fn page_data_to_vec_link(table: &str) -> Vec<Link> {
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

fn find_path<'a>(conv_table: HashMap<&str, Node>) -> Vec<&'a str> {
    let result = bfs(
        &conv_table.get("location").unwrap(),
        |n| {
            n.conversion_from
                .iter()
                .map(|s| conv_table.get(s).unwrap())
                .collect::<Vec<&Node>>()
        },
        |n| n.name == "seed",
    );
    dbg!(result);
    Vec::new()
}

#[derive(Debug, Clone)]
struct Node<'a> {
    name: &'a str,
    conversion_from: Vec<&'a str>,
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
            conversion_to: HashMap::new(),
            conversion_from: Vec::new(),
        }
    }

    fn add_convertion(&mut self, to: &'a str, table: &'a str) {
        self.conversion_to.insert(to, page_data_to_vec_link(table));
    }

    fn add_back_path(&mut self, from: &'a str) {
        self.conversion_from.push(from);
    }
}

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    // println!("{}", input);
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

    for conv in conv_iter {
        let (_, [from, to, table]) = conv.extract();

        if !conversions.contains_key(from) {
            conversions.insert(from, Node::new(from));
        }
        if !conversions.contains_key(to) {
            conversions.insert(to, Node::new(to));
        }

        conversions.get_mut(from).unwrap().add_convertion(to, table);
        conversions.get_mut(to).unwrap().add_back_path(from);
    }
    // dbg!(conversions);

    let path: Vec<&str> = find_path(conversions);

    None
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
