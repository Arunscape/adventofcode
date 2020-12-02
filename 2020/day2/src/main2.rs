use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let ans = stdin
        .lock()
        .lines()
        .flatten()
        .filter(|line| {
            let mut line = line.split(' ');

            let mut range = line.next().unwrap().split('-').flat_map(|i| i.parse());
            let pos1: usize = range.next().unwrap();
            let pos2: usize = range.next().unwrap();

            let letter = line.next().unwrap().chars().next().unwrap();

            let passwd = line.next().unwrap();

            if passwd.chars().nth(pos1 - 1).unwrap() == letter {
                passwd.chars().nth(pos2 - 1).unwrap() != letter
            } else {
                passwd.chars().nth(pos2 - 1).unwrap() == letter
            }
        })
        .count();

    println!("{}", ans);
}
