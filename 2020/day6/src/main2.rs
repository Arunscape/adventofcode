#![feature(iterator_fold_self)]

use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let responses: Vec<_> = buffer.split("\n\n").collect();

    let ans: usize = responses
        .iter()
        .map(|&group| {
            group
                .split_whitespace()
                .map(|person| person.chars().collect::<HashSet<_>>())
                .fold_first(|a, b| a.intersection(&b).map(|&c| c).collect::<HashSet<char>>())
                .unwrap()
                .len()
        })
        .sum();

    println!("{}", ans);

    Ok(())
}
