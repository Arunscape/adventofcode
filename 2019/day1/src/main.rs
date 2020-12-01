use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let nums: Vec<usize> = stdin
        .lock()
        .lines()
        .flatten()
        .map(|s| s.parse::<usize>())
        .flatten()
        .collect();

    let sum: usize = nums.clone().into_iter().map(|n| n / 3 - 2).sum();

    println!("Sum: {}", sum);

    fn fuel(n: usize) -> usize {
        fn fuel_calculation(n: usize, acc: usize) -> usize {
            match (n / 3).checked_sub(2) {
                Some(x) => fuel_calculation(x, acc + x),
                None => acc,
            }
        }
        fuel_calculation(n, 0)
    }

    let sum: usize = nums.clone().into_iter().map(fuel).sum();
    println!("Sum with fuel: {}", sum);
}
