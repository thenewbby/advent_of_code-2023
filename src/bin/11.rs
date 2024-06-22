use itertools::Itertools;
use std::ops::Sub;
advent_of_code::solution!(11);

#[derive(Debug)]
struct Vector(i64, i64);

impl Vector {
    fn norm(&self) -> u64 {
        (self.0.abs() + self.1.abs()) as u64
    }
}

#[derive(Debug, Clone, Copy)]
struct Position(usize, usize);

impl Sub for Position {
    type Output = Vector;
    fn sub(self, other: Self) -> Self::Output {
        Vector(
            self.0 as i64 - other.0 as i64,
            self.1 as i64 - other.1 as i64,
        )
    }
}

type Size = (usize, usize);

#[derive(Debug, Clone, Copy)]
struct Galaxie {
    position: Position,
}

#[derive(Debug)]
struct GalaxieMap {
    size: Size,
    galaxies: Vec<Galaxie>,
}

impl GalaxieMap {
    fn new(galaxies: Vec<Galaxie>, size: Size) -> Self {
        Self { galaxies, size }
    }

    fn from_input(input: &str) -> Self {
        let size = (
            input.lines().count(),
            input.lines().nth(0).unwrap().chars().count(),
        );
        Self::new(
            input
                .lines()
                .enumerate()
                .map(|(line_num, l)| {
                    l.chars().enumerate().filter_map(move |(char_num, c)| {
                        if c == '#' {
                            Some(Galaxie::new(Position(line_num, char_num)))
                        } else {
                            None
                        }
                    })
                })
                .flatten()
                .collect(),
            size,
        )
    }

    fn expand_map(&mut self) {
        for i in (0..self.size.0).rev() {
            if !self.galaxies.iter().any(|g| g.position.0 == i) {
                // println!("No galaxies in y {}", i);
                for galaxie in &mut self.galaxies {
                    if galaxie.position.0 > i {
                        // print!("\t galaxies shifted {:?} ", galaxie.position);
                        galaxie.position.0 += 1;
                        // println!("to {:?}", galaxie.position);
                    }
                }
                self.size.0 += 1;
            }
        }

        for i in (0..self.size.1).rev() {
            if !self.galaxies.iter().any(|g| g.position.1 == i) {
                // println!("No galaxies in x {}", i);
                for galaxie in &mut self.galaxies {
                    if galaxie.position.1 > i {
                        galaxie.position.1 += 1;
                    }
                }
                self.size.1 += 1;
            }
        }
    }

    fn get_shotest_path_comb(&self) -> u64 {
        self.galaxies
            .iter()
            .tuple_combinations()
            .map(|(a, b)| (a.position - b.position).norm())
            .sum()
    }
}

impl Galaxie {
    fn new(position: Position) -> Self {
        Self { position }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut galaxies: GalaxieMap = GalaxieMap::from_input(input);

    // println!("{:?}", galaxies);

    galaxies.expand_map();
    println!("{:?}", galaxies);
    Some(galaxies.get_shotest_path_comb())
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
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
