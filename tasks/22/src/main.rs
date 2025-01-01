use std::collections::HashMap;

static _INPUT_PATH: &str = "tasks/21/input.txt";

// recursion won't do the trick here
fn main() {
    let input = t21::load_vec(_INPUT_PATH);
    let mut input_map: HashMap<usize, usize> = HashMap::new();
    for num in input {
        *input_map.entry(num).or_insert(0) += 1;
    }

    const STEPS: usize = 75;

    let cnt: usize = expand::<STEPS>(input_map)
        .iter()
        .map(|x| x.1)
        .sum();

    println!("final length after {STEPS} steps: {cnt}");
}

fn expand<const STEPS: usize>(nums: HashMap<usize, usize>) -> HashMap<usize, usize> {

    let mut curr_nums = nums;

    for _ in 0..STEPS {
        let mut transformed_nums: HashMap<usize, usize> = HashMap::new();

        for (num, cnt) in curr_nums.iter() {
            match num {
                0 => {
                    insert(&mut transformed_nums, 1, *cnt);
                },
                _ if format!("{num}").len() % 2 == 0 => {
                    let num_str = format!("{num}");
                    let half_ix = num_str.len() / 2;
                    let new_left = num_str[0..half_ix].parse::<usize>().unwrap();
                    let new_right = num_str[half_ix..].parse::<usize>().unwrap();
                    insert(&mut transformed_nums, new_left, *cnt);
                    insert(&mut transformed_nums, new_right, *cnt);
                },
                _ => {
                    // hackity hack - "<< 3" = "* 8"
                    insert(&mut transformed_nums, (num * 253) << 3, *cnt);
                }
            }
        }

        curr_nums = transformed_nums;
    }

    curr_nums

}

fn insert(target: &mut HashMap<usize, usize>, key: usize, cnt: usize) {
    target.insert(key, target.get(&key).unwrap_or(&0) + cnt);
}
