use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};

static _INPUT_PATH: &str = "tasks/05/input.txt";

fn main() {
    let reader = BufReader::new(File::open(_INPUT_PATH).unwrap());

    let pattern = r"mul\((\d+),(\d+)\)";
    let regex = Regex::new(pattern).unwrap();

    let mut total: usize = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        for found in regex.captures_iter(&line) {
            let n1: usize = found[1].parse().unwrap();
            let n2: usize = found[2].parse().unwrap();
            total += n1 * n2;
        }
    }

    println!("the total is {total}")
}