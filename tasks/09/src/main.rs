use std::collections::HashSet;



static _INPUT_PATH: &str = "tasks/09/input.txt";

fn main() {
    let (pairs, vectors) = t09::parse_file(_INPUT_PATH);

    let total = sum_ordered(pairs, vectors);
    println!("The total is {total}");
}

fn sum_ordered(rules: HashSet<(usize, usize)>, orders: Vec<Vec<usize>>) -> usize {
    orders.iter()
        .filter_map(|update| {
            // check if the order satisfies the rules
            if update.windows(2).all(|w| !rules.contains(&(w[1], w[0]))) {
                // if it does, return the middle element
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum() // and sum them all up
}

