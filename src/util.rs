use std::env;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn get_lines() -> std::io::Lines<BufReader<File>> {
    let arg = env::args().nth(1).expect("Need file to read");
    let path = Path::new(&arg);
    let file = File::open(path).unwrap_or_else(|_| panic!("could not open {}", path.display()));
    BufReader::new(file).lines()
}

pub fn get_all_input() -> String {
    let arg = env::args().nth(1).expect("Need file to read");
    read_to_string(&arg).unwrap_or_else(|_| panic!("Could not read {}", arg))
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn neighbor(self, point: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => if point.0 == 0 { None } else { Some((point.0 - 1, point.1))},
            Direction::Down => Some((point.0 + 1, point.1)),
            Direction::Left => if point.1 == 0 { None } else { Some((point.0, point.1 - 1))},
            Direction::Right => Some((point.0, point.1 + 1)),
        }
    }
    pub fn immediate_neighbors(self) -> [Self; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        }
    }
    pub const fn directions() -> [Self; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}
