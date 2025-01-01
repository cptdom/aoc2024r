use std::fs;

pub fn load_vec(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>()
            .unwrap())
        .collect()
}