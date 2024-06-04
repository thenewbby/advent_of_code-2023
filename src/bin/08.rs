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
#[derive(Debug)]
struct Ghost<'a> {
    pos: &'a str,
    cycle: Option<usize>,
}

fn all_node_finish(gh: &[Ghost]) -> bool {
    gh.iter().all(|g| g.pos.ends_with('Z'))
}

fn all_ghosts_cycled(gh: &[Ghost]) -> bool {
    gh.iter().all(|g| g.cycle.is_some())
}

fn traverse_land<'a>(patern: &str, ghosts: &mut Vec<Ghost<'a>>, map: HashMap<&str, Node<'a>>) {
    let mut cycle: usize = 1;

    // println!("{:?}", ghosts);
    while !all_ghosts_cycled(&ghosts) {
        ghosts
            .iter_mut()
            .filter(|g| g.cycle.is_none())
            .for_each(|g| {
                // match patern.chars().nth(steps % patern.len()).unwrap() {
                patern.chars().for_each(|c| {
                    match c {
                        'L' => g.pos = map.get(g.pos).unwrap().left_path,
                        'R' => g.pos = map.get(g.pos).unwrap().right_path,
                        _ => todo!(),
                    };
                });
                if g.pos.ends_with('Z') {
                    g.cycle = Some(cycle);
                    // println!("{:?}", g);
                }
            });
        // println!("{:?}", ghosts);

        cycle += 1;
    }

    // while !all_node_finish(&ghosts) {
    //     // println!("{:?}", next_node);

    //     ghosts
    //         .iter()
    //         .map(
    //             |&s| match patern.chars().nth(steps % patern.len()).unwrap() {
    //                 'L' => map.get(s.pos).unwrap().left_path,
    //                 'R' => map.get(s.pos).unwrap().right_path,
    //                 _ => todo!(),
    //             },
    //         )
    //         .collect();

    //     steps += 1;
    //     if steps % 1000000 == 0 {
    //         println!("{:?}:  {:?}", steps, next_node);
    //     }
    // }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines_iter = input.lines();
    let patern = &lines_iter.next().unwrap();
    lines_iter.next();

    let mut hash: HashMap<&str, Node> = HashMap::new();

    lines_iter.for_each(|line| {
        let node = Node::new(line);
        hash.insert(node.name, node);
    });

    let mut ghosts: Vec<Ghost> = vec![Ghost {
        pos: "AAA",
        cycle: None,
    }];
    traverse_land(patern, &mut ghosts, hash);

    let min_shared_cycles = ghosts
        .into_iter()
        .filter_map(|ghost| ghost.cycle)
        .fold(1, lcm);
    Some(min_shared_cycles * patern.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines_iter = input.lines();
    let patern = &lines_iter.next().unwrap();
    lines_iter.next();

    let mut hash: HashMap<&str, Node> = HashMap::new();

    lines_iter.for_each(|line| {
        let node = Node::new(line);
        hash.insert(node.name, node);
    });

    let mut ghosts: Vec<_> = hash
        .clone()
        .into_keys()
        .filter(|&k| k.ends_with('A'))
        .map(|k| Ghost {
            pos: k,
            cycle: None,
        })
        .collect();

    // println!("{:?}", ghosts);

    traverse_land(patern, &mut ghosts, hash);
    // println!("{:?}", ghosts);
    let min_shared_cycles = ghosts
        .into_iter()
        .filter_map(|ghost| ghost.cycle)
        .fold(1, lcm);
    println!("{:?}", min_shared_cycles);

    Some(min_shared_cycles * patern.len())
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
