use std::vec;

use pathfinding::num_traits::Zero;

advent_of_code::solution!(9);

fn generate_next_line(line: &[i64]) -> Vec<i64> {
    line.windows(2).map(|slice| slice[1] - slice[0]).collect()
}

fn front_propagate(workspace: &mut [Vec<i64>]) {
    for i in (0..workspace.len() - 1).rev() {
        workspace[i].push(workspace[i].last().unwrap() + workspace[i + 1].last().unwrap())
    }
}

// fn back_propagate(workspace: &mut [Vec<i64>]) {
//     for i in (0..workspace.len() - 1).rev() {
//         workspace[i].push(workspace[i].last().unwrap() + workspace[i + 1].last().unwrap())
//     }
// }

fn frontrapolate(history: Vec<i64>) -> i64 {
    let mut workspace: Vec<Vec<_>> = vec![history];
    // println!("{:?}", workspace);

    while workspace.last().unwrap().iter().any(|v| !v.is_zero()) {
        workspace.push(generate_next_line(workspace.last().unwrap()));
    }
    front_propagate(workspace.as_mut_slice());
    // println!("{:?}", workspace);

    workspace[0].last().unwrap().to_owned()
}

fn backapolate(history: Vec<i64>) -> i64 {
    let mut workspace: Vec<Vec<_>> = vec![history];
    workspace[0].reverse();

    // println!("{:?}", workspace);

    while workspace.last().unwrap().iter().any(|v| !v.is_zero()) {
        workspace.push(generate_next_line(workspace.last().unwrap()));
    }
    front_propagate(workspace.as_mut_slice());
    // println!("{:?}", workspace);

    workspace[0].last().unwrap().to_owned()
}

pub fn part_one(input: &str) -> Option<i64> {
    let histories: Vec<Vec<_>> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    // println!("{:?}", histories);

    Some(histories.iter().map(|v| frontrapolate(v.to_owned())).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let histories: Vec<Vec<_>> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    // println!("{:?}", histories);

    Some(histories.iter().map(|v| backapolate(v.to_owned())).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
