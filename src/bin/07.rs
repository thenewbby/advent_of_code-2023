advent_of_code::solution!(7);
use std::cmp::Ordering;

use itertools::Itertools;
#[derive(Debug, PartialEq, Eq)]
struct CardHand {
    s: String,
    hand_card: [u32; 13],
    class: u32,
    highier_card_multiple: usize,
    nb_diff_card: usize,
}

impl CardHand {
    pub fn new(hand: &str) -> Self {
        let mut arr: [u32; 13] = [0; 13];
        for c in hand.chars() {
            for (i, cv) in CARD_VALUE.iter().enumerate() {
                if *cv == c {
                    arr[i] += 1;
                    break;
                }
            }
        }

        let cl = *arr.iter().max().unwrap();

        let high_val_mul = arr.iter().rev().position(|v| *v == cl).unwrap();

        let diff = arr.iter().filter(|&n| *n != 0).count();

        Self {
            s: hand.to_owned(),
            hand_card: arr,
            class: cl,
            highier_card_multiple: high_val_mul,
            nb_diff_card: diff,
        }
    }

    fn cmp_higher(&self, other: &Self) -> Ordering {
        let my_higher_card_iter = self
            .hand_card
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(i, &n)| if n != 0 { Some(i) } else { None });
        let their_higher_card_iter = other
            .hand_card
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(i, &n)| if n != 0 { Some(i) } else { None });

        // dbg!(my_higher_card_iter.collect::<Vec<_>>());
        // dbg!(their_higher_card_iter.collect::<Vec<_>>());
        match my_higher_card_iter.cmp(their_higher_card_iter) {
            Ordering::Less => {
                // println!("HIGHER GREAT");
                Ordering::Greater
            }
            Ordering::Equal => {
                println!("{:?}", self);
                println!("{:?}", other);
                println!("_______________________");
                Ordering::Equal
            }
            Ordering::Greater => {
                // println!("HIGHER LESS");
                Ordering::Less
            }
        }
        // todo!()
    }

    fn cmp_double_pair(&self, other: &Self) -> Ordering {
        let my_higher_pairs_iter = self
            .hand_card
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(i, &n)| {
                if n == self.highier_card_multiple as u32 {
                    Some(i)
                } else {
                    None
                }
            });

        let their_higher_pairs_iter =
            other
                .hand_card
                .iter()
                .rev()
                .enumerate()
                .filter_map(|(i, &n)| {
                    if n == other.highier_card_multiple as u32 {
                        Some(i)
                    } else {
                        None
                    }
                });

        match my_higher_pairs_iter.cmp(their_higher_pairs_iter) {
            // match self.highier_card_multiple.cmp(&other.highier_card_multiple) {
            Ordering::Less => {
                // println!("DOUBLE GREAT");
                Ordering::Less
            }
            Ordering::Equal => self.cmp_higher(other),
            Ordering::Greater => {
                // println!("DOUBLE LESS");
                Ordering::Greater
            }
        }
    }

    fn cmp_same_class(&self, other: &Self) -> Ordering {
        match self.nb_diff_card.cmp(&other.nb_diff_card) {
            Ordering::Less => {
                // println!("SAME GREAT");
                Ordering::Greater
            }
            Ordering::Equal => self.cmp_double_pair(other),
            Ordering::Greater => {
                // println!("SAME LESS");
                Ordering::Less
            }
        }
    }
}

impl PartialOrd for CardHand {
    fn partial_cmp(&self, other: &CardHand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> Ordering {
        // println!("%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");
        // println!("comparing \n{:?}\nto\n{:?}", self, other);

        match self.class.cmp(&other.class) {
            Ordering::Less => {
                // println!("LESS");
                Ordering::Less
            }
            Ordering::Equal => self.cmp_same_class(other),
            Ordering::Greater => {
                // println!("GREAT");
                Ordering::Greater
            }
        }
    }
}

const CARD_VALUE: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

pub fn part_one(input: &str) -> Option<u32> {
    let mut player: Vec<_> = input
        .lines()
        .map(|l| {
            let v: Vec<&str> = l.split_ascii_whitespace().collect();
            (CardHand::new(v[0]), v[1].parse::<u32>().unwrap())
        })
        .collect();
    // println!("{:#?}", player);

    player.sort_by(|a, b| a.0.cmp(&b.0));

    // for (hand, _) in &player {
    //     println!("{:?}", hand);
    // }
    let res = player
        .iter()
        .enumerate()
        .map(|(i, (_, chip))| chip * (i as u32 + 1))
        .sum();
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
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
