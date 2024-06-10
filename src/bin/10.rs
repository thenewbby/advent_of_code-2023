use std::cell::RefCell;
use std::rc::Rc;
advent_of_code::solution!(10);

type Position = (usize, usize);
type Size = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    N,
    S,
    W,
    E,
}

#[derive(Debug, Clone, Copy)]
pub enum PipeDirection {
    NS,    //= b'|',
    EW,    //= b'-',
    NE,    //= b'L',
    NW,    //= b'7',
    SE,    //= b'F',
    SW,    //= b'F',
    GRND,  //= b'.',
    START, //= b'S',
}

impl PipeDirection {
    fn from(s: char) -> Result<PipeDirection, ()> {
        match s {
            '|' => Ok(PipeDirection::NS),
            '-' => Ok(PipeDirection::EW),
            'L' => Ok(PipeDirection::NE),
            'J' => Ok(PipeDirection::NW),
            'F' => Ok(PipeDirection::SE),
            '7' => Ok(PipeDirection::SW),
            '.' => Ok(PipeDirection::GRND),
            'S' => Ok(PipeDirection::START),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    c: char,
    visited_from: Option<Direction>,
    pipe_direction: PipeDirection,
    position: Position,
    jump: u64,
}

impl Node {
    fn new(c: char, position: Position) -> Self {
        Self {
            c,
            position,
            visited_from: None,
            pipe_direction: PipeDirection::from(c).unwrap(),
            jump: 0,
        }
    }

    pub fn set_direction(&mut self, direction: Option<Direction>, jump: u64) {
        self.visited_from = direction;
        self.jump = jump + 1;
    }

    pub fn get_next_position(&self) -> Option<Position> {
        let next_dir = self.get_next_direction();

        match next_dir {
            Some(dir) => match dir {
                Direction::E => Some((self.position.0, self.position.1 + 1)),
                Direction::N => Some((self.position.0 - 1, self.position.1)),
                Direction::S => Some((self.position.0 + 1, self.position.1)),
                Direction::W => Some((self.position.0, self.position.1 - 1)),
            },
            None => None,
        }
    }

    pub fn get_next_direction(&self) -> Option<Direction> {
        match self.pipe_direction {
            PipeDirection::NS => match self.visited_from {
                Some(d) => match d {
                    Direction::S => Some(Direction::N),
                    Direction::N => Some(Direction::S),
                    _ => panic!(),
                },
                None => panic!(),
            },
            PipeDirection::EW => match self.visited_from {
                Some(d) => match d {
                    Direction::E => Some(Direction::W),
                    Direction::W => Some(Direction::E),
                    _ => panic!(),
                },
                None => panic!(),
            },
            PipeDirection::NE => match self.visited_from {
                Some(d) => match d {
                    Direction::N => Some(Direction::E),
                    Direction::E => Some(Direction::N),
                    _ => panic!(),
                },
                None => panic!(),
            },
            PipeDirection::NW => match self.visited_from {
                Some(d) => match d {
                    Direction::N => Some(Direction::W),
                    Direction::W => Some(Direction::N),
                    _ => panic!(),
                },
                None => panic!(),
            },
            PipeDirection::SE => match self.visited_from {
                Some(d) => match d {
                    Direction::S => Some(Direction::E),
                    Direction::E => Some(Direction::S),
                    _ => panic!(),
                },
                None => panic!(),
            },
            PipeDirection::SW => match self.visited_from {
                Some(d) => match d {
                    Direction::S => Some(Direction::W),
                    Direction::W => Some(Direction::S),
                    _ => panic!(),
                },
                None => panic!(),
            },
            PipeDirection::GRND => panic!(),
            PipeDirection::START => panic!(),
        }
    }
}

// fn get_neighbours(p: Position) -> impl Iterator<Item = Position> {
//     let m_range = if p.0 > 0 { p.0 - 1..p.0 + 2 } else { 0..2 };
//     let n_range = if p.1 > 0 { p.1 - 1..p.1 + 2 } else { 0..2 };
//     m_range
//         .flat_map(move |m| n_range.clone().map(move |n| (m, n)))
//         .filter(move |&q| p != q)
// }

fn bfs(map: &mut [Vec<Rc<RefCell<Node>>>], start: Position) -> usize {
    let mut node_to_proc: Vec<Rc<RefCell<Node>>> = Vec::new();
    let map_size: Size = (map.len(), map[0].len());

    if (start.0 > 0)
        & !matches!(
            map[(start.0 as i32 - 1) as usize][start.1]
                .borrow()
                .pipe_direction,
            PipeDirection::GRND
        )
    {
        map[(start.0 as i32 - 1) as usize][start.1]
            .borrow_mut()
            .set_direction(Some(Direction::S), 0);
        node_to_proc.push(map[(start.0 as i32 - 1) as usize][start.1].clone())
    }
    if (start.0 < map_size.0)
        & !matches!(
            &map[(start.0 as i32 + 1) as usize][start.1]
                .borrow()
                .pipe_direction,
            PipeDirection::GRND
        )
    {
        map[(start.0 as i32 + 1) as usize][start.1]
            .borrow_mut()
            .set_direction(Some(Direction::N), 0);
        node_to_proc.push(map[(start.0 as i32 + 1) as usize][start.1].clone())
    }

    if (start.1 > 0)
        & !matches!(
            &map[start.0][(start.1 as i32 - 1) as usize]
                .borrow()
                .pipe_direction,
            PipeDirection::GRND
        )
    {
        map[start.0][(start.1 as i32 - 1) as usize]
            .borrow_mut()
            .set_direction(Some(Direction::E), 0);
        node_to_proc.push(map[start.0][(start.1 as i32 - 1) as usize].clone())
    }
    if (start.1 < map_size.1)
        & !matches!(
            &map[start.0][(start.1 as i32 + 1) as usize]
                .borrow()
                .pipe_direction,
            PipeDirection::GRND
        )
    {
        map[start.0][(start.1 as i32 + 1) as usize]
            .borrow_mut()
            .set_direction(Some(Direction::W), 0);
        node_to_proc.push(map[start.0][(start.1 as i32 + 1) as usize].clone())
    }

    while !node_to_proc.is_empty() {
        let n = dbg!(node_to_proc.pop().unwrap());

        jump_to_node(n, map);
    }
    todo!()
}

fn jump_to_node(n: Rc<RefCell<Node>>, map: &mut [Vec<Rc<RefCell<Node>>>]) {
    let next_pos = dbg!(n.borrow().get_next_position());

    todo!()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut start: Position = (0, 0);
    let mut map: Vec<Vec<Rc<RefCell<Node>>>> = input
        .lines()
        .enumerate()
        .map(|(line_num, l)| {
            l.chars()
                .enumerate()
                .map(|(char_num, c)| {
                    if c == 'S' {
                        start = (line_num, char_num);
                    }
                    Rc::new(RefCell::new(Node::new(c, (line_num, char_num))))
                })
                .collect()
        })
        .collect();

    bfs(map.as_mut_slice(), start);
    // dbg!(start);
    // dbg!(map);
    todo!()
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
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
