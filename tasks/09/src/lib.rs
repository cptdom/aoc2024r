use std::fs::File;
use std::io::{BufReader, BufRead};

use std::collections::HashSet;


pub fn parse_file(file_path: &str) -> (HashSet<(usize, usize)>, Vec<Vec<usize>>) {
    let lines: Vec<String> = BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let rules: HashSet<(usize, usize)> = lines.iter()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            line.split_once('|')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        })
        .collect();
    let updates = lines.iter()
        .skip(rules.len() + 1)
        .map(|update| {
            update
                .split(',')
                .filter_map(|num| num.parse().ok())
                .collect()
        })
        .collect();
    (rules, updates)
}
