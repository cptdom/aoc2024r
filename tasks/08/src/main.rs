use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;

static _INPUT_PATH: &str = "tasks/07/input.txt"; // "tasks/07/input.txt";

fn main() {
    let reader = BufReader::new(File::open(_INPUT_PATH).unwrap());
    let mut total: usize = 0;

    // get grid
    let grid: Vec<Vec<String>> = reader.lines()
        .map(| line| line.unwrap().chars()
            .map(|c| c.to_string()).collect()).collect();

    let grid_length = grid[0].len();
    let grid_height = grid.len();

    let patterns = vec![
        r"S.S.A.M.M",
        r"S.M.A.S.M",
        r"M.S.A.M.S",
        r"M.M.A.S.S",
    ];
    let regexes: Vec<Regex> = patterns.into_iter().map(|p| Regex::new(p).unwrap()).collect();

    for row_ix in 0..grid_length-2 {
        'column:
        for col_ix in 0..grid_height-2 {
            // now we get the window as a single line String
            let mut window = String::new();
            for i in 0..3 {
                let row = &grid[row_ix+i];
                for j in 0..3 {
                    window.push_str(&row[col_ix+j])
                }
            }
            // we perform the check
            for regex in &regexes {
                if regex.is_match(&window) {
                    total += 1;
                    continue 'column
                }
            }
        }
    }

    println!("Total count is {total}");
}

