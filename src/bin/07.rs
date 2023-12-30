advent_of_code::solution!(7);
use std::{cmp::Ordering, fmt};

const CARD_VALUE: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardMultiple {
    One(usize),
    Two(usize),
    Double(usize, usize),
    Three(usize),
    House(usize, usize),
    Four(usize),
    Five(usize),
}

impl fmt::Debug for CardMultiple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CardMultiple::One(val) => write!(f, "One({})", CARD_VALUE[val]),
            CardMultiple::Two(val) => write!(f, "Two({})", CARD_VALUE[val]),
            CardMultiple::Double(val, val2) => {
                write!(f, "Double({}, {})", CARD_VALUE[val], CARD_VALUE[val2])
            }
            CardMultiple::Three(val) => write!(f, "Three({})", CARD_VALUE[val]),
            CardMultiple::House(val, val2) => {
                write!(f, "House({}, {})", CARD_VALUE[val], CARD_VALUE[val2])
            }
            CardMultiple::Four(val) => write!(f, "Four({})", CARD_VALUE[val]),
            CardMultiple::Five(val) => write!(f, "Five({})", CARD_VALUE[val]),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CardHand {
    s: String,
    hand_card: [u32; 13],
    hand_abstract: Vec<CardMultiple>,
}

impl CardHand {
    pub fn new(hand: &str) -> Self {
        let mut arr: [u32; 13] = [0; 13];

        // for c in hand.chars() {
        //     for (i, cv) in CARD_VALUE.iter().enumerate() {
        //         if *cv == c {
        //             arr[i] += 1;
        //             break;
        //         }
        //     }
        // }

        for x in hand.chars() {
            let pos = CARD_VALUE.iter().position(|&c| c == x).unwrap();
            arr[pos] += 1;
        }

        let mut v: Vec<CardMultiple> = arr
            .iter()
            .enumerate()
            .filter_map(|(i, nb)| match nb {
                5 => Some(CardMultiple::Five(i)),
                4 => Some(CardMultiple::Four(i)),
                3 => Some(CardMultiple::Three(i)),
                2 => Some(CardMultiple::Two(i)),
                1 => Some(CardMultiple::One(i)),
                _ => None,
            })
            .collect();
        // println!("{:?}", v);
        v.sort_by(|a, b| b.cmp(a));

        let mut it = v.iter().enumerate().peekable();

        let mut vv = Vec::new();

        // for mut i in 0..v.len() {
        while let Some((i, &cm)) = &it.next() {
            // println!("{}", i);
            match cm {
                CardMultiple::Two(val) => {
                    if i + 1 < v.len() {
                        match v[i + 1] {
                            CardMultiple::Two(val2) => {
                                vv.push(CardMultiple::Double(val, val2));
                                it.nth(0);
                            }
                            _ => vv.push(cm),
                        }
                    } else {
                        vv.push(cm)
                    }
                }
                CardMultiple::Three(val) => {
                    if i + 1 < v.len() {
                        match v[i + 1] {
                            CardMultiple::Two(val2) => {
                                vv.push(CardMultiple::House(val, val2));
                                it.nth(0);
                            }
                            _ => vv.push(cm),
                        }
                    } else {
                        vv.push(cm)
                    }
                }
                _ => vv.push(cm),
            }
        }
        // println!("{:?}", vv);

        Self {
            s: hand.to_owned(),
            hand_card: arr,
            hand_abstract: vv,
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
        // println!("{:?}  {:?}", self.hand_abstract, self.s);
        // println!("{:?}  {:?}", other.hand_abstract, other.s);

        match self.hand_abstract.cmp(&other.hand_abstract) {
            Ordering::Less => {
                // println!("LESS");
                Ordering::Less
            }
            Ordering::Equal => {
                // println!("EQUAL");
                Ordering::Equal
            }
            Ordering::Greater => {
                // println!("GREAT");
                Ordering::Greater
            }
        }
    }
}

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

    // for (hand, v) in &player {
    //     println!("{} {} {:?}", v, hand.s, hand.hand_abstract);
    // }

    let res = player
        .iter()
        .enumerate()
        .map(|(i, (hand, chip))| {
            // println!(
            //     "{} {} {:?} | {} {}",
            //     chip,
            //     hand.s,
            //     hand.hand_abstract,
            //     i + 1,
            //     chip * (i as u32 + 1)
            // );
            chip * (i as u32 + 1)
        })
        .sum();

    // println!("{:?}", res);
    Some(res)
    // None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_real_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(255048101));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_hands() {
        assert_eq!(
            CardHand::new("AAAAA").hand_abstract,
            vec![CardMultiple::Five(12)]
        );

        assert_eq!(
            CardHand::new("AA8AA").hand_abstract,
            vec![CardMultiple::Four(12), CardMultiple::One(6)]
        );

        assert_eq!(
            CardHand::new("23332").hand_abstract,
            vec![CardMultiple::House(1, 0)]
        );

        assert_eq!(
            CardHand::new("TTT98").hand_abstract,
            vec![
                CardMultiple::Three(8),
                CardMultiple::One(7),
                CardMultiple::One(6)
            ]
        );

        assert_eq!(
            CardHand::new("23432").hand_abstract,
            vec![CardMultiple::Double(1, 0), CardMultiple::One(2)]
        );
        assert_eq!(
            CardHand::new("A23A4").hand_abstract,
            vec![
                CardMultiple::Two(12),
                CardMultiple::One(2),
                CardMultiple::One(1),
                CardMultiple::One(0)
            ]
        );
        assert_eq!(
            CardHand::new("23456").hand_abstract,
            vec![
                CardMultiple::One(4),
                CardMultiple::One(3),
                CardMultiple::One(2),
                CardMultiple::One(1),
                CardMultiple::One(0)
            ]
        );
    }

    #[test]
    fn test_ordering() {
        let v_res = vec![
            vec![
                CardMultiple::One(4),
                CardMultiple::One(3),
                CardMultiple::One(2),
                CardMultiple::One(1),
                CardMultiple::One(0),
            ],
            vec![
                CardMultiple::Two(12),
                CardMultiple::One(2),
                CardMultiple::One(1),
                CardMultiple::One(0),
            ],
            vec![CardMultiple::Double(1, 0), CardMultiple::One(2)],
            vec![
                CardMultiple::Three(8),
                CardMultiple::One(7),
                CardMultiple::One(6),
            ],
            vec![CardMultiple::House(1, 0)],
            vec![CardMultiple::Four(12), CardMultiple::One(6)],
            vec![CardMultiple::Five(12)],
        ];
        let mut v = v_res.clone();

        v.shuffle(&mut thread_rng());

        // rand::thread_rng().shuffle(&mut v);
        v.sort();

        assert_eq!(v, v_res)
    }
}
