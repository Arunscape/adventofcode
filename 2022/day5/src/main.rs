use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Ship {
    crates: HashMap<u32, Vec<char>>,
    n: u32,
}

impl Ship {
    pub fn move_crate_str(&mut self, command: &str, use_new: bool) {
        let nums: Vec<_> = command
            .split_whitespace()
            .filter_map(|s| {
                let s: u32 = s.parse().ok()?;
                Some(s)
            })
            .collect();

        let num = nums[0];
        let src = nums[1];
        let dest = nums[2];

        if use_new {
            self.move_crate_new(num, src, dest);
        } else {
            self.move_crate(num, src, dest);
        }
    }

    pub fn move_crate(&mut self, num: u32, src: u32, dest: u32) {
        if num < 1 {
            return;
        }

        if let Some(s) = self.crates.get_mut(&src) {
            if let Some(c) = s.pop() {
                if let Some(d) = self.crates.get_mut(&dest) {
                    d.push(c);
                }
            }
        }

        self.move_crate(num - 1, src, dest);
    }

    pub fn move_crate_new(&mut self, num: u32, src: u32, dest: u32) {
        if let Some(s) = self.crates.get_mut(&src) {
            let c: Vec<_> = s.drain((s.len() - num as usize)..).collect();
            if let Some(d) = self.crates.get_mut(&dest) {
                for &x in c.iter() {
                    d.push(x);
                }
            }
        }
    }
}

impl FromStr for Ship {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let columns = s.lines().last().ok_or(())?;
        let columns: HashMap<usize, u32> = columns
            .chars()
            .enumerate()
            .filter_map(|(i, c)| {
                let c = c.to_digit(10)?;
                Some((i, c))
            })
            .collect();

        let mut crates = HashMap::new();

        for (_, &c) in columns.iter() {
            crates.insert(c, Vec::new());
        }

        // fn column_to_char_position(c: usize) -> usize{
        //     // 1 -> 1
        //     // 2 -> 5
        //     // 3 -> 9
        //     // 4 -> 13
        //     // 5 -> 17
        //     1 + 4*(c-1)
        // }

        for line in s.lines() {
            for (&i, &col) in columns.iter() {
                let c = line.chars().nth(i).ok_or(())?;
                if c.is_whitespace() || c.is_numeric() {
                    continue;
                }
                crates.entry(col).and_modify(|v| v.push(c));
            }
        }

        for (_, v) in crates.iter_mut() {
            v.reverse();
        }

        let n = *columns.iter().map(|(_, x)| x).max().ok_or(())?;

        let s = Self { crates, n };

        Ok(s)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let (state, commands) = input.split_once("\n\n").ok_or("failed to parse input")?;

    let mut ship = Ship::from_str(&state).unwrap();

    for command in commands.lines() {
        ship.move_crate_str(command, false);
        dbg!(&ship);
    }

    print!("part 1: ");
    for v in 1..=ship.n {
        let l = ship.crates.get(&v).unwrap().last().unwrap();
        print!("{l}");
    }
    println!();

    let mut ship = Ship::from_str(&state).unwrap();
    for command in commands.lines() {
        ship.move_crate_str(command, true);
        dbg!(&ship);
    }

    print!("part 2: ");
    for v in 1..=ship.n {
        let l = ship.crates.get(&v).unwrap().last().unwrap();
        print!("{l}");
    }
    println!();
    Ok(())
}
