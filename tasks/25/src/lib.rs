
use std::fs;

pub fn load_input(path: &str) -> Vec<Vec<f64>> {
    let mut inputs: Vec<Vec<f64>> = vec![];
    let input = fs::read_to_string(path).unwrap();
    for block in input.split("\n\n") {
        let curr_block: Vec<f64> = block
            .lines()
            .map(|line| {
                line.split(": ")
                    .nth(1)
                    .unwrap()
                    .split(", ")
                    .map(|s| s.split_at(2).1)
                    .map(|s| s.parse::<f64>().unwrap())
                    .collect::<Vec<f64>>()
            })
            .flatten()
            .collect();
        inputs.push(curr_block.try_into().unwrap());

    }
    inputs
}