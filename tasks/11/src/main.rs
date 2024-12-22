use std::collections::HashSet;

use t11::Direction;


static _INPUT_PATH: &str = "tasks/11/input.txt";


fn main() {
    let grid = t11::load_grid(_INPUT_PATH);
    // println!("{:?}", grid);
    let cnt = walk_the_line(&grid);

    println!("The count is {cnt}");
}


fn walk_the_line(grid: &Vec<Vec<String>>) -> usize {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    // find current position
    // assert its direction
    let (mut start, mut start_dir) = t11::find_start(grid).clone();
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
            start_dir.turn();
            // we do not advance into the obstacle
            continue
        }
        start = move_to;
    }

    visited.len()
}