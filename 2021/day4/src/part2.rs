use std::fmt::Write;
use std::io::{self, BufRead, Read};
use std::str::FromStr;
use std::collections::HashSet;

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

impl FromStr for BingoBoard {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = Self::new(&s)?;
        Ok(x)
    }

}



impl BingoBoard {
    pub fn new(input: &str) -> Result<Self, std::num::ParseIntError> {
        let board: Vec<_> = input.lines().map(|line| {

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

        let x = Self { board };

        Ok(x)
    }

    pub fn process_one(&mut self, n: usize) {
        for line in self.board.iter_mut(){
            line.iter_mut().for_each(|c| c.process(n));
        }
    }

    pub fn process_many(&mut self, nums: &[usize]) {
        nums.iter().for_each(|&n| self.process_one(n));
    }

    pub fn is_winner(&self) -> bool {
        for row in self.board.iter() {
            if row.iter().all(BingoCell::is_marked) {
                return true;
            }
        }

        for i in 0..5 {
            if self.board.iter().map(|line| line.iter().nth(i).unwrap()).all(BingoCell::is_marked) {
                return true;
            }
        }

        // IF YOU COULD READ YOU WOULD HAVE SEEN THAT DIAGONALS DON'T COUNT DUMBASS
        // if (0..5).map(|i| &self.board[i][i]).all(BingoCell::is_marked) {
        //     return true;
        // }

        // if (0..5).map(|i| &self.board[i][4-i]).all(BingoCell::is_marked) {
        //     return true;
        // }

        false
    }


    pub fn is_winner_with_score(&mut self, n: usize) -> Option<usize> {
    
        self.process_one(n);

        if !self.is_winner() {
            return None;
        }


        let score: usize = self.board.iter().flatten().filter(|c| !c.is_marked()).map(|c| c.number).sum();

        println!("sum: {score}");

        let score = score * n;

        Some(score)

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

fn main() -> std::io::Result<()> {
    let mut input = String::new();

    std::io::stdin().read_to_string(&mut input)?;

    let mut input = input.split("\n\n");

    let nums: Vec<usize> = input.next().unwrap().split(",").flat_map(str::parse::<usize>).collect();

    dbg!(&nums);


    let mut boards : Vec<BingoBoard> = input.flat_map(|s| BingoBoard::from_str(s)).collect();


    let mut winning = HashSet::new();

    for n in nums.iter() {

        for (i, board) in boards.iter_mut().enumerate() {
            if winning.contains(&i) {
                continue;
            }
            if let Some(score) = board.is_winner_with_score(*n) {
                println!("score: {score} | winning number: {n} | winning board: {i}");
                println!("{board}");
                println!();
                winning.insert(i);
            }
        }
    }


    Ok(())
}


