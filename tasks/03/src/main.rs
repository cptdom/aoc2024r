use std::fs::File;
use std::io::{BufReader, BufRead};

static _INPUT_PATH: &str = "tasks/03/input.txt";

fn main() {
    let reader = BufReader::new(File::open(_INPUT_PATH).unwrap());
    let mut total_ok: usize = 0;
    for line in reader.lines() {
        // get rid of the result
        let line = line.unwrap();
        if t03::check_array_safe(line) {
            total_ok += 1;
        }

    }
    println!("Total count is {total_ok}");
}

