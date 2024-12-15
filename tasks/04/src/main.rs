use std::fs::File;
use std::io::{BufReader, BufRead};


static _INPUT_PATH: &str = "tasks/04/input.txt";

fn main() {
    let reader = BufReader::new(File::open(_INPUT_PATH).unwrap());
    let mut total_ok: usize = 0;
    'outer:
    for line in reader.lines() {
        // get rid of the result
        let line = line.unwrap();
        // we just go brute force
        // the array either passes as a whole
        // or passes as one of its subsets where one of the indices is popped out
        if t03::check_array_safe(line.clone()) {
            total_ok+=1;
            continue 'outer
        }

        for variation in get_array_variations(line.clone()) {
            if t03::check_array_safe(variation) {
                total_ok+=1;
                continue 'outer
            }
        }
    }
    println!("Total count is {total_ok}");
}


fn get_array_variations(line: String) -> Vec<String> {
    let split_data = line.split_whitespace();
    let cnt = split_data.clone().count();

    let mut result = vec![];
    for ix in 0..cnt {
        let mut variation: Vec<usize> = vec![];
        for str_digit in split_data.clone().into_iter() {
            variation.push(str_digit.parse().unwrap());
        }
        variation.remove(ix);
        // an one-liner to convert a vector of values into a string separated by a single whitespace
        result.push(variation.iter()
                        .map(|num| num.to_string())
                        .collect::<Vec<_>>()
                        .join(" "))
    }
    result
}
