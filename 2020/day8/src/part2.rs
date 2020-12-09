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

    for (n, line) in input.iter().enumerate() {
        let mut input = input.clone();

        let x = input[n].replace("jmp", "nop");
        input[n] = &x;
        let y = input[n].replace("nop", "jmp");
        input[n] = &y;

        loop {
            println!("{} acc: {}", input[pc as usize], acc);

            if let Some(v) = visited.get(&pc) {
                if *v == 1 {
                    pc = 0;
                    acc = 0;
                    break;
                }
            }
            //println!("{}", input[pc as usize]);
            visited.entry(pc).and_modify(|n| *n += 1).or_insert(1);
            execute(input[pc as usize], &mut pc, &mut acc);
            //std::thread::sleep(std::time::Duration::new(1, 0));
            println!("acc changed to {}", acc);
        }
    }
    Ok(())
}
