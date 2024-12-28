use std::io::{BufRead, BufReader};
use std::fs::File;

static _INPUT_PATH: &str = "tasks/17/input.txt";

fn main() {
    // load file
    // single vector of usize
    let input: Vec<usize> = BufReader::new(File::open(_INPUT_PATH).unwrap())
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as usize)
                .collect::<Vec<usize>>()
        })
        .collect();

    // can be done in one iteration while going from both directions
    // and having 2 queues, one for whitespace indices, one for odd members
    // however, let's keep it readable

    // step 1 - transform input into human-readable disk space

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


    // step 2 - reposition accordingly
    // simple approach - 2 pointers
    let mut left = 0;
    let mut right = transformed_input.len()-1;

    while left < right {
        // negative number lookup from the left
        if transformed_input[left] < 0 && transformed_input[right] >= 0 {
            transformed_input.swap(left, right);
            left += 1;
            right -= 1;
        } else {
            // if left positive, move to the right
            if transformed_input[left] >= 0 {
                left += 1;
            }
            // if right negative, move to the left
            if transformed_input[right] < 0 {
                right -= 1;
            }
        }
    }
    let checksum: usize = transformed_input.iter()
        .filter(|&&a| a >= 0)
        .enumerate()
        .map(|(i, &n)| n as usize * i).sum();

    println!("CHECKSUM {checksum}");

}