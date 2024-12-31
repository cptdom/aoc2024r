
static _INPUT_PATH: &str = "tasks/19/input.txt";

fn main() {
    // get grid
    let grid = t19::load_grid(_INPUT_PATH);
    let mut total_score: usize = 0;
    // iterate over the grid
    // for each zero, trigger recursive lookup
    for (row_ix, row) in grid.iter().enumerate() {
        for (col_ix, &el) in row.iter().enumerate() {
            if el == 0 {
                total_score += gts(row_ix, col_ix, 0, &grid);
            }
        }
    }
    // profit
    println!("Total score sum: {total_score}");
}


fn gts(row_ix: usize, col_ix: usize, val: usize, grid: &Vec<Vec<usize>>) -> usize {
    // boundaries
    let height = grid.len();
    let width = grid[0].len();

    let mut trailhead_score: usize = 0;

    if grid[row_ix][col_ix] != val {
        return 0
    }
    if grid[row_ix][col_ix] == 9 && val == 9 {
        return 1
    }
    // look up
    if row_ix as isize - 1 >= 0 {
        trailhead_score += gts(row_ix-1, col_ix, val + 1, grid);
    }
    // look down
    if row_ix + 1 < height {
        trailhead_score += gts(row_ix+1, col_ix, val + 1, grid);
    }
    // look left
    if col_ix as isize -1 >= 0 {
        trailhead_score += gts(row_ix, col_ix-1, val + 1, grid);
    }
    // look right
    if col_ix + 1 < width {
        trailhead_score += gts(row_ix, col_ix+1, val + 1, grid);
    }
    // unmark this cell for other branches

    trailhead_score
}
