use std::collections::HashSet;

static _INPUT_PATH: &str = "tasks/23/input.txt";


// in theory we already have all the data we need
// so instead of counting, we will now record all fence
// locations in tuples for each direction for each new region
// when we have that complete, we will sort the vertical fences by Y primarily
// and horizontal fences by X primarily
// then, we look for continuation
// each bump is a side

#[derive(Debug)]
struct Fences {
    upwards: Vec<(usize, usize)>,
    downwards: Vec<(usize, usize)>,
    left: Vec<(usize, usize)>,
    right: Vec<(usize, usize)>,
}

impl Fences {
    fn new() -> Self {
        Fences {
            upwards: vec![],
            downwards: vec![],
            left: vec![],
            right: vec![],
        }
    }
}

fn main() {
    // recycle grid load
    let grid = t11::load_grid(_INPUT_PATH);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut cnt: usize = 0;

    for (row_ix, row) in grid.iter().enumerate() {
        for (col_ix, el) in row.iter().enumerate() {
            let mut fences = Fences::new();
            let area = recursive_flood(&grid, (row_ix, col_ix), el,
                &mut visited, &mut fences);
            let sides = get_sides(&mut fences);
            cnt += area * sides;
        }
    }
    // result
    println!("The result is {cnt}");
}

// cell = (row_ix, col_ix)
fn recursive_flood(grid: &Vec<Vec<String>>, cell: (usize, usize),
    letter: &String, visited: &mut HashSet<(usize, usize)>, fences: &mut Fences) -> usize {
    let mut area_cnt: usize = 0;

    if grid[cell.0][cell.1] != *letter || visited.contains(&cell) {
        return 0
    }
    // mark as visited
    visited.insert(cell);

    // incr area for current cell
    area_cnt += 1;

    // beginning of a line
    if cell.0 == 0 {
        fences.upwards.push(cell);
    }
    if cell.1 == 0 {
        fences.left.push(cell);
    }
    // end of a line
    if cell.0 == grid.len()-1 {
        fences.downwards.push(cell);
    }

    if cell.1 == grid[0].len()-1 {
        fences.right.push(cell);
    }
    // up
    if cell.0 != 0 {
        // peek
        if grid[cell.0-1][cell.1] != *letter {
            fences.upwards.push(cell);
        } else {
            let area_inc = recursive_flood(grid, (cell.0-1, cell.1),
                letter, visited, fences);
            area_cnt += area_inc;
        }
    }
    // down
    if cell.0 < grid.len()-1 {
        if grid[cell.0+1][cell.1] != *letter {
            fences.downwards.push(cell);
        } else {
            let area_inc = recursive_flood(grid, (cell.0+1, cell.1),
                letter, visited, fences);
            area_cnt += area_inc;
        }
    }
    // left
    if cell.1 != 0 {
        if grid[cell.0][cell.1-1] != *letter {
            fences.left.push(cell);
        } else {
            let area_inc = recursive_flood(grid, (cell.0, cell.1-1),
                letter, visited, fences);
            area_cnt += area_inc;
        }
    }
    // right
    if cell.1 < grid[0].len()-1 {
        if grid[cell.0][cell.1+1] != *letter {
            fences.right.push(cell);
        } else {
            let area_inc = recursive_flood(grid, (cell.0, cell.1+1),
                letter, visited, fences);
            area_cnt += area_inc;
        }
    }

    return area_cnt
}


fn count_vertical_sides(vertical_fences: &mut Vec<(usize, usize)>) -> usize {
    // sort vertical fences into columns
    vertical_fences.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    // count vert sides
    let mut sides = 0;
    for (i, &(y, x)) in vertical_fences.iter().enumerate() {
        if i == 0 {
            // first fence is always new side
            sides += 1;
        } else {
            let (prev_y, prev_x) = vertical_fences[i - 1];
            let is_continuation_of_side = prev_x == x && prev_y == y - 1;
            if !is_continuation_of_side {
                sides += 1;
            }
        }
    }

    sides
}

fn count_horizontal_sides(horizontal_sides: &mut Vec<(usize, usize)>) -> usize {
    // get rows
    horizontal_sides.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let mut sides = 0;
    for (i, &(y, x)) in horizontal_sides.iter().enumerate() {
        if i == 0 {
            // first => new side again
            sides += 1;
        } else {
            let (prev_y, prev_x) = horizontal_sides[i - 1];
            let is_continuation_of_side = prev_y == y && prev_x == x - 1;
            if !is_continuation_of_side {
                sides += 1;
            }
        }
    }

    sides
}

fn get_sides(fences: &mut Fences) -> usize {
    let top = count_horizontal_sides(&mut fences.upwards);
    let bottom = count_horizontal_sides(&mut fences.downwards);
    let left = count_vertical_sides(&mut fences.left);
    let right = count_vertical_sides(&mut fences.right);

    top + bottom + left + right
}
