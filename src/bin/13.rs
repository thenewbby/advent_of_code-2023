advent_of_code::solution!(13);

#[derive(Debug, Clone)]
struct Map {
    matrix: Vec<Vec<char>>,
}

#[derive(Debug)]
enum Symetry {
    HORIZONTAL(usize),
    VERTICAL(usize),
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn find_line_symetry(line: &Vec<char>, sym_pos: Vec<usize>) -> Vec<usize> {
    let mut sym: Vec<usize> = Vec::new();
    for pos in sym_pos {
        let (start, end) = line.split_at(pos);

        // let mut end = end.to_string();
        // let mut start = start.to_string();

        // if end.len() > start.len() {
        //     end = end.chars().take(start.len()).collect();
        // } else if end.len() < start.len() {
        //     // take n laste
        //     start.drain(0..start.len() - end.len());
        // }
        // end = end.chars().rev().collect::<String>();

        let mut end = end.to_vec();
        let mut start = start.to_vec();

        if end.len() > start.len() {
            end = end[0..start.len()].to_vec();
        } else if end.len() < start.len() {
            // take n laste
            start = start[start.len() - end.len()..start.len()].to_vec();
        }
        end.reverse();
        if start == end {
            // println!("{}", pos);
            sym.push(pos);
        }
        // dbg!(start, end);
    }

    sym
}

fn find_symetry_pos(matrix: Vec<Vec<char>>) -> Option<usize> {
    let mut sym_pos = Vec::new();
    for i in 1..matrix[0].len() {
        sym_pos.push(i);
    }
    for line in matrix.iter() {
        let sym = find_line_symetry(line, sym_pos.clone());
        if sym.is_empty() {
            return None;
        }
        sym_pos = sym;
    }
    assert!(sym_pos.len() == 1);
    return Some(sym_pos[0]);
}

impl Map {
    fn new(matrix: Vec<Vec<char>>) -> Self {
        Self { matrix }
    }

    fn parse(str_map: &str) -> Self {
        let matrix: Vec<Vec<char>> = str_map.lines().map(|l| l.chars().collect()).collect();
        Self { matrix }
    }

    fn find_symetry(&self) -> Symetry {
        if let Some(pos) = find_symetry_pos(self.matrix.clone()) {
            return Symetry::VERTICAL(pos);
        } else {
            if let Some(pos) = find_symetry_pos(transpose(self.matrix.clone())) {
                return Symetry::HORIZONTAL(pos);
            }
        }
        panic!();
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let maps: Vec<_> = input.split("\n\n").map(Map::parse).collect();

    let v = maps.into_iter().fold(0, |acc, m| match m.find_symetry() {
        Symetry::HORIZONTAL(p) => acc + p * 100,
        Symetry::VERTICAL(p) => acc + p,
    });
    Some(v as u64)
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
