use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug)]
struct Game {
    hashmap: HashMap<usize, Vec<usize>>,
    last_said: usize,
    index: usize,
}

impl Game {
    pub fn new(starting_nums: &[usize]) -> Self {
        let mut hashmap = HashMap::new();
        let index = starting_nums.len() - 1;
        let last_said = starting_nums[index];

        // assuming no repeats in starting_nums
        for (idx, &num) in starting_nums.iter().enumerate() {
            let v = vec![idx];
            hashmap.insert(num, v);
        }

        Self {
            hashmap,
            last_said,
            index,
        }
    }

    fn first_time_last_has_been_spoken(&self) -> bool {
        if let Some(v) = self.hashmap.get(&self.last_said) {
            return v.len() == 1;
        }
        false
    }
}

impl Iterator for Game {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        dbg!(&self);
        println!("Turn {}", self.index);
        if self.first_time_last_has_been_spoken() {
            self.index += 1;
            let v = self.hashmap.get_mut(&0).unwrap();
            v.push(self.index);
            println!(
                "{} was first spoken at index {}, saying 0",
                self.last_said,
                v[v.len() - 2]
            );
            self.last_said = 0;
            return Some(0);
        }

        // the number has been spoken before

        let previously_spoken_indexes = self.hashmap.get_mut(&self.last_said).unwrap();
        let last_said_index = previously_spoken_indexes[previously_spoken_indexes.len() - 2];

        self.last_said = self.index - last_said_index;
        println!("{} had been spoken before", self.last_said);
        println!("current index: {}", self.index);
        println!("last spoken index: {}", last_said_index);
        self.index += 1;
        if let Some(v) = self.hashmap.get_mut(&self.last_said) {
            v.push(self.index);
        } else {
            self.hashmap.insert(self.last_said, vec![self.index]);
        }
        Some(dbg!(self.last_said))
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input)?;

    let starting_nums: Vec<_> = input
        .trim_end()
        .split(',')
        .flat_map(str::parse::<usize>)
        .collect();

    let mut game = Game::new(dbg!(&starting_nums));

    let ans = game.nth(10 - starting_nums.len()).unwrap();

    println!("{}", ans);
    Ok(())
}
