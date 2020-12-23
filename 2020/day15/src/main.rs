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
        let index = starting_nums.len();
        let last_said = starting_nums[index - 1];

        // assuming no repeats in starting_nums
        for (idx, &num) in starting_nums.iter().enumerate() {
            let v = vec![idx + 1];
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
        self.index += 1;
        let index = self.index;
        if self.first_time_last_has_been_spoken() {
            self.hashmap
                .entry(0)
                .and_modify(|v| v.push(index))
                .or_insert_with(|| vec![index]);
            self.last_said = 0;
            return Some(0);
        }

        // the number has been spoken before

        let previously_spoken_indexes = self.hashmap.get_mut(&self.last_said).unwrap();
        let last_said_index = previously_spoken_indexes[previously_spoken_indexes.len() - 2];

        self.last_said = self.index - last_said_index - 1;
        self.hashmap
            .entry(self.last_said)
            .and_modify(|v| v.push(index))
            .or_insert_with(|| vec![index]);
        Some(self.last_said)
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

    let mut game = Game::new(&starting_nums);

    let ans = game.nth(30000000 - starting_nums.len() - 1).unwrap();

    println!("{}", ans);
    Ok(())
}
