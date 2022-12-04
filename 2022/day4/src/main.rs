use std::io::{self, BufRead};

fn main() {
    let input: Vec<_> = io::stdin().lock().lines().flatten().collect();
    let ans = input
        .iter()
        .filter(|line| {
            let (p1, p2) = line.split_once(",").unwrap();

            let (low1, high1) = p1.split_once("-").unwrap();
            let (low2, high2) = p2.split_once("-").unwrap();

            let low1: usize = low1.parse().unwrap();
            let low2: usize = low2.parse().unwrap();
            let high1: usize = high1.parse().unwrap();
            let high2: usize = high2.parse().unwrap();

            (low1..=high1).contains(&low2) && (low1..=high1).contains(&high2)
                || (low2..=high2).contains(&low1) && (low2..=high2).contains(&high1)
        })
        .count();

    println!("part 1: {ans}");

    let ans = input.iter().filter(|line| {
            let (p1, p2) = line.split_once(",").unwrap();

            let (low1, high1) = p1.split_once("-").unwrap();
            let (low2, high2) = p2.split_once("-").unwrap();

            let low1: usize = low1.parse().unwrap();
            let low2: usize = low2.parse().unwrap();
            let high1: usize = high1.parse().unwrap();
            let high2: usize = high2.parse().unwrap();

            (low1 <= high2) && (high1 >= low2)
        })
        .count();
    
    println!("part 2: {ans}");

}
