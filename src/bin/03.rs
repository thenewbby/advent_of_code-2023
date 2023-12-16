use regex::{Matches, Regex};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // println!("{}", input);
    let re_number = Regex::new(r"([\d]+)").unwrap();
    let re_special = Regex::new(r"([^.\w\n])").unwrap();

    let line_size = input.lines().next().unwrap().len();

    let numbers: Vec<regex::Match<'_>> = re_number.find_iter(input).collect();

    // let v = re_special.find(input);

    // dbg!(v);

    let v: u32 = re_special
        .find_iter(input)
        .flat_map(|map| {
            // dbg!(map);
            // dbg!(map.start(), map.end() + 1, map.start() - 11,);

            // println!(
            //     "{}{}{}\n{}{}{}\n{}{}{}\n",
            //     input.chars().nth(map.start() - line_size - 2).unwrap(),
            //     input.chars().nth(map.start() - line_size - 1).unwrap(),
            //     input.chars().nth(map.start() - line_size).unwrap(),
            //     input.chars().nth(map.start() - 1).unwrap(),
            //     input.chars().nth(map.start()).unwrap(),
            //     input.chars().nth(map.end()).unwrap(),
            //     input.chars().nth(map.end() + line_size - 1).unwrap(),
            //     input.chars().nth(map.end() + line_size).unwrap(),
            //     input.chars().nth(map.end() + line_size + 1).unwrap()
            // );
            numbers
                .iter()
                .filter_map(move |num| {
                    if (map.start() - line_size - 2 <= num.start()
                        && num.start() <= map.start() - line_size)
                        || (map.start() - line_size - 2 < num.end()
                            && num.end() - 1 <= map.start() - line_size)
                    {
                        println!("UPPER {}", num.as_str());
                        return Some(num.as_str().parse::<u32>().unwrap());
                    } else if num.end() == map.start() || num.start() == map.end() {
                        println!("SAME {}", num.as_str());
                        return Some(num.as_str().parse::<u32>().unwrap());
                    }
                    if (map.end() + line_size - 1 <= num.start()
                        && num.start() <= map.end() + line_size + 1)
                        || (map.end() + line_size - 1 < num.end()
                            && num.end() - 1 <= map.end() + line_size + 1)
                    {
                        println!("LOWER {}", num.as_str());
                        return Some(num.as_str().parse::<u32>().unwrap());
                    } else {
                        // println!("NOT FOUND {}", num.as_str());
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .sum();

    // v.sort();
    // dbg!(v);

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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
