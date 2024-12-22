use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::{HashMap, HashSet};

use once_cell::sync::Lazy;

static _INPUT_PATH: &str = "tasks/11/input.txt";

#[derive(Clone,Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

static DIR_MAP: Lazy<HashMap<String, Direction>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("^".to_string(), Direction::Up);
    map.insert("v".to_string(), Direction::Down);
    map.insert("<".to_string(), Direction::Left);
    map.insert(">".to_string(), Direction::Right);
    map
});



fn main() {
    let grid = load_grid(_INPUT_PATH);
    // println!("{:?}", grid);
    let cnt = walk_the_line(&grid);

    println!("The count is {cnt}");
}

fn load_grid(path: &str) -> Vec<Vec<String>> {
    let reader = BufReader::new(File::open(path).unwrap());
    reader.lines()
    .map(|line| line.unwrap().chars().map(|c| c.to_string()).collect())
        .collect()
}

fn walk_the_line(grid: &Vec<Vec<String>>) -> usize {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    // find current position
    // assert its direction
    let (mut start, mut start_dir) = find_start(grid).clone();
    // move until obstacle or out of bounds
    //
    'walking:
    loop {
        visited.insert(start);
        let mut move_to: (isize, isize) = (0,0);
        match start_dir {
            Direction::Up => {move_to = (start.0-1, start.1)}
            Direction::Down => {move_to = (start.0+1, start.1)}
            Direction::Left => {move_to = (start.0, start.1-1)}
            Direction::Right => {move_to = (start.0, start.1+1)}
        }
        if move_to.1 < 0 || move_to.1 >= grid.len() as isize {
            break 'walking;
        }
        if move_to.0 < 0 || move_to.0 >= grid[0].len() as isize {
            break 'walking;
        }
        if grid[move_to.0 as usize][move_to.1 as usize] == "#" {
            start_dir = change_dir(start_dir);
            // we do not advance into the obstacle
            continue
        }
        start = move_to;
    }

    visited.len()
}

fn find_start(grid: &Vec<Vec<String>>) -> ((isize, isize), Direction) {
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

fn change_dir(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}