use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let ans = stdin
        .lock()
        .lines()
        .flatten()
        .map(|line| line.chars().cycle());

    println!("{}", ans);
}
