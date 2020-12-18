use std::collections::HashMap;
use std::io::{self, BufRead};

enum Instruction {
    Mask(String),
    Mem { pos: u64, val: u64 },
}

impl From<String> for Instruction {
    fn from(s: String) -> Self {
        let mut s = s.split(" = ");
        let first = s.next().unwrap();

        if first == "mask" {
            let next = s.next().unwrap();
            return Self::Mask(next.into());
        }

        // it's a mem instruction
        let pos: u64 = first[4..first.len() - 1].parse().unwrap();
        let val: u64 = s.next().unwrap().parse().unwrap();
        Self::Mem { pos, val }
    }
}

fn write_floating(mem: &mut HashMap<u64, u64>, pos: u64, floating: &[usize], val: u64) {
    if floating.is_empty() {
        mem.insert(pos, val);
        return;
    }

    let bit = 1 << floating[0];
    let floating = &floating[1..];
    write_floating(mem, pos | bit, floating, val);

    let bit = !bit;
    write_floating(mem, pos & bit, floating, val);
}

fn main() {
    let mut mem: HashMap<u64, u64> = HashMap::new();

    let mut mask = String::new();

    for line in io::stdin().lock().lines().flatten() {
        let instruction = Instruction::from(line);
        match instruction {
            Instruction::Mask(s) => {
                mask = s;
            }
            Instruction::Mem { pos, val } => {
                // positions of floating bits
                let floating: Vec<usize> = mask
                    .chars()
                    .rev()
                    .enumerate()
                    .filter(|(_, s)| *s == 'X')
                    .map(|(n, _)| n)
                    .collect();

                let ones = mask.replace('X', "0");
                let ones = u64::from_str_radix(&ones, 2).unwrap();

                let pos = pos | ones;

                write_floating(&mut mem, pos, &floating, val);
            }
        };
    }

    let ans: u64 = mem.iter().map(|(_, v)| v).sum();

    println!("{}", ans);
}
