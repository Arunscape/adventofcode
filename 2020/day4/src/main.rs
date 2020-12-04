use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let p: Vec<_> = buffer.split("\n\n").collect();

    println!("{:?}", p);

    Ok(())
}
