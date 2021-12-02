use itertools::Itertools;
use std::io::{self, BufRead};

fn main() {
    let mut count = -1;
    let mut previous_sum = 0;

    for (d1, d2, d3) in io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|s| s.parse::<usize>())
        .tuple_windows()
    {
        let sum = d1 + d2 + d3;
        if previous_sum < sum {
            count += 1
        }
        previous_sum = sum;
    }

    println!("Part 2: {}", count);
}
