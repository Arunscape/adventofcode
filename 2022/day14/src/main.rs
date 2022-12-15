use std::collections::HashMap;
use std::io::{self, BufRead};

type Point = (usize, usize);

// https://stackoverflow.com/a/70352626
fn range_inclusive(a: usize, b: usize) -> impl Iterator<Item = usize> {
    let x: Box<dyn Iterator<Item = usize>>;
    if b > a {
        x = Box::new(a..=b)
    } else {
        x = Box::new((b..=a).rev())
    }
    x
}

#[derive(Debug)]
enum Block {
    Sand,
    Rock,
}

#[derive(Debug)]
struct Fall {
    source: Point,
    grid: HashMap<Point, Block>,
    max_depth: usize,
    n_sand: usize,
}

impl Fall {
    pub fn new(input: &Vec<Vec<Point>>) -> Self {
        let mut grid = HashMap::new();
        for line in input.iter() {
            let mut line = line.windows(2);
            while let Some(&[(x, y), (i, j)]) = line.next() {
                // dbg!(&[(x, y), (i, j)]);
                for n in range_inclusive(y, j) {
                    for m in range_inclusive(x, i) {
                        // dbg!((m, n));
                        grid.insert((m, n), Block::Rock);
                    }
                }
            }
        }

        let source = (500, 0);

        let n_sand = 0;

        let max_depth = *grid.keys().map(|(_, y)| y).max().unwrap();

        Self {
            source,
            grid,
            max_depth,
            n_sand,
        }
    }
    pub fn fall(&mut self, part_two: bool) -> Option<Point> {
        let mut curr_point = self.source;
        while let Some(new_point) = self.can_fall(curr_point, part_two) {
            let max_depth = if part_two {
                self.max_depth + 2
            } else {
                self.max_depth
            };
            if max_depth < new_point.1 {
                // overflow of sand
                return None;
            }

            if new_point == self.source {
                break;
            }
            curr_point = new_point;
        }
        self.n_sand += 1;
        self.grid.insert(curr_point, Block::Sand);
        if curr_point == self.source {
            return None;
        }
        Some(curr_point)
    }

    pub fn can_fall(&self, sand: Point, part_two: bool) -> Option<Point> {
        // down
        let down = Self::down(sand);
        self.can_move_to(down, part_two)
            .or_else(|| {
                let down_left = Self::down_left(sand);
                self.can_move_to(down_left, part_two)
            })
            .or_else(|| {
                let down_right = Self::down_right(sand);
                self.can_move_to(down_right, part_two)
            })
    }
    pub fn down((x, y): Point) -> Point {
        (x, y + 1)
    }

    pub fn down_left((x, y): Point) -> Point {
        (x - 1, y + 1)
    }

    pub fn down_right((x, y): Point) -> Point {
        (x + 1, y + 1)
    }

    pub fn can_move_to(&self, p: Point, part_two: bool) -> Option<Point> {
        if part_two && p.1 > self.max_depth + 1 {
            return None;
        }
        if self.grid.get(&p).is_none() {
            Some(p)
        } else {
            None
        }
    }

    pub fn visualize(&self) {
        let minx = *self.grid.keys().map(|(x, _)| x).min().unwrap();
        let maxx = *self.grid.keys().map(|(x, _)| x).max().unwrap();
        let miny = *self.grid.keys().map(|(_, y)| y).min().unwrap();
        let maxy = *self.grid.keys().map(|(_, y)| y).max().unwrap();

        let mut s = String::new();
        for y in range_inclusive(miny, maxy) {
            for x in range_inclusive(minx, maxx) {
                match self.grid.get(&(x, y)) {
                    Some(Block::Rock) => s.push('#'),
                    Some(Block::Sand) => s.push('o'),
                    _ => s.push('.'),
                }
            }
            s.push('\n')
        }
        // println!();
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{s}");
        // print it all at once to prevent flickering in the visualization
    }
}

fn main() {
    let input: Vec<Vec<Point>> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| {
            let v: Vec<Point> = line
                .split(" -> ")
                .filter_map(|p| p.split_once(","))
                .filter_map(|(a, b)| {
                    let a: usize = a.parse().ok()?;
                    let b: usize = b.parse().ok()?;
                    Some((a, b))
                })
                .collect();
            v
        })
        .collect();

    let mut grid = Fall::new(&input);

    while let Some(_) = grid.fall(false) {
        grid.visualize();
    }

    println!("part 1: {}", grid.n_sand);
    dbg!(&grid.n_sand);

    let mut grid = Fall::new(&input);

    while let Some(_) = grid.fall(true) {
        grid.visualize();
    }

    println!("part 2: {}", grid.n_sand);
}
