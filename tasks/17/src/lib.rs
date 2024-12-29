
use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn get_input(path: &str) -> Vec<usize> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as usize)
                .collect::<Vec<usize>>()
        })
        .collect()
}

pub fn transform_input(input: Vec<usize>) -> Vec<isize> {
    let mut transformed_input: Vec<isize> = vec![];
    let mut file_id = 0;

    for i in 0..input.len() {
        let length = input[i];
        let to_push = if i % 2 == 0 { file_id } else { -1 }; // file_id for files, -1 for spaces
        for _ in 0..length {
            transformed_input.push(to_push);
        }
        if i % 2 == 0 {
            file_id += 1;
        }
    }
    transformed_input
}

pub fn calculate_checksum(input: &Vec<isize>) -> usize {
    input.iter()
        .filter(|&&a| a >= 0)
        .enumerate()
        .map(|(i, &n)| n as usize * i).sum()
}