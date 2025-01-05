use std::collections::HashSet;

static _INPUT_PATH: &str = "tasks/27/input.txt";

const SECONDS: isize = 10000;

fn main() {
    let input = t27::load_input(_INPUT_PATH);

    let w: isize = 101;
    let h: isize = 103;

    // I have no idea what I'm looking for
    'lookup:
    for second in 1..SECONDS + 1 {
        // we initialize a set with coords of each robot
        let coords = input.iter()
            .map(|robot| {
                let final_x = (robot[0] + second * robot[2]).rem_euclid(w); // such wow
                let final_y = (robot[1] + second * robot[3]).rem_euclid(h);
                (final_x, final_y)
            })
            .collect::<HashSet<_>>();

        // now we have coords of all robots in a given frame
        // so we look for 10 consecutive robots vertically or horizontally
        // NOTE: THIS FAILED MISERABLY
        // if has_consecutive(coords, 10) {
        //     draw(w as usize, h as usize, coords);
        //     println!("FOUND IN {second} SECONDS");
        //     break 'lookup
        // }
        // we check if every robot is in a unique position in the frame
        // NOTE: WTF IT WORKS, must be a coincidence
        if is_all_unique(coords, input.len()) {
            // draw(w as usize, h as usize, coords);
            println!("FOUND IN {second} SECONDS");
            break 'lookup
        }
    }
}

fn is_all_unique(coords: HashSet<(isize, isize)>, n_robots: usize) -> bool {
    coords.len() == n_robots
}

fn has_consecutive(coords: HashSet<(isize, isize)>, n: usize) -> bool {
    // vertical check
    let mut coords_vertical: Vec<(isize, isize)> = coords.clone().into_iter().collect();
    coords_vertical.sort_by(|x, y| {
        if x.0 == y.0 {
            x.1.cmp(&y.1)
        } else {
            x.0.cmp(&y.0)
        }
    });

    let mut coords_horizontal: Vec<(isize, isize)> = coords.clone().into_iter().collect();
    coords_horizontal.sort_by(|x, y| {
        if x.1 == y.1 {
            x.0.cmp(&y.0)
        } else {
            x.1.cmp(&y.1)
        }
    });

    let mut cnt: usize = 0;
    for (i, &(x, _)) in coords_vertical.iter().enumerate() {
        if i == 0 {
            cnt += 1;
            continue
        }
        if x == coords_vertical[i-1].0 {
            cnt += 1;
            if cnt == n {
                println!("VERTICAL COORDS: {:?}, N {x}",coords_vertical);
                return true
            }
        } else {
            cnt = 0;
        }
    }

    let mut cnt: usize = 0;
    for (i, &(_, y)) in coords_horizontal.iter().enumerate() {
        if i == 0 {
            cnt += 1;
            continue
        }
        if y == coords_horizontal[i-1].1 {
            cnt += 1;
            if cnt == n {
                println!("HORIZONTAL COORDS: {:?}, N {y}",coords_horizontal);
                return true
            }
        } else {
            cnt = 0;
        }
    }

    false
}


fn draw(width: usize, height: usize, coords: Vec<(isize, isize)>) {
    let mut grid = vec![vec![".".to_string(); width]; height];
    for coord in coords {
        grid[coord.1 as usize][coord.0 as usize] = "#".to_string();
    }

    println!("{:?}", grid);
}