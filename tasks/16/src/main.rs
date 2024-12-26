use std::collections::{HashSet, HashMap};

static _INPUT_PATH: &str = "tasks/15/input.txt";

fn main() {
    let grid = t11::load_grid(_INPUT_PATH);
    let cnt = find_antinodes(&grid);

    println!("Total count is {cnt}");
}


fn find_antinodes(grid: &Vec<Vec<String>>) -> usize {
    let mut antinodes_coords: HashSet<(usize, usize)> = HashSet::new();
    let mut char_map: HashMap<String, HashSet<(usize, usize)>> = HashMap::new();

    for row in grid.iter() {
        for col in row.iter() {
            if col != "." {
                char_map.entry(col.clone()).or_insert_with(|| t15::find_string_coords(col, grid));
            }
        }
    }

    for (_, coords) in &char_map {
        let coord_vec: Vec<&(usize, usize)> = coords.iter().collect();

        for &(x, y) in coord_vec.iter() {
            antinodes_coords.insert((*x, *y));
        }

        // I assume there's some super efficient library that does this 1000 times faster
        // but that's not the point
        for (i, &(x1, y1)) in coord_vec.iter().enumerate() {
            for &(x2, y2) in coord_vec.iter().skip(i + 1) {
                let dx = *x2 as isize - *x1 as isize;
                let dy = *y2 as isize - *y1 as isize;

                // direction vector normalization
                let gcd = gcd(dx.abs(), dy.abs());
                let step_x = dx / gcd;
                let step_y = dy / gcd;

                let mut current_x = *x1 as isize;
                let mut current_y = *y1 as isize;

                while current_x >= 0
                    && current_y >= 0
                    && current_x < grid[0].len() as isize
                    && current_y < grid.len() as isize
                {
                    antinodes_coords.insert((current_x as usize, current_y as usize));
                    current_x -= step_x;
                    current_y -= step_y;
                }

                current_x = *x2 as isize;
                current_y = *y2 as isize;

                while current_x >= 0
                    && current_y >= 0
                    && current_x < grid[0].len() as isize
                    && current_y < grid.len() as isize
                {
                    antinodes_coords.insert((current_x as usize, current_y as usize));
                    current_x += step_x;
                    current_y += step_y;
                }
            }
        }
    }

    antinodes_coords.len()
}

// greatest common divisor
fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
