
static _INPUT_PATH: &str = "tasks/17/input.txt";

fn main() {
    // load file
    // single vector of usize
    let input: Vec<usize> = t17::get_input(_INPUT_PATH);

    // can be done in one iteration while going from both directions
    // and having 2 queues, one for whitespace indices, one for odd members
    // however, let's keep it readable

    // step 1 - transform input into human-readable disk space

    let mut transformed_input = t17::transform_input(input);

    // step 2 - reposition accordingly

    transformed_input = defragment(transformed_input);

    let checksum: usize = t17::calculate_checksum(&transformed_input);

    println!("CHECKSUM {checksum}");

}

// simple approach - 2 pointers
fn defragment(mut transformed_input: Vec<isize>) -> Vec<isize> {
    let mut left = 0;
    let mut right = transformed_input.len()-1;

    while left < right {
        // negative number lookup from the left
        if transformed_input[left] < 0 && transformed_input[right] >= 0 {
            transformed_input.swap(left, right);
            left += 1;
            right -= 1;
        } else {
            // if left positive, move to the right
            if transformed_input[left] >= 0 {
                left += 1;
            }
            // if right negative, move to the left
            if transformed_input[right] < 0 {
                right -= 1;
            }
        }
    }
    transformed_input.to_vec()
}