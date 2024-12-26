use std::collections::HashSet;

pub fn find_string_coords(x: &str, grid: &Vec<Vec<String>>) -> HashSet<(usize, usize)> {

    let mut result = HashSet::new();

    for (row_ix, row) in grid.iter().enumerate() {
        for (col_ix, el) in row.iter().enumerate() {
            if el == x {
                result.insert((col_ix, row_ix));
            }
        }
    }

    result
}