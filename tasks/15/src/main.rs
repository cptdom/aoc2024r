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

        for (i, &(x1, y1)) in coord_vec.iter().enumerate() {
            for &(x2, y2) in coord_vec.iter().skip(i + 1) {
                let (x1, y1, x2, y2) = if (x1, y1) < (x2, y2) {
                    (x1, y1, x2, y2)
                } else {
                    (x2, y2, x1, y1)
                };

                let dx = *x2 as isize - *x1 as isize;
                let dy = *y2 as isize - *y1 as isize;

                let antinode1 = (*x1 as isize - dx, *y1 as isize - dy);
                let antinode2 = (*x2 as isize + dx, *y2 as isize + dy);


                if antinode1.0 >= 0 && antinode1.1 >= 0
                    && antinode1.0 < grid[0].len() as isize
                    && antinode1.1 < grid.len() as isize
                {
                    antinodes_coords.insert((antinode1.0 as usize, antinode1.1 as usize));
                }
                if antinode2.0 >= 0 && antinode2.1 >= 0
                    && antinode2.0 < grid[0].len() as isize
                    && antinode2.1 < grid.len() as isize
                {
                    antinodes_coords.insert((antinode2.0 as usize, antinode2.1 as usize));
                }
            }
        }
    }

    antinodes_coords.len()
}

