use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};

static _INPUT_PATH: &str = "tasks/05/input.txt";
static _DO_KEY: &str = "do()";
static _DONT_KEY: &str = "don't()";
// no need for the third key if we use match case upon the capture

fn main() {
    let reader = BufReader::new(File::open(_INPUT_PATH).unwrap());

    let pattern = r"(do\(\))|(don't\(\))|mul\((\d+),(\d+)\)";
    let regex = Regex::new(pattern).unwrap();

    let mut total: usize = 0;

    let mut is_process = true;

    for line in reader.lines() {
        let line = line.unwrap();
        'inner:
        for found in regex.captures_iter(&line) {
            let captured_word: String = found[0].parse().unwrap();
            match captured_word {
                _ if captured_word.starts_with(_DO_KEY) => {
                    is_process = true;
                    continue 'inner;
                }
                _ if captured_word.starts_with(_DONT_KEY) => {
                    is_process = false;
                    continue 'inner;
                }
                _ => {
                    if is_process {
                        let n1: usize = found[3].parse().unwrap(); // 3 and 4 now because 1 and 2 are the do and don't
                        let n2: usize = found[4].parse().unwrap();
                        total += n1 * n2;
                    }
                }
            }
        }
    }
    println!("the total is {total}")
}