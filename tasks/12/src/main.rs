use std::collections::HashSet;
use t11::Direction;

// static _INPUT_PATH: &str = "tasks/11/input.txt";
static _INPUT_PATH: &str = "tasks/12/test.txt";



fn main() {
    let grid = t11::load_grid(_INPUT_PATH);
    // println!("{:?}", grid);
    let cnt = walk_the_line(&grid);

    println!("The count is {cnt}");
}

// TBD
fn walk_the_line(grid: &Vec<Vec<String>>) -> usize {
    0
}