use itertools::Itertools;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input)?;

    let input: Vec<usize> = input.lines().flat_map(str::parse).collect();

    let window_size = 25;

    for v in input.windows(window_size + 1) {
        if v[..window_size]
            .iter()
            .tuple_combinations()
            .any(|(x, y)| x + y == v[window_size])
        {
            continue;
        } else {
            println!("{}", v[window_size]);
            return Ok(());
        }
    }
    Ok(())
}
