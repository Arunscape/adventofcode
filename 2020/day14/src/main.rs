use std::collections::HashMap;
use std::io::{self, BufRead};

enum Instruction {
    Mask { ones: u64, zeros: u64 },
    Mem { pos: u64, val: u64 },
}

impl From<String> for Instruction {
    fn from(s: String) -> Self {
        let mut s = s.split(" = ");
        let first = s.next().unwrap();

        if first == "mask" {
            let next = s.next().unwrap();
            let ones = next.replace('X', "0");
            let zeros = next.replace('X', "1");
            let ones = u64::from_str_radix(&ones, 2).unwrap();
            let zeros = u64::from_str_radix(&zeros, 2).unwrap();
            return Self::Mask { ones, zeros };
        }

        // it's a mem instruction
        let pos: u64 = first[4..first.len() - 1].parse().unwrap();
        let val: u64 = s.next().unwrap().parse().unwrap();
        Self::Mem { pos, val }
    }
}

fn main() {
    let mut mem: HashMap<u64, u64> = HashMap::new();

    let mut mask = Instruction::Mask { ones: 0, zeros: 0 };

    for line in io::stdin().lock().lines().flatten() {
        let instruction = Instruction::from(line);
        match instruction {
            Instruction::Mask { ones: _, zeros: _ } => {
                mask = instruction;
            }
            Instruction::Mem { pos, val } => {
                if let Instruction::Mask { ones, zeros } = mask {
                    let val = val | ones;
                    let val = val & zeros;
                    mem.entry(pos).and_modify(|v| *v = val).or_insert(val);
                }
            }
        };
    }

    let ans: u64 = mem.iter().map(|(_, v)| v).sum();

    println!("{}", ans);
}
