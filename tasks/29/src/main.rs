
static _INPUT_PATH: &str = "tasks/29/input.txt";

// battle plan: for each direction, implement a function/method for the grid to recursively shift
// the grid elements across X and Y axis
// then just move the target according to the instructions and


fn main() {
    let (mut grid, instructions) = t29::load_input(_INPUT_PATH);

    // apply instructions to the grid
    t29::process_instructions(&mut grid, instructions);

    println!("{:?}", grid);

    // calculate the result from the grid
    let score: usize = grid.iter().enumerate()
        .map(|(row_ix, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, val)| **val == "O")
                .map(|(col_ix, _)| 100 * row_ix + col_ix)
                .sum::<usize>()
        })
        .sum();

    println!("FINAL SCORE IS {score}")
}

