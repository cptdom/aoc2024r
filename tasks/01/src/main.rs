use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};

static _INPUT_PATH: &str = "tasks/01/input.txt";

fn main() {
    if let Ok((mut left_col, mut right_col)) = parse_input(_INPUT_PATH) {
        // assuming both columns have the same lengths
        let mut total_distance = 0;
        // sort both in place since smallest number is compared to the smallest in
        // the other column
        left_col.sort();
        right_col.sort();
        for (left_num, right_num) in left_col.iter().zip(right_col.iter()) {
            total_distance += left_num.abs_diff(*right_num);
        }
        println!("The total distance is {total_distance}");
    } else {
        println!("input parsing failed miserably")
    }
}


fn parse_input(path: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn Error>> {
    let reader = BufReader::new(fs::File::open(path)?);
    let (mut left_col, mut right_col): (Vec<u32>, Vec<u32>) = (vec![], vec![]);
    for line in reader.lines() {
        let line = line?;
        let mut line_data = line.split_whitespace();
        if let (Some(left), Some(right)) = (line_data.next(), line_data.next()) {
            left_col.push(left.parse()?);
            right_col.push(right.parse()?);
        }
    }

    Ok((left_col, right_col))
}