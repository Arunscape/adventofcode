use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let ans = stdin
        .lock()
        .lines()
        .flatten()
        .enumerate()
        .filter(|(lineno, line)| line.chars().cycle().nth(lineno * 3).unwrap() == '#')
        .count();

    println!("{}", ans);
}
