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
        self.n_inspected += 1;
        let x = self.items.pop_front()?;

        let x = self.do_op(x);

        let x = x / 3;

        let x = if x % self.testnum == 0 {
            (self.iftrue, x)
        } else {
            (self.iffalse, x)
        };

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
        let mut i = 0;

        let n_monke: usize = *monkes.keys().max().unwrap();


        while i < 20 {
            let m = monkes.get_mut(&curr).unwrap();
            if let Some((n, m)) = m.inspect() {
                let dest = monkes.get_mut(&n).unwrap();
                dest.items.push_back(m);
                i += 1;
            }
             else {
                 curr += 1;
                 curr %= n_monke;
             }

        }

    for (k, v) in monkes.iter() {
        println!("{k}, {}", v.n_inspected);
    }

    Ok(())
}
