use std::cell::{Ref, RefCell};
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;
advent_of_code::solution!(10);

type Position = (usize, usize);
type Size = (usize, usize);

struct Map {
    nodes: Vec<Vec<Rc<RefCell<Node>>>>,
}

impl Map {
    fn new(nodes: Vec<Vec<Rc<RefCell<Node>>>>) -> Self {
        Self { nodes }
    }

    fn get(&self, pos: Position) -> Rc<RefCell<Node>> {
        self.nodes[pos.0][pos.1].clone()
    }

    fn get_size(&self) -> Size {
        (self.nodes.len(), self.nodes[0].len())
    }
}

impl fmt::Display for Map {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for v in self.nodes.clone() {
            for n in v {
                let n_burrow = n.borrow();
                if let Some(_) = n_burrow.visited_from {
                    if let Some(i) = char::from_digit(n_burrow.jump as u32, 10) {
                        s.push(i);
                    } else {
                        s.push('x');
                    }
                } else {
                    s.push(n_burrow.c);
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    N,
    S,
    W,
    E,
    X,
}

impl Direction {
    // pub fn reverse(&mut self) {
    //     *self = match *self {
    //         Direction::E => Direction::W,
    //         Direction::N => Direction::S,
    //         Direction::S => Direction::N,
    //         Direction::W => Direction::E,
    //     };
    // }

    pub fn get_reverse(&self) -> Direction {
        match self {
            Direction::E => Direction::W,
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            Direction::X => panic!(),
        }
    }
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

    pub fn has_east(&self) -> bool {
        match *self {
            PipeDirection::EW | PipeDirection::NE | PipeDirection::SE => true,
            _ => false,
        }
    }

    pub fn has_north(&self) -> bool {
        match *self {
            PipeDirection::NS | PipeDirection::NW | PipeDirection::NE => true,
            _ => false,
        }
    }

    pub fn has_south(&self) -> bool {
        match *self {
            PipeDirection::SW | PipeDirection::SE | PipeDirection::NS => true,
            _ => false,
        }
    }

    pub fn has_west(&self) -> bool {
        match *self {
            PipeDirection::EW | PipeDirection::SW | PipeDirection::NW => true,
            _ => false,
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
            visited_from: match c {
                'S' => Some(Direction::X),
                _ => None,
            },
            pipe_direction: PipeDirection::from(c).unwrap(),
            jump: 0,
        }
    }

    pub fn set_direction(&mut self, direction: Option<Direction>, jump: u64) {
        self.visited_from = direction;
        self.jump = jump + 1;
    }

    pub fn is_visited(&self) -> bool {
        match self.visited_from {
            Some(_) => true,
            None => false,
        }
    }

    pub fn is_accessible(&self, approache_from: &Direction) -> bool {
        match approache_from {
            Direction::N => self.pipe_direction.has_south(),
            Direction::S => self.pipe_direction.has_north(),
            Direction::W => self.pipe_direction.has_east(),
            Direction::E => self.pipe_direction.has_west(),
            Direction::X => false,
        }
    }

    pub fn get_next_position(&self) -> Option<Position> {
        let next_dir = self.get_next_direction();

        match next_dir {
            Some(dir) => match dir {
                Direction::E => Some((self.position.0, self.position.1 + 1)),
                Direction::N => Some((self.position.0 - 1, self.position.1)),
                Direction::S => Some((self.position.0 + 1, self.position.1)),
                Direction::W => Some((self.position.0, self.position.1 - 1)),
                Direction::X => None,
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
                    _ => None,
                },
                None => panic!(),
            },
            PipeDirection::EW => match self.visited_from {
                Some(d) => match d {
                    Direction::E => Some(Direction::W),
                    Direction::W => Some(Direction::E),
                    _ => None,
                },
                None => panic!(),
            },
            PipeDirection::NE => match self.visited_from {
                Some(d) => match d {
                    Direction::N => Some(Direction::E),
                    Direction::E => Some(Direction::N),
                    _ => None,
                },
                None => panic!(),
            },
            PipeDirection::NW => match self.visited_from {
                Some(d) => match d {
                    Direction::N => Some(Direction::W),
                    Direction::W => Some(Direction::N),
                    _ => None,
                },
                None => panic!(),
            },
            PipeDirection::SE => match self.visited_from {
                Some(d) => match d {
                    Direction::S => Some(Direction::E),
                    Direction::E => Some(Direction::S),
                    _ => None,
                },
                None => panic!(),
            },
            PipeDirection::SW => match self.visited_from {
                Some(d) => match d {
                    Direction::S => Some(Direction::W),
                    Direction::W => Some(Direction::S),
                    _ => None,
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

fn bfs(map: &mut Map, start: Position) -> u64 {
    let mut node_to_proc: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
    let map_size: Size = map.get_size();

    if (start.0 > 0)
        && map
            .get(((start.0 as i32 - 1) as usize, start.1))
            .borrow()
            .pipe_direction
            .has_south()
    {
        map.get(((start.0 as i32 - 1) as usize, start.1))
            .borrow_mut()
            .set_direction(Some(Direction::S), 0);
        node_to_proc.push_back(map.get(((start.0 as i32 - 1) as usize, start.1)));
        println!("Push 0 -1");
    }
    if (start.0 < map_size.0)
        && map
            .get(((start.0 as i32 + 1) as usize, start.1))
            .borrow()
            .pipe_direction
            .has_north()
    {
        map.get(((start.0 as i32 + 1) as usize, start.1))
            .borrow_mut()
            .set_direction(Some(Direction::N), 0);
        node_to_proc.push_back(map.get(((start.0 as i32 + 1) as usize, start.1)));
        println!("Push 0 + 1");
    }

    if (start.1 > 0)
        && map
            .get((start.0, (start.1 as i32 - 1) as usize))
            .borrow()
            .pipe_direction
            .has_east()
    {
        map.get((start.0, (start.1 as i32 - 1) as usize))
            .borrow_mut()
            .set_direction(Some(Direction::E), 0);
        node_to_proc.push_back(map.get((start.0, (start.1 as i32 - 1) as usize)));
        println!("Push 1 - 1");
    }
    if (start.1 < map_size.1)
        && map
            .get((start.0, (start.1 as i32 + 1) as usize))
            .borrow()
            .pipe_direction
            .has_west()
    {
        map.get((start.0, (start.1 as i32 + 1) as usize))
            .borrow_mut()
            .set_direction(Some(Direction::W), 0);
        node_to_proc.push_back(map.get((start.0, (start.1 as i32 + 1) as usize)));
        println!("Push 1 + 1");
    }
    let mut max_jump = 0;
    while !node_to_proc.is_empty() {
        // let n = dbg!(node_to_proc.pop_front().unwrap());
        let n = node_to_proc.pop_front().unwrap();
        {
            let n = n.borrow();
            if n.jump > max_jump {
                max_jump = n.jump;
            }
            push_next_node(n, &mut node_to_proc, map);
        }
    }
    // println!("{}", max_jump);
    return max_jump;
}

fn visite_next_node(n: Ref<Node>, map: &mut Map) -> Option<Rc<RefCell<Node>>> {
    if let Some(next_dir) = n.get_next_direction() {
        // let next_pos = dbg!(n.get_next_position());
        let next_pos = n.get_next_position();

        match next_pos {
            Some(pos) => {
                let next_node = map.get(pos);
                if !next_node.borrow().is_accessible(&next_dir) | next_node.borrow().is_visited() {
                    None
                } else {
                    next_node
                        .borrow_mut()
                        .set_direction(Some(next_dir.get_reverse()), n.jump);
                    Some(next_node)
                }
            }
            None => None,
        }
    } else {
        None
    }
}

fn push_next_node(n: Ref<Node>, queue: &mut VecDeque<Rc<RefCell<Node>>>, map: &mut Map) {
    if let Some(node) = visite_next_node(n, map) {
        queue.push_back(node)
    }
}

fn part_one(input: &str) -> Option<u64> {
    let mut start: Position = (0, 0);
    let mut map: Map = Map::new(
        input
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
            .collect(),
    );

    // println!("{}", map);
    let max = bfs(&mut map, start);
    // println!("{}", map);
    // dbg!(start);
    // dbg!(map);
    return Some(max);
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
        // assert_eq!(result, Some(4));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
