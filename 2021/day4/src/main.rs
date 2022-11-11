use std::fmt::Write;

#[derive(Debug)]
struct BingoBoard {
    board: [[BingoCell; 5]; 5],
}

impl std::fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();

        for line in self.board.iter() {
            let mut lit = line.iter().peekable();
            while let Some(c) = lit.next() {
                if lit.peek().is_none() {
                    write!(s, "{c}\n")?;
                } else {
                    write!(s, "{c} ")?;
                }
            }
        }
        write!(f, "{s}")
    }
}



impl BingoBoard {
    pub fn new(input: &[&str; 5]) -> Self {
        let board: Vec<_> = input.iter().map(|line| {

            let l: Vec<BingoCell> = line
                .split_whitespace()
                //.flat_map(|c| {
                //    let c: Result<usize, _> = c.parse();
                //    c
                //})
                .flat_map(str::parse::<usize>)
                .map(BingoCell::from)
                .collect();

            let l: [BingoCell; 5] = l
                .try_into()
                .unwrap();
            l
        }).collect();

        let board = board.try_into().unwrap();

        Self { board }
    }

    pub fn process_one(&mut self, n: usize) {
        for line in self.board.iter_mut(){
            line.iter_mut().for_each(|c| c.process(n));
        }
    }

    pub fn process_many(&mut self, nums: &[usize]) {
        nums.iter().for_each(|&n| self.process_one(n));
    }
}

struct BingoCell {
    number: usize,
    marked: bool,
}

impl std::fmt::Display for BingoCell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.marked {
            write!(f, "\x1b[41m {:2} \x1b[0m", self.number)
        } else {
            write!(f, "{:2}", self.number)
        }
    }
}

impl std::fmt::Debug for BingoCell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl BingoCell {
    pub fn is_marked(&self) -> bool {
        self.marked
    }

    pub fn process(&mut self, number: usize) {
        if self.number == number {
            self.marked = true
        }
    }
}

impl From<usize> for BingoCell {
    fn from(number: usize) -> Self {
        Self {
            number,
            marked: false,
        }
    }
}

fn main() {
    let input = [
        "22 13 17 11  0",
        " 8  2 23  4 24",
        "21  9 14 16  7",
        " 6 10  3 18  5",
        " 1 12 20 15 19",
    ];


    let b = BingoBoard::new(&input);

    println!("{b}");

    println!("Hello, world!");
}
