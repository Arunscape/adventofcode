use std::str::FromStr;

use std::io::{self, BufRead};

#[derive(PartialEq, Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

struct Game {
    you: Hand,
    other: Hand,
}

impl Game {
    pub fn score(&self) -> usize {
        let mut score = match self.you {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        };

        // tie + 3
        if self.you == self.other {
            score += 3
        }

        // win + 6
        if let (Hand::Paper, Hand::Rock)
        | (Hand::Scissors, Hand::Paper)
        | (Hand::Rock, Hand::Scissors) = (self.you, self.other)
        {
            score += 6
        }

        score
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let (other, you) = s.split_once(" ").ok_or(())?;

        let other = match other {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissors),
            _ => Err(()),
        }?;

        let you = match you {
            "X" => Ok(Hand::Rock),
            "Y" => Ok(Hand::Paper),
            "Z" => Ok(Hand::Scissors),
            _ => Err(()),
        }?;

        let game = Self { other, you };

        Ok(game)
    }
}

fn main() {
    let ans: usize = io::stdin()
        .lock()
        .lines()
        .flatten()
        .filter_map(|game| Game::from_str(&game).ok())
        .map(|game| game.score())
        .sum();

    println!("part 1: {ans}");
}
