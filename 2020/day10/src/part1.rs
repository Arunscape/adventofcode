use itertools::Itertools;
use std::io::{self, BufRead};

fn main() {
    let mut input: Vec<usize> = std::io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|s| s.parse())
        .collect();

    input.sort_unstable();

    let mut one_jolt = 1;
    let mut three_jolt = 1;

    for (x, y) in input.iter().tuple_windows() {
        match y - x {
            1 => one_jolt += 1,
            3 => three_jolt += 1,
            _ => {}
        }
    }

    println!("1 jolt: {}\n3 jolt: {}", one_jolt, three_jolt);
}
