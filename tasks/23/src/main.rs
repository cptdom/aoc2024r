use std::collections::HashSet;

static _INPUT_PATH: &str = "tasks/23/input.txt";


// iterate over the row elements
// if element has been visited, continue
// else n each row elements, trigger a recursive flooding mechanism
// on each visit, mark the cell as visited
// on each visit, check the environment up, down, left and right and add fence accordingly
// when it all backtracks, multiply the count of the regions with the count of the fences
// sum it all up

fn main() {
    // recycle grid load
    let grid = t11::load_grid(_INPUT_PATH);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut cnt: usize = 0;

    for (row_ix, row) in grid.iter().enumerate() {
        for (col_ix, el) in row.iter().enumerate() {
            let (area, perimeter) = recursive_flood(&grid, (row_ix, col_ix), el, &mut visited);
            cnt += area * perimeter;
        }
    }
    // result
    println!("The result is {cnt}");
}

// cell = (row_ix, col_ix)
fn recursive_flood(grid: &Vec<Vec<String>>, cell: (usize, usize), letter: &String, visited: &mut HashSet<(usize, usize)>) -> (usize, usize) {
    let mut area_cnt: usize = 0;
    let mut peri_cnt: usize = 0;

    if grid[cell.0][cell.1] != *letter || visited.contains(&cell) {
        return (0,0)
    }
    // mark as visited
    visited.insert(cell);

    // incr area for current cell
    area_cnt += 1;
    // incr fence count for current cell
    // beginning of a line
    if cell.0 == 0 {
        peri_cnt += 1;
    }
    if cell.1 == 0 {
        peri_cnt += 1;
    }
    // end of a line
    if cell.0 == grid.len()-1 {
        peri_cnt += 1;
    }

    if cell.1 == grid[0].len()-1 {
        peri_cnt += 1;
    }
    // up
    if cell.0 != 0 {
        // peek
        if grid[cell.0-1][cell.1] != *letter {
            peri_cnt += 1;
        } else {
            let (area_inc, peri_inc) = recursive_flood(grid, (cell.0-1, cell.1), letter, visited);
            area_cnt += area_inc;
            peri_cnt += peri_inc;
        }
    }
    // down
    if cell.0 < grid.len()-1 {
        if grid[cell.0+1][cell.1] != *letter {
            peri_cnt += 1;
        } else {
            let (area_inc, peri_inc) = recursive_flood(grid, (cell.0+1, cell.1), letter, visited);
            area_cnt += area_inc;
            peri_cnt += peri_inc;
        }
    }
    // left
    if cell.1 != 0 {
        if grid[cell.0][cell.1-1] != *letter {
            peri_cnt += 1;
        } else {
            let (area_inc, peri_inc) = recursive_flood(grid, (cell.0, cell.1-1), letter, visited);
            area_cnt += area_inc;
            peri_cnt += peri_inc;
        }
    }
    // right
    if cell.1 < grid[0].len()-1 {
        if grid[cell.0][cell.1+1] != *letter {
            peri_cnt += 1;
        } else {
            let (area_inc, peri_inc) = recursive_flood(grid, (cell.0, cell.1+1), letter, visited);
            area_cnt += area_inc;
            peri_cnt += peri_inc;
        }
    }

    return (area_cnt, peri_cnt)
}