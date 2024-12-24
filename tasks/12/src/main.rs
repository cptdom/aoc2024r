use std::collections::HashSet;
use t11::Direction;

static _INPUT_PATH: &str = "tasks/11/input.txt";
// static _INPUT_PATH: &str = "tasks/12/test.txt";

// WARNING: this brute force approach takes about a minute or two to finish
fn main() {
    let grid = t11::load_grid(_INPUT_PATH);
    let mut iter = 0;
    let mut cnt = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if cell != "." {
                continue;
            }

            let mut grid_with_obstacle = grid.clone();
            grid_with_obstacle[row_idx][col_idx] = "#".to_string();

            println!("Iter {iter}");
            if walk_the_line(&grid_with_obstacle) {
                cnt += 1;
            }
            iter += 1;
        }
    }

    println!("The count is {cnt}");
}

// we reuse the code from the first half
// and use brute force - on each empty field in the grid (excluding starting position)
// we will insert an obstacle.
// then we will run the walking simulation for that particular change
// and upon each step, look for a loop
// re-visiting the same cell in the same direction = endless loop -> return true
// finishing the simulation and going out of bounds = nope, next -> return false
// we will count the number of occurences of true
fn walk_the_line(grid: &Vec<Vec<String>>) -> bool {
    let mut visited: HashSet<((isize, isize), Direction)> = HashSet::new();
    // find current position
    // assert its direction
    let (mut start, mut start_dir) = t11::find_start(grid).clone();
    // move until obstacle or out of bounds
    //
    loop {
        visited.insert((start, start_dir.clone()));
        let mut move_to: (isize, isize) = (0,0);
        match start_dir {
            Direction::Up => {move_to = (start.0-1, start.1)}
            Direction::Down => {move_to = (start.0+1, start.1)}
            Direction::Left => {move_to = (start.0, start.1-1)}
            Direction::Right => {move_to = (start.0, start.1+1)}
        }
        if move_to.1 < 0 || move_to.1 >= grid.len() as isize {
            return false
        }
        if move_to.0 < 0 || move_to.0 >= grid[0].len() as isize {
            return false
        }
        if visited.contains(&(move_to, start_dir.clone())) {
            // same place, same direction = endless loop
            return true
        }
        if grid[move_to.0 as usize][move_to.1 as usize] == "#" {
            start_dir.turn();
            // we do not advance into the obstacle
            continue
        }
        start = move_to;
    }

}


// THIS GETS OOM KILLED

// fn main() {
//     let grid = t11::load_grid(_INPUT_PATH);
//     // println!("{:?}", grid);
//     let alternate_grids = generate_alternate_grids(grid);
//     let cnt: usize = alternate_grids
//         .iter()
//         .filter(|grid| walk_the_line(grid))
//         .count();
//     println!("The count is {cnt}");
// }


// fn generate_alternate_grids(
//     grid: Vec<Vec<String>>,
// ) -> Vec<Vec<Vec<String>>> {
//     let mut alternate_grids = Vec::new();

//     for (row_idx, row) in grid.iter().enumerate() {
//         for (col_idx, cell) in row.iter().enumerate() {
//             // skip starting position or non-empty cells
//             if cell != "." {
//                 continue;
//             }

//             // Create a new grid with the obstacle added
//             let mut new_grid = grid.clone();
//             new_grid[row_idx][col_idx] = "#".to_string();

//             // Add the new grid to the list of alternate grids
//             alternate_grids.push(new_grid);
//         }
//     }

//     alternate_grids
// }