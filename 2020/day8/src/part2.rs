#![feature(str_split_once)]
use std::{
    collections::HashMap,
    io::{self, Read},
};

fn execute(instruction: &str, pc: &mut isize, acc: &mut isize) {
    let (instruction, num) = instruction.split_once(' ').unwrap();
    let num: isize = num.parse().unwrap();

    let ret = match instruction {
        "nop" => *pc += 1,
        "acc" => {
            *pc += 1;
            *acc += num;
        }
        "jmp" => *pc += num,
        _ => unreachable!(),
    };

    println!("acc: {}", acc);
    ret
}

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input)?;

    let input: Vec<&str> = input.lines().collect();

    // line number and times visited
    let mut visited: HashMap<isize, usize> = HashMap::new();

    'outer: for (num, _) in input.iter().enumerate() {
        let mut input2: Vec<String> = input.iter().map(|s| s.to_string()).collect();

        if input2[num].contains("jmp") {
            input2[num] = input[num].replace("jmp", "nop");
        } else if input2[num].contains("nop") {
            input2[num] = input[num].replace("nop", "jmp");
        } else {
            continue 'outer;
        }
        let mut pc = 0;
        let mut acc = 0;
        visited = HashMap::new();

        'inner: loop {
            if pc == input2.len() as isize {
                println!("we found it: {}", input[num]);
                return Ok(());
            }
            println!("{} | {}", input2[pc as usize], acc);
            if let Some(&val) = visited.get(&pc) {
                if val == 1 {
                    println!("{}", acc);
                    continue 'outer;
                }
            }

            visited.entry(pc).and_modify(|n| *n += 1).or_insert(1);
            execute(&input2[pc as usize], &mut pc, &mut acc);
        }
    }
    Ok(())
}
