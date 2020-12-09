#![feature(str_split_once)]
use std::{
    collections::HashMap,
    io::{self, Read},
};

fn execute(instruction: &str, pc: &mut isize, acc: &mut isize) {
    let (instruction, num) = instruction.split_once(' ').unwrap();
    let num: isize = num.parse().unwrap();

    match instruction {
        "nop" => *pc += 1,
        "acc" => {
            *pc += 1;
            *acc += num;
        }
        "jmp" => *pc += num,
        _ => unreachable!(),
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input)?;

    let input: Vec<&str> = input.lines().collect();

    // line number and times visited
    let mut visited: HashMap<isize, usize> = HashMap::new();

    let mut pc = 0;
    let mut acc = 0;
    loop {
        if let Some(val) = visited.get(&pc) {
            if *val == 1 {
                println!("{}", acc);
                return Ok(());
            }
        }

        visited.entry(pc).and_modify(|n| *n += 1).or_insert(1);
        execute(input[pc as usize], &mut pc, &mut acc);
    }
}
