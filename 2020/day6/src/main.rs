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
                .chars()
                .filter(char::is_ascii_lowercase)
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();

    println!("{}", ans);

    Ok(())
}
