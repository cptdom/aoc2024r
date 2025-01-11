
use std::fs;

pub fn load_input(path: &str) -> (Vec<Vec<String>>, Vec<String>) {
    let input = fs::read_to_string(path).unwrap();

    let parts: Vec<&str> = input.split("\n\n").collect();

    let grid = parts[0]
        .lines()
        .map(|line| line
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();


        let instructions = parts[1]
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

    (grid, instructions)
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            ">" => Self::Right,
            "<" => Self::Left,
            "^" => Self::Up,
            "v" => Self::Down,
            _ => todo!(),
        }
    }
    fn as_vector_tuple(&self) -> (isize, isize) {
        match self {
            Self::Down => (1,0),
            Self::Up => (-1,0),
            Self::Left => (0,-1),
            Self::Right => (0,1),
        }
    }
}

pub fn process_instructions(grid: &mut Vec<Vec<String>>, instructions: Vec<String>) -> &mut Vec<Vec<String>> {
    let mut curr_pos = find_starting_position(grid);
    for ins in instructions {
        let dir = Direction::from_str(&ins);
        curr_pos = make_move(curr_pos, dir, grid);
    }
    grid
}

fn make_move(pos: (isize, isize), direction: Direction, grid: &mut Vec<Vec<String>>) -> (isize, isize) {

    let vector = direction.as_vector_tuple();

    let mut curr_pos = pos.clone();
    let mut mass: usize = 0;
    let mut steps: usize = 0;

    'stepping:
    loop {
        match grid[(curr_pos.0 + vector.0) as usize][(curr_pos.1 + vector.1) as usize].as_str() {
            "O" => {
                mass += 1;
                curr_pos = (curr_pos.0+vector.0, curr_pos.1+vector.1);
                if grid[curr_pos.0 as usize][curr_pos.1 as usize] == "O" {
                    grid[curr_pos.0 as usize][curr_pos.1 as usize] = ".".to_string();
                }
                continue
            }
            "." => {
                steps += 1;
                if grid[curr_pos.0 as usize][curr_pos.1 as usize] == "O" {
                    grid[curr_pos.0 as usize][curr_pos.1 as usize] = ".".to_string();
                }
                break 'stepping
            }
            "#" => {
                break 'stepping
            }
            _ => todo!()
        }
    }

    // set current position
    // to new position according to steps made
    let new_pos = (pos.0+vector.0*steps as isize, pos.1+vector.1*steps as isize);


    for i in 1..mass+1 {
        let i = i as isize;
        let new_row_ix = (new_pos.0+vector.0*i) as usize;
        let new_col_ix = (new_pos.1+vector.1*i) as usize;
        grid[new_row_ix][new_col_ix] = "O".to_string();
    }

    new_pos

}

// pos = (row_ix, col_ix) = (y, x) in human
fn find_starting_position(grid: &mut Vec<Vec<String>>) -> (isize, isize) {
    for (row_ix, row) in grid.iter().enumerate() {
        for (col_ix, val) in row.iter().enumerate() {
            if val == "@" {
                grid[row_ix][col_ix] = ".".to_string();
                return (row_ix as isize, col_ix as isize)
            }
        }
    }
    (0,0)
}

