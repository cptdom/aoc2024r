use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn parse_input(path: &str) -> Vec<(usize, Vec<usize>)> {
    let reader = BufReader::new(File::open(path).unwrap());
    reader.lines()
        .map(|line| {
            let line_split: Vec<String> = line.unwrap().split(":")
                .map(|c| c.to_string()).collect();
            let vectors: Vec<usize> = line_split[1].trim().split_whitespace()
                .map(|c| c.parse::<usize>().unwrap()).collect();
            (line_split[0].parse::<usize>().unwrap(), vectors)
        }).collect()
}