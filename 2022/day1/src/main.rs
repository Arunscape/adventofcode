#![feature(binary_heap_into_iter_sorted)]
use std::io::Read;
use std::collections::BinaryHeap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    std::io::stdin().read_to_string(&mut input)?;

    let ans = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .flat_map(|food| food.parse::<usize>())
                .sum::<usize>()
        })
        .max();

    println!("part 1: {ans:?}");


    let mut ans: BinaryHeap<_> = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .flat_map(|food| food.parse::<usize>())
                .sum::<usize>()
        }).collect();


    let ans: usize = ans.into_iter_sorted().take(3).sum();

    println!("part 2: {ans:?}");

    Ok(())
}
