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

            let range = range.next().unwrap()..=range.next().unwrap();

            let letter = line.next().unwrap().chars().next().unwrap();

            let passwd = line.next().unwrap();

            range.contains(&passwd.matches(letter).count())
        })
        .count();

    println!("{}", ans);
}
