
static _INPUT_PATH: &str = "tasks/18/input.txt";

fn main() {
    // load file
    // single vector of usize
    let input: Vec<usize> = t17::get_input(_INPUT_PATH);

    // step 1 - transform input into human-readable disk space

    let mut transformed_input = t17::transform_input(input);

    // step 2 - reposition accordingly
    println!("transformed input: {:?}", transformed_input);

    transformed_input = defragment(transformed_input);

    println!("defragmented input: {:?}", transformed_input);

    let checksum: usize = t17::calculate_checksum(&transformed_input);

    println!("CHECKSUM {checksum}");

}
fn defragment(mut transformed_input: Vec<isize>) -> Vec<isize> {
    vec![]
}