use std::fs::File;
use std::io::{BufReader, BufRead};

use std::collections::HashSet;


static _INPUT_PATH: &str = "tasks/09/input.txt";

fn main() {
    let (pairs, vectors) = parse_file(_INPUT_PATH);

    let total = sum_ordered(pairs, vectors);
    println!("The total is {total}");
}

fn sum_ordered(rules: HashSet<(usize, usize)>, orders: Vec<Vec<usize>>) -> usize {
    orders.iter()
        .filter_map(|update| {
            // check if the order satisfies the rules
            if update.windows(2).all(|w| !rules.contains(&(w[1], w[0]))) {
                // if it does, return the middle element
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum() // and sum them all up
}


fn parse_file(file_path: &str) -> (HashSet<(usize, usize)>, Vec<Vec<usize>>) {
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
