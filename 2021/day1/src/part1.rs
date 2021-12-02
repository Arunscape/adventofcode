use itertools::Itertools;
use std::io::{self, BufRead};

fn main() {
    let mut count = 0;

    for (d1, d2) in io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|s| s.parse::<usize>())
        .tuple_windows()
    {
        if d1 < d2 {
            count += 1
        }
    }

    println!("Part 1: {}", count);
}
