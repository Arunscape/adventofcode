mod main;
use itertools::Itertools;
use std::io::{self, BufRead};

fn main() {
    let mut ids: Vec<_> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(main::find_id)
        .collect();
    ids.sort_unstable();

    for (id, &next) in ids.iter().tuple_windows() {
        if next != id + 1 {
            println!("{}", id + 1);
            return;
        }
    }
}
