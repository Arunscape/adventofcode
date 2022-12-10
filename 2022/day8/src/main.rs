use ndarray::prelude::*;
use std::collections::HashSet;
use std::io::{self, BufRead};
fn main() {
    // need to know the shape of the vec so can't flatten it right away
    //let input: Vec<_> = io::stdin()
    //    .lock()
    //    .lines()
    //    .flatten()
    //    .flat_map(|line| line
    //              .chars()
    //              .filter_map(|c| c.to_digit(10))
    //              .collect::<Vec<_>>()
    //              )
    //    .collect();
    let input: Vec<_> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect();

    let (x, y) = (input[0].len(), input.len());
    let input = input.iter().cloned().flatten().collect();

    let input: Array2<u32> = Array2::from_shape_vec((x, y), input).unwrap();

    let mut visible = HashSet::new();
    for ((x, y), &tree) in input.indexed_iter() {
        println!("({x},{y}) tree: {tree}");
        if is_visible(&input, (x, y), tree) {
            visible.insert((x, y));
            println!("  visible");
        }
    }
    println!("part 1: {}", visible.len());

    let ans = input
        .indexed_iter()
        .map(|((x, y), &tree)| score(&input, (x, y), tree))
        .max()
        .unwrap();
    println!("part 2: {ans}");
}

fn is_visible(input: &Array2<u32>, (x, y): (usize, usize), height: u32) -> bool {
    if x == 0 || y == 0 || x == input.nrows() - 1 || y == input.ncols() - 1 {
        return true;
    }

    if input.row(x).slice(s![..y]).iter().all(|&t| t < height) {
        return true;
    }
    if input.row(x).slice(s![y + 1..]).iter().all(|&t| t < height) {
        return true;
    }
    if input.column(y).slice(s![..x]).iter().all(|&t| t < height) {
        return true;
    }
    if input
        .column(y)
        .slice(s![x + 1..])
        .iter()
        .all(|&t| t < height)
    {
        return true;
    }

    false
}

fn score(input: &Array2<u32>, (x, y): (usize, usize), height: u32) -> usize {
    if x == 0 || y == 0 || x == input.nrows() - 1 || y == input.ncols() - 1 {
        return 0;
    }

    println!("({y},{x}): {height}");
    print!("  left: [");
    let left = input
        .row(x);
    let left = left
        .slice(s![..y]);
    let left = left
        .iter()
        .rev()
        .take_while(|&&t| t <= height);

    let left: Vec<u32> = left.cloned().collect();
    let left = left.len();
    println!(" ] count: {left}");
    print!("  right: [");
    let right = input
        .row(x)
        .slice(s![y + 1..])
        .iter()
        .take_while(|&&t| {
            t <= height
        })
    let up = input
        .column(y)
        .slice(s![..x])
        .iter()
        .rev()
        .take_while(|&&t| {
            t <= height
        })
        .count()
        + 1;
    let down = input
        .column(y)
        .slice(s![x + 1..])
        .iter()
        .rev()
        .take_while(|&&t| {
            t <= height
        })
        .count()
        + 1;
    left * right * up * down
}
