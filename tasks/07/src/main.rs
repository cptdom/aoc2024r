use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;

static _INPUT_PATH: &str = "tasks/07/input.txt";

fn main() {
    // the idea is to shift the input and do a simple pattern search on a row
    // 1. get rows as they are and look for "XMAS" and "SMAX"
    // 2. get columns as rows and do the same
    // 3. shift left-to-right directional diagonals and do the same
    // 4. shift right-to-left diagonals and do the same
    // 5. sum them up
    // 6. profit
    let reader = BufReader::new(File::open(_INPUT_PATH).unwrap());
    let patterns = vec!["XMAS", "SAMX"];
    let mut total: usize = 0;

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // iterating over patterns in the outer loop will make us shift the input twice per each check
    // but screw that for now
    for pattern in patterns {
        let regex = Regex::new(pattern).unwrap();
        // VERTICALLY
        for line in &lines {
            total += regex.captures_iter(&line).count();
        }
        // HORIZONTALLY
        for line in transpose(&lines) {
            total += regex.captures_iter(&line).count();
        }
        // SHIFT LEFT
        for line in left_diagonals_transpose(&lines) {
            total += regex.captures_iter(&line).count();
        }
        // SHIFT RIGHT
        for line in right_diagonals_transpose(&lines) {
            total += regex.captures_iter(&line).count();
        }
    }

    println!("Total count is {total}");


}

fn transpose(lines: &Vec<String>) -> Vec<String> {
    let rows = lines.len();
    let cols = lines[0].len(); // let's skip emptiness checks

    let mut transposed = vec![String::with_capacity(rows); cols];

    for row in lines {
        let row_vec = string_to_vec(&row);
        for (col_ix, letter) in row_vec.into_iter().enumerate() {
            transposed[col_ix].push_str(&letter);
        }
    }
    transposed
}

fn string_to_vec(input: &str) -> Vec<String> {
    input.chars().map(|c| c.to_string()).collect()
}

fn left_diagonals_transpose(lines: &Vec<String>) -> Vec<String> {
    let rows = lines.len();
    let cols = lines[0].len(); // let's skip emptiness checks

    let mut diagonals = vec![String::new(); rows+cols-1];

    for (row_ix, row) in lines.iter().enumerate() {
        for (col_ix, letter) in row.chars().enumerate() {
            let diag_ix = row_ix+col_ix;
            diagonals[diag_ix].push(letter);
        }
    }
    diagonals
}

fn right_diagonals_transpose(lines: &Vec<String>) -> Vec<String> {
    let rows = lines.len();
    let cols = lines[0].len(); // let's skip emptiness checks

    let mut diagonals = vec![String::new(); rows+cols-1];

    for (row_ix, row) in lines.iter().enumerate() {
        for (col_ix, letter) in row.chars().enumerate() {
            let diag_ix = row_ix + (cols -1 - col_ix);
            diagonals[diag_ix].push(letter);
        }
    }

    diagonals
}