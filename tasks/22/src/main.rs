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
                // (num.ilog10() + 1) -> count of digits,
                // e.g. 25 -> log(10)25 =~ 1.4 -> 2.4 -> 2
                // e.g. 125 e.g. 25 -> log(10)25 =~ 2.1 -> 3.1 -> 3
                _ if (num.ilog10() + 1) % 2 == 0 => {
                    let scale = 10_usize.pow((num.ilog10() + 1) / 2); // e.g. 25 -> 10 ^ 1
                    insert(&mut transformed_nums, *num / scale, *cnt); // e.g. 25 -> 10 -> 2.5 -> 2
                    insert(&mut transformed_nums, *num % scale, *cnt); // e.g. 25 -> 10 -> 5 -> 5
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
