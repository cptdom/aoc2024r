use std::collections::HashSet;

static _INPUT_PATH: &str = "tasks/17/input.txt";

fn main() {
    // load file
    // single vector of usize
    let input: Vec<usize> = t17::get_input(_INPUT_PATH);

    // step 1 - transform input into human-readable disk space

    let mut transformed_input = t17::transform_input(input);

    // step 2 - reposition accordingly
    // println!("transformed input: {:?}", transformed_input);

    transformed_input = defragment(transformed_input);

    // println!("defragmented input: {:?}", transformed_input);

    let checksum: usize = calculate_checksum(&transformed_input);

    println!("CHECKSUM {checksum}");

}
fn defragment(mut transformed_input: Vec<isize>) -> Vec<isize> {
    // 2 pointers approach
    // 1 going from the start
    // 1 going from the back
    let mut back_pointer: usize = transformed_input.len() -1;
    //
    //
    // first, the back pointer goes backwards looking for chunks of files
    let mut curr_file_size: usize = 0;

    // when a chunk of file is found, we save the size and the "ID" of it
    // now, we'll start looking for a spot for the file
    // going from the start UNTIL THE CURRENT POSITION OF THE BACK POINTER IS REACHED
    // when a suitable chunk of empty space is found
    // we switch the indexes accordingly
    // and move the back pointer towards the next chunk
    // println!("BACK POINTER: {back_pointer}");

    let mut already_moved: HashSet<isize> = HashSet::new();
    // 'backwards:
    while back_pointer > 0 {
        // println!("BACK POINTER LOOP: {back_pointer}");
        // if the back starts with free space
        if transformed_input[back_pointer] == -1 || already_moved.contains(&transformed_input[back_pointer]){
            if curr_file_size > 0 {
                // here we have a file chunk and need to find space for it
                // println!("BACK POINTER: {back_pointer}");
                // println!("FILE CHUNK: {curr_file_size} of ID {:?}", transformed_input[back_pointer+1]);
                let mut curr_free_space: usize = 0;

                let mut start_pointer: usize = 0;
                'space_lookup:
                while start_pointer < back_pointer {
                    // println!("DEBUG 1");
                    if transformed_input[start_pointer] == -1 {
                        curr_free_space += 1;
                        // println!("DEBUG 2 at index {start_pointer}");
                    }
                    if curr_free_space >= curr_file_size {
                        // println!("FOUND FREE SPACE OF {curr_free_space} AT INDEX {start_pointer}");
                        // swap
                        already_moved.insert(transformed_input[back_pointer+1]);
                        swap_n_times(&mut transformed_input, start_pointer, back_pointer-curr_file_size, curr_file_size);
                        // move pointers
                        // here break space lookup
                        // println!("DEBUG 3");
                        break 'space_lookup
                    }
                    if transformed_input[start_pointer] != -1 {
                        // println!("DEBUG 4 at index {start_pointer}");
                        curr_free_space = 0;
                        start_pointer += 1;
                        continue 'space_lookup
                    }
                    // we're still in the loop about to end
                    // and we haven't found a match
                    // so we leave the file in place and move back pointer backwards
                    if start_pointer == back_pointer-1 && curr_file_size > 0 {
                         // if not, continue and move the back_pointer to the end of the file
                        back_pointer -= curr_file_size;
                        // println!("DEBUG 5");
                        break 'space_lookup
                    }
                    start_pointer += 1;
                }
            }
            curr_file_size = 0;
        } else {
            if !already_moved.contains(&transformed_input[back_pointer]) {
                // println!("INDEX {back_pointer} INCR SIZE FOR {:?}",transformed_input[back_pointer]);
                curr_file_size += 1;
            } else {
                curr_file_size = 0;
            }
        }
        back_pointer -= 1;
    }
    transformed_input
}

// 154123
// 0.....1111.22...
// 022...1111......


fn swap_n_times(target: &mut Vec<isize>, left: usize, right: usize, steps: usize) -> &mut Vec<isize> {
    // println!("SWAPPING WITH PARAMS: {left}, {right}, {steps}");
    // println!("PRE TARGET: {:?}", target);
    for i in 0..steps {
        // println!("PERFORMING SWAP: {:?}, {:?}", left-steps+1+i, right+steps+1+i);
        target.swap(left-steps+1+i, right+steps+1+i);
    }
    // println!("POST TARGET: {:?}", target);
    target
}

fn calculate_checksum(input: &Vec<isize>) -> usize {
    input.iter()
        .enumerate()
        .filter(|(_, &a)| a >= 0)
        .map(|(i, n)| i * (*n as usize))
        .sum()
}
