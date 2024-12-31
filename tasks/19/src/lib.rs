use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn load_grid(path: &str) -> Vec<Vec<usize>> {
    let reader = BufReader::new(File::open(path).unwrap());
    reader.lines()
        .map(|line|
            line.unwrap()
                .chars()
                .map(|n| n.to_digit(10).unwrap() as usize)
                .collect()
            )
        .collect()
}