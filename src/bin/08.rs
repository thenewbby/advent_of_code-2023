use std::{borrow::Borrow, collections::HashMap, vec};

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy)]
struct Node<'a> {
    name: &'a str,
    left_path: &'a str,
    right_path: &'a str,
}

impl<'a> Node<'a> {
    fn new(line: &str) -> Node {
        let node_name = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        Node {
            name: node_name,
            left_path: left,
            right_path: right,
        }
    }
}

fn all_node_finish(nodes: &[&str]) -> bool {
    nodes.iter().all(|&s| s.ends_with('Z'))
}

fn traverse_land(patern: &str, start_node: Vec<&str>, map: HashMap<&str, Node>) -> u32 {
    let mut steps: usize = 0;
    let mut next_node: Vec<_> = start_node;

    while !all_node_finish(&next_node) {
        // println!("{:?}", next_node);

        next_node = next_node
            .iter()
            .map(
                |&s| match patern.chars().nth(steps % patern.len()).unwrap() {
                    'L' => map.get(s).unwrap().left_path,
                    'R' => map.get(s).unwrap().right_path,
                    _ => todo!(),
                },
            )
            .collect();

        steps += 1;
        if steps % 1000000 == 0 {
            println!("{:?}:  {:?}", steps, next_node);
        }
    }
    steps as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines_iter = input.lines();
    let patern = &lines_iter.next().unwrap();
    lines_iter.next();

    let mut hash: HashMap<&str, Node> = HashMap::new();

    lines_iter.for_each(|line| {
        let node = Node::new(line);
        hash.insert(node.name, node);
    });

    let node: Vec<&str> = vec!["AAA"];
    // println!("{:?}", node);

    let res = traverse_land(patern, node, hash);

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines_iter = input.lines();
    let patern = &lines_iter.next().unwrap();
    lines_iter.next();

    let mut hash: HashMap<&str, Node> = HashMap::new();

    lines_iter.for_each(|line| {
        let node = Node::new(line);
        hash.insert(node.name, node);
    });

    let node: Vec<_> = hash
        .clone()
        .into_keys()
        .filter(|&k| k.ends_with('A'))
        .collect();

    println!("{:?}", node);

    let res = traverse_land(patern, node, hash);

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
