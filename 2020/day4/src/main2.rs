use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let passports: Vec<_> = buffer.split("\n\n").collect();

    let ans = passports
        .iter()
        .filter(|&passport| {
            let mut count = 0;

            for field in passport.split_whitespace() {
                match &field[..3] {
                    "byr" => {
                        let year: usize = field[4..].parse().unwrap();
                        if (1920..=2002).contains(&year) {
                            count += 1;
                        }
                    }
                    "iyr" => {
                        let year: usize = field[4..].parse().unwrap();
                        if (2010..=2020).contains(&year) {
                            count += 1;
                        }
                    }
                    "eyr" => {
                        let year: usize = field[4..].parse().unwrap();
                        if (2020..=2030).contains(&year) {
                            count += 1;
                        }
                    }
                    "hgt" => {
                        if match &field[field.len() - 2..] {
                            "cm" => {
                                let height = field[4..7].parse();
                                height.is_ok() && (150..=193).contains(&height.unwrap())
                            }
                            "in" => {
                                let height = field[4..6].parse();
                                height.is_ok() && (59..=76).contains(&height.unwrap())
                            }
                            _ => false,
                        } {
                            count += 1;
                        }
                    }
                    "hcl" => {
                        if field[5..].len() == 6 && field[5..].chars().all(char::is_alphanumeric) {
                            count += 1;
                        }
                    }
                    "ecl" => {
                        if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                            .iter()
                            .any(|&c| c.eq(&field[4..]))
                        {
                            count += 1;
                        }
                    }
                    "pid" => {
                        if field[4..].len() == 9 && field[4..].chars().all(char::is_numeric) {
                            count += 1;
                        }
                    }
                    "cid" => {}
                    _ => panic!("invalid field"),
                }
            }
            count == 7
        })
        .count();

    println!("{}", ans);

    Ok(())
}
