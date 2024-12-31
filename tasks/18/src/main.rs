use std::fs;

static _INPUT_PATH: &str = "tasks/17/input.txt";

fn main() {
    // load file
    // as a string this time
    let input = fs::read_to_string(_INPUT_PATH).unwrap();

    let checksum = defragment(&input);

    println!("CHECKSUM {checksum}");

}

// DISCLAIMER
// not my code
// source: https://github.com/TcePrepK/AOC24_Rust
// my solutions kept returning wrong checksums (off by tens or hundreds)
// so I gave up on this one
// eventually it turned out I misunderstood the task in the first place
fn defragment(input: &str) -> i64 {
    // iterate over the input
    // and record empty spaces and files locations
    let mut empty_spaces: Vec<(usize, usize)> = vec![];
    let mut files: Vec<(usize, usize)> = vec![];

    let chars = input.chars().collect::<Vec<char>>();
    let mut last_index = 0;
    for (i, char) in chars.iter().enumerate() {
        let len = char.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            files.push((last_index, len));
        } else {
            empty_spaces.push((last_index, len));
        }
        last_index += len;
    }

    let mut end_files: Vec<(usize, usize, usize)> = vec![];

    // iterate over the files and try to find space for them
    while !files.is_empty() {
        let (start, length) = files.pop().unwrap();
        let index = files.len();

        let mut empty_index = empty_spaces.len();
        for i in 0..index {
            if empty_spaces[i].1 >= length {
                empty_index = i;
                break;
            }
        }

        if empty_index >= index {
            end_files.push((start, length, index));
            continue;
        }

        let empty_data = &mut empty_spaces[empty_index];

        end_files.push((empty_data.0, length, index));
        empty_data.0 += length;
        if empty_data.1 <= length {
            empty_data.1 = 0;
            continue;
        }
        empty_data.1 -= length;
    }

    // now we calculate the checksum from end files only
    let mut result: i64 = 0;
    for (start, length, index) in end_files {
        for i in start..start + length {
            result += (i as i64) * (index as i64);
        }
    }

    result
}