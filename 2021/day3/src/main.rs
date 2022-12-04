use std::io::{self, BufRead};

fn main() {
    io::stdin().lock().lines().flatten().map(|s| s.parse::<usize>(2))
}
