use itertools::Itertools;
use std::io::BufRead;

/// https://stackoverflow.com/a/50392400
fn consecutive_slices(data: &[usize]) -> impl Iterator<Item = &[usize]> {
    let mut slice_start = 0;
    (1..=data.len()).flat_map(move |i| {
        if i == data.len() || data[i - 1] != data[i] - 1 {
            let begin = slice_start;
            slice_start = i;
            Some(&data[begin..i])
        } else {
            None
        }
    })
}

fn count(n: usize) -> usize {
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 4,
        _ => count(n - 1) + count(n - 2) + count(n - 3),
    }
}

fn main() {
    let mut input: Vec<usize> = std::io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|s| s.parse())
        .collect();

    input.sort_unstable();

    let ans: usize = consecutive_slices(&input)
        .map(|s| s.len() - 1)
        .map(count)
        .product();

    println!("{}", ans);
}
