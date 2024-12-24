use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use once_cell::sync::Lazy;


#[derive(Clone,Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
            Direction::Right => *self = Direction::Down,
        }
    }
}

pub static DIR_MAP: Lazy<HashMap<String, Direction>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("^".to_string(), Direction::Up);
    map.insert("v".to_string(), Direction::Down);
    map.insert("<".to_string(), Direction::Left);
    map.insert(">".to_string(), Direction::Right);
    map
});

pub fn load_grid(path: &str) -> Vec<Vec<String>> {
    let reader = BufReader::new(File::open(path).unwrap());
    reader.lines()
    .map(|line| line.unwrap().chars().map(|c| c.to_string()).collect())
        .collect()
}
pub fn find_start(grid: &Vec<Vec<String>>) -> ((isize, isize), Direction) {
    let mut start: (isize, isize) = (0,0);
    let mut start_dir: Direction = Direction::Up;
    'search:
    for row_ix in 0..grid.len() as isize {
        for col_ix in 0..grid[0].len() as isize {

            if let Some(dir) = DIR_MAP.get(&grid[row_ix as usize][col_ix as usize]) {
                start = (row_ix, col_ix);
                start_dir = dir.clone();
                break 'search
            }
        }
    }
    (start, start_dir)
}
