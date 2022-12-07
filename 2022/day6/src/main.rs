use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;


    let it: Vec<_> = input.chars().collect();
    let it = it.windows(4).enumerate().find(|(i, window)| {
        let set: HashSet<_> = window.iter().collect();
        set.len() == window.len()
    }).unwrap();


    println!("part 1: {}", it.0 + 4);
    
    let it: Vec<_> = input.chars().collect();
    let it = it.windows(14).enumerate().find(|(i, window)| {
        let set: HashSet<_> = window.iter().collect();
        set.len() == window.len()
    }).unwrap();
    
    println!("part 2: {}", it.0 + 14);

    Ok(())
}
