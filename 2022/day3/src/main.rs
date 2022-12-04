#![feature(iter_next_chunk)]
use std::collections::HashSet;
use std::io::{self, BufRead};

fn priority(letter: char) -> u32 {
    if ('a'..='z').contains(&letter) {
        letter as u32 - 96
    } else if ('A'..='Z').contains(&letter) {
        letter as u32 - 38
    } else {
        panic!("not [a-zA-Z]")
    }
}

fn main() {
    let input: Vec<_> = io::stdin().lock().lines().flatten().collect();

    let ans: u32 = input
        .iter()
        .map(|line| {
            let (c1, c2) = line.split_at(line.len() / 2);

            let mutual: HashSet<_> = c1.chars().filter(|&c| c2.contains(c)).collect();

            let s: u32 = mutual.iter().map(|&x| priority(x)).sum();
            s
        })
        .sum();

    println!("part1: {ans:?}");


    let mut sum = 0;

    let mut it = input.iter();
    while let Ok([a, b, c]) = it.next_chunk(){
        let x: HashSet<_> = a.chars().filter(|&ch| b.contains(ch) && c.contains(ch)).collect();
        assert_eq!(x.len(), 1);

        sum += priority(*x.iter().next().unwrap());
    }
    
    println!("part2: {sum:?}");
}
