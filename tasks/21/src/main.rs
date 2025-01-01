
static _INPUT_PATH: &str = "tasks/21/input.txt";

// we can solve this by iterating 25 times
// or perhaps recursion?
// though the trick is to keep track of the "blinks"
fn main() {
    let input = t21::load_vec(_INPUT_PATH);
    let steps = 25;

    let cnt = expand(&input, steps);

    println!("final length after {steps} steps: {cnt}");
}

fn expand(nums: &Vec<usize>, steps: usize) -> usize {
    let mut transformed_nums = vec![];
    let mut members_count: usize = 0;
    // base case
    if steps == 0 {
        return nums.len()
    }
    for &num in nums {
        match num {
            0 => {
                transformed_nums.append(&mut vec![1]);
            },
            _ if format!("{num}").len() % 2 == 0 => {
                let num_str = format!("{num}");
                let half_ix = num_str.len() / 2;
                let new_left = num_str[0..half_ix].parse::<usize>().unwrap();
                let new_right = num_str[half_ix..].parse::<usize>().unwrap();
                transformed_nums.append(&mut vec![new_left, new_right]);
            },
            _ => {
                transformed_nums.append(&mut vec![num * 2024]);
            }
        }
    }

    members_count += expand(&transformed_nums, steps-1);

    members_count

}
