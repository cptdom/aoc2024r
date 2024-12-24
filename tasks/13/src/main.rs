use std::io::{BufRead, BufReader};
use std::fs::File;

static _INPUT_PATH: &str = "tasks/13/input.txt";

fn main() {
    let input_lines = parse_input(_INPUT_PATH);
    let sum = get_valid_sum(input_lines);
    println!("The sum is {}", sum);
}


fn parse_input(path: &str) -> Vec<(usize, Vec<usize>)> {
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

fn get_valid_sum(input: Vec<(usize, Vec<usize>)>) -> usize {
    input.iter().filter(|&eq|
        is_valid_equation(eq.clone()))
        .map(|eq| eq.0).sum()
}

fn is_valid_equation(members: (usize, Vec<usize>)) -> bool {
    let (target, numbers) = members;
    evaluate(target, &numbers[1..], numbers[0])
}

// recursive function
// we will recursively add and multiply and try to find the correct result
fn evaluate(target: usize, numbers: &[usize], current_value: usize) -> bool {
    // base case
    if numbers.is_empty() {
        return current_value == target;
    }

    let next = numbers[0];
    let remaining = &numbers[1..];

    // try addition
    if evaluate(target, remaining, current_value + next) {
        return true;
    }

    // try multiplication
    if evaluate(target, remaining, current_value * next) {
        return true;
    }

    false
}