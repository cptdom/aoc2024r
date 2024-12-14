use common::parse_two_lists_input;

static _INPUT_PATH: &str = "tasks/01/input.txt";

fn main() {
    if let Ok((left_col, right_col)) = parse_two_lists_input(_INPUT_PATH) {
        // assuming both columns have the same length
        let mut similarity_score: usize = 0;
        // for each number from the left col, figure out the num of its occurences
        // in the right col, and multiply the number with this num
        // and append to the total
        for num in left_col.iter() {
            let n_occ = count_number(&right_col, *num);
            similarity_score += num * n_occ;
        }

        println!("The final similarity score is {similarity_score}");
    } else {
        println!("input parsing failed miserably")
    }
}

// NOTE: to self - in order to borrow and not take ownership, we use a reference to the vector
// not the vector itself
fn count_number(vec: &Vec<usize>, target: usize) -> usize {
    // NOTE: to self -> iter() returns iterator of references
    // and filter() works with references to references
    vec.iter().filter(|x| **x == target).count()
}