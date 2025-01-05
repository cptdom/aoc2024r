use std::fs;

pub fn load_input(path: &str) -> Vec<[isize; 4]> {
    let mut input: Vec<[isize; 4]> = vec![];
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .for_each(|line| {

            let parts: Vec<&str> = line.split_whitespace().collect();

            let p_values: Vec<isize> = parts[0][2..]
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();

            let v_values: Vec<isize> = parts[1][2..]
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();

            input.push([p_values[0], p_values[1], v_values[0], v_values[1]]);
        });


    input
}