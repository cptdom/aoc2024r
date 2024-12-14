
use common::parse_two_lists_input;

static _INPUT_PATH: &str = "tasks/01/input.txt";

fn main() {
    if let Ok((mut left_col, mut right_col)) = parse_two_lists_input(_INPUT_PATH) {
        // assuming both columns have the same length
        let mut total_distance = 0;
        // sort both in place since smallest number is compared to the smallest in
        // the other column
        left_col.sort();
        right_col.sort();
        for (left_num, right_num) in left_col.iter().zip(right_col.iter()) {
            total_distance += left_num.abs_diff(*right_num);
        }
        println!("The total distance is {total_distance}");
    } else {
        println!("input parsing failed miserably")
    }
}
