use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let passports: Vec<_> = buffer.split("\n\n").collect();

    let ans = passports
        .iter()
        .filter(|&passport| {
            let p: Vec<_> = passport.split_whitespace().collect();

            if p.iter().count() == 8 {
                return true;
            }

            if p.iter().count() == 7 {
                return p.iter().all(|field| !field[..3].eq("cid"));
            }

            false
        })
        .count();

    println!("{}", ans);

    Ok(())
}
