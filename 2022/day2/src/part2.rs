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

fn get_losing(other: Hand) -> Hand {
    match other {
        Hand::Rock => Hand::Scissors,
        Hand::Paper => Hand::Rock,
        Hand::Scissors => Hand::Paper,
    }
}

fn get_winning(other: Hand) -> Hand {
    match other {
        Hand::Rock => Hand::Paper,
        Hand::Paper => Hand::Scissors,
        Hand::Scissors => Hand::Rock,
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
            "X" => Ok(get_losing(other)),
            "Y" => Ok(other),
            "Z" => Ok(get_winning(other)),
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
        .filter_map(|game_str| Game::from_str(&game_str).ok())
        .map(|game| game.score())
        .sum();

    println!("part 2: {ans}");
}
