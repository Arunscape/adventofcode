use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

#[derive(Debug)]
struct Monke {
    items: VecDeque<usize>,
    operation: (Operand, OpSign, Operand),
    testnum: usize,
    iftrue: usize,
    iffalse: usize,
    n_inspected: usize,
}

impl Monke {
    pub fn new(
        items: &[usize],
        operation: (Operand, OpSign, Operand),
        testnum: usize,
        iftrue: usize,
        iffalse: usize,
    ) -> Self {
        let items = items.iter().cloned().collect();
        Self {
            items,
            operation,
            testnum,
            iftrue,
            iffalse,
            n_inspected: 0,
        }
    }

    pub fn do_op(&self, old: usize) -> usize {
        match self.operation {
            (Operand::Old, OpSign::Mul, Operand::Old) => old * old,
            (Operand::Old, OpSign::Add, Operand::Old) => old + old,
            (Operand::Old, OpSign::Mul, Operand::Num(n)) => old * n,
            (Operand::Old, OpSign::Add, Operand::Num(n)) => old + n,
            (Operand::Num(x), OpSign::Mul, Operand::Num(y)) => x * y,
            (Operand::Num(x), OpSign::Add, Operand::Num(y)) => x + y,
            (Operand::Num(n), OpSign::Mul, Operand::Old) => n * n,
            (Operand::Num(n), OpSign::Add, Operand::Old) => n + n,
        }
    }

    pub fn inspect(&mut self) -> Option<(usize, usize)> {
        let x = self.items.pop_front()?;
        self.n_inspected += 1;

        println!("  monke inspects an item with a worry level of {x}");
        //println!("{:?}", self);

        let x = self.do_op(x);

        println!("    worry level is {:?} to = {}", self.operation, x);

        let x = x / 3;

        println!("    monke gets bored. worry level is divided by 3 to {x}");

        let x = if x % self.testnum == 0 {
            (self.iftrue, x)
        } else {
            (self.iffalse, x)
        };

        println!(
            "    item with worry level {} is thrown to monke {}",
            x.1, x.0
        );

        Some(x)
    }
}

#[derive(Debug)]
enum OpSign {
    Add,
    Mul,
}

#[derive(Debug)]
enum Operand {
    Old,
    Num(usize),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut monkes: HashMap<usize, Monke> = input
        .split("\n\n")
        .filter_map(|m| {
            let m: Vec<&str> = m.lines().collect();

            let monke_num = m.get(0)?;
            let monke_num: String = monke_num.chars().filter(|c| c.is_numeric()).collect();
            let monke_num: usize = monke_num.parse().ok()?;

            let items = m.get(1)?;
            let items = items.trim_start().strip_prefix("Starting items: ")?;
            let items: &Vec<usize> = &items.split(", ").filter_map(|i| i.parse().ok()).collect();

            let op = m.get(2)?;
            let op = op.trim_start().strip_prefix("Operation: new = ")?;
            let op: Vec<_> = op.split_whitespace().collect();
            let op1 = match op.get(0) {
                Some(&"old") => Some(Operand::Old),
                Some(n) => {
                    let n = n.parse().ok()?;
                    Some(Operand::Num(n))
                }
                None => None,
            }?;
            let op2 = match op.get(1) {
                Some(&"+") => Some(OpSign::Add),
                Some(&"*") => Some(OpSign::Mul),
                _ => None,
            }?;
            let op3 = match op.get(2) {
                Some(&"old") => Some(Operand::Old),
                Some(n) => {
                    let n = n.parse().ok()?;
                    Some(Operand::Num(n))
                }
                None => None,
            }?;
            let operation = (op1, op2, op3);

            let testnum = m.get(3)?;
            let testnum = testnum.trim_start().strip_prefix("Test: divisible by ")?;
            let testnum: usize = testnum.parse().ok()?;

            let iftrue = m.get(4)?;
            let iftrue = iftrue
                .trim_start()
                .strip_prefix("If true: throw to monkey ")?;
            let iftrue: usize = iftrue.parse().ok()?;

            let iffalse = m.get(5)?;
            let iffalse = iffalse
                .trim_start()
                .strip_prefix("If false: throw to monkey ")?;
            let iffalse: usize = iffalse.parse().ok()?;

            let monke = Monke::new(items, operation, testnum, iftrue, iffalse);

            Some((monke_num, monke))
        })
        .collect();

    let mut curr = 0;
    let mut n_round = 1;

    let n_monke: usize = *monkes.keys().max().unwrap();

    dbg!(&monkes);

    println!("monke: 0 {:?}", monkes[&0]);
    while n_round <= 20 {
        let m = monkes.get_mut(&curr).unwrap();
        if let Some((n, mon)) = m.inspect() {
            let dest = monkes.get_mut(&n).unwrap();
            dest.items.push_back(mon);
            //println!(
            //    "monke {} threw to monke {} which now has items {:?}",
            //    curr, n, dest.items
            //);
        } else {
            curr += 1;
            if n_monke < curr {
                curr = 0;
                println!();
                println!("End of round {n_round}");
                n_round += 1;
                for (k, v) in monkes.iter() {
                    println!("#monke {k} items: {:?}", v.items);
                }
            }
            println!("monke {curr}:");
        }
    }

    for (k, v) in monkes.iter() {
        println!("monke {k}, {}", v.n_inspected);
    }
    let mut monke_business: Vec<_> = monkes.values().map(|v| v.n_inspected).collect();
    monke_business.sort();
    let monke_business = monke_business.last().unwrap() * monke_business[monke_business.len() -2];
    println!("part 1: {monke_business}");

    Ok(())
}
