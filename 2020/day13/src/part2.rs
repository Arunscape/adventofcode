use std::io::{self, BufRead};

fn main() {
    let busses: Vec<(usize, usize)> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .map(|(p, s)| (p, s.parse().unwrap_or(0)))
        .filter(|(_, n)| n > &0)
        .collect();
}
