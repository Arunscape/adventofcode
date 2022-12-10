use std::io::{self, BufRead};

const WIDTH: isize = 40;
const HEIGHT: isize = 6;

#[derive(Debug)]
struct CPU {
    reg_x: isize,
    cycle: isize,
    signal_strengths: Vec<isize>,
    scanline: Vec<char>,
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Noop,
    Addx(isize),
}

impl CPU {
    pub fn signal_strength(&self) -> isize {
        self.cycle * self.reg_x
    }
    pub fn sprite_visible(&self) -> bool {
        let pixel = (self.cycle % WIDTH) - 1;
        (pixel - 1) <= self.reg_x && self.reg_x <= (pixel + 1)
    }
    pub fn add_cycle(&mut self) {
        self.cycle += 1;
        if (self.cycle - 20) % WIDTH == 0 {
            self.signal_strengths.push(self.signal_strength());
        }
        if self.sprite_visible() {
            self.scanline.push('#');
        } else {
            self.scanline.push('.');
        }
        if self.scanline.len() == 40 {
            for c in self.scanline.iter(){
                print!("{c}");
            }
            println!();
            self.scanline = Vec::with_capacity(WIDTH as usize);
        }
        //dbg!(&self);
    }
    pub fn process(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => self.add_cycle(),
            Instruction::Addx(n) => {
                self.add_cycle();
                self.add_cycle();
                self.reg_x += n;
            }
        }
    }
}
fn main() {
    let input: Vec<_> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .filter_map(|line| {
            if line == "noop" {
                Some(Instruction::Noop)
            } else if let Some(n) = line.strip_prefix("addx ") {
                let n = n.parse().ok()?;
                Some(Instruction::Addx(n))
            } else {
                None
            }
        })
        .collect();
    let mut cpu = CPU {
        reg_x: 1,
        cycle: 0,
        signal_strengths: vec![],
        scanline: vec![],
    };

    dbg!(&cpu);
    for &instr in input.iter() {
        cpu.process(instr);
    }

    let part1: isize = cpu.signal_strengths.iter().sum();
    println!("part 1: {part1:?}");
}
