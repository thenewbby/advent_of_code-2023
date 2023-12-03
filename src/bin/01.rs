advent_of_code::solution!(1);

#[macro_use]
extern crate icecream;

pub fn part_one(input: &str) -> Option<u32> {
    // ic!(input);

    let mut v: u32 = 0;
    let mut s_first: Option<char> = None;
    let mut s_last: Option<char> = None;

    for c in input.chars() {
        if c.is_ascii_digit() {
            match s_first {
                None => s_first = Some(c),
                Some(_) => s_last = Some(c),
            }
        } else if c.is_ascii_control() {
            let d = s_first.unwrap().to_digit(10).unwrap();

            match s_last {
                None => {
                    v += d * 10 + d;
                }
                Some(val) => {
                    let u: u32 = val.to_digit(10).unwrap();
                    v += d * 10 + u;
                }
            }
            s_first = None;
            s_last = None;
        }
    }

    Some(v)
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
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
