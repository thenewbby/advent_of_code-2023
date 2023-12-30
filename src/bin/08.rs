use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug)]
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

fn traverse_land(patern: &str, map: HashMap<&str, Node>) -> u32 {
    let mut steps: usize = 0;
    let mut next_node = "AAA";

    while next_node != "ZZZ" {
        match patern.chars().nth(steps % patern.len()).unwrap() {
            'L' => next_node = map.get(next_node).unwrap().left_path,
            'R' => next_node = map.get(next_node).unwrap().right_path,
            _ => todo!(),
        }
        steps += 1;
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

    let res = traverse_land(patern, hash);

    Some(res)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
