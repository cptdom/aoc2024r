
use std::{cmp::Ordering, collections::HashSet};

static _INPUT_PATH: &str = "tasks/09/input.txt";

fn main() {
    let (pairs, vectors) = t09::parse_file(_INPUT_PATH);

    let total = sum_ordered(pairs, vectors);
    println!("The total is {total}");
}

fn sum_ordered(rules: HashSet<(usize, usize)>, mut orders: Vec<Vec<usize>>) -> usize {
    orders.iter_mut()
        .filter_map(|update| {
            // check if the order breaks the rules
            if !is_sorted(update, &rules) {
                // if it does, we sort it by the rules
                update.sort_by(|a, b| {
                    if rules.contains(&(*a, *b)) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                Some(update[update.len()/2])
            } else {
                None
            }
        }).sum()
}

fn is_sorted(update: &Vec<usize>, rules: &HashSet<(usize,usize)>) -> bool {
    update.windows(2).all(|w| !rules.contains(&(w[1], w[0])))
}