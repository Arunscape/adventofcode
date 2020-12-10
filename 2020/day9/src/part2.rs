use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{self, Read},
};

fn get_invalid_num(input: &[usize], window_size: usize) -> usize {
    for v in input.windows(window_size + 1) {
        if v[..window_size]
            .iter()
            .tuple_combinations()
            .any(|(x, y)| x + y == v[window_size])
        {
            continue;
        } else {
            return v[window_size];
        }
    }
    unreachable!();
}

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input)?;

    let input: Vec<usize> = input.lines().flat_map(str::parse).collect();

    let invalid = get_invalid_num(&input, 25);

    // key = cumsum val = index
    let cum_sum: HashMap<usize, usize> = input
        .iter()
        .scan(0, |acc, &n| {
            *acc += n;
            Some(*acc)
        })
        .enumerate()
        .map(|(x, y)| (y, x))
        .collect();

    for (sum, &sum_n) in cum_sum.iter() {
        if let Some(&other_n) = cum_sum.get(&(sum + invalid)) {
            // bottom value is one lower
            let (min, max) = if sum_n < other_n {
                (sum_n, other_n)
            } else {
                (other_n, sum_n)
            };

            if input[min + 1] == invalid || input[max] == invalid {
                continue;
            }

            let min_num = input[(min + 1)..=max].iter().min().unwrap();

            let max_num = input[(min + 1)..=max].iter().max().unwrap();

            println!("{}", min_num + max_num);
        }
    }

    Ok(())
}
