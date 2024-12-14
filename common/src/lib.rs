use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn parse_two_lists_input(path: &str) -> Result<(Vec<usize>, Vec<usize>), Box<dyn Error>> {
    let reader = BufReader::new(File::open(path)?);
    let (mut left_col, mut right_col): (Vec<usize>, Vec<usize>) = (vec![], vec![]);
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