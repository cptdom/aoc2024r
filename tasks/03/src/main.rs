use std::fs::File;
use std::io::{BufReader, BufRead};

static _INPUT_PATH: &str = "tasks/03/input.txt";

fn main() {
    let reader = BufReader::new(File::open(_INPUT_PATH).unwrap());
    let mut total_ok: usize = 0;
    for line in reader.lines() {
        // get rid of the result
        let line = line.unwrap();
        let split_data = line.split_whitespace();
        let mut last_digit: usize = 0;
        let mut is_decreasing = false;
        let cnt = split_data.clone().count();
        for (i, digit) in split_data.enumerate() {
            let digit: usize = digit.parse().unwrap();
            if i == 0 {
                last_digit = digit;
                continue
            }
            if digit == last_digit {
                break
            }
            if i == 1 && digit < last_digit {
                is_decreasing = true;
            }
            if i > 1 && (digit < last_digit) != is_decreasing {
                break
            }
            let diff = last_digit.abs_diff(digit);
            if diff >= 1 && diff <= 3 {
                last_digit = digit;
                if i == cnt-1 {
                    total_ok += 1;
                }
                continue
            } else {
                break
            }
        }
    }
    println!("Total count is {total_ok}");
}

