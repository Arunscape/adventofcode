use std::collections::HashSet;
use std::io::{self, BufRead};

type Point = (isize, isize);
#[derive(Debug)]
struct Rope {
    rope: Vec<Point>,
    tail_visited: HashSet<Point>,
}

impl Rope {
    pub fn new(length: usize) -> Self {
        assert!(2 <= length);
        let rope = vec![(0, 0); length];
        let tail_visited = HashSet::new();

        Self {
            rope,
            tail_visited,
        }
    }

    pub fn tail(&self) -> Point {
        *self.rope.last().unwrap()
    }

    pub fn head(&self) -> Point {
        self.rope[0]
    }

    pub fn move_head(&mut self, direction: Direction, n: usize) -> (isize, isize) {
        if n < 1 {
            return self.head();
        }
        let (ref mut x, ref mut y) = self.rope[0];
        match direction {
            Direction::Up => *y += 1,
            Direction::Down => *y -= 1,
            Direction::Right => *x += 1,
            Direction::Left => *x -= 1,
        };

        let tail = self.update_tail(1);
        self.tail_visited.insert(tail);
        
        self.move_head(direction, n - 1);

        self.head()
    }

    pub fn update_tail(&mut self, index: usize) -> Point {
        let (x, y) = self.rope[index-1];
        let (ref mut i, ref mut j) = self.rope[index];
        let (dy, dx) = (y - *j, x - *i);

        if 1 < dy.abs() || 1 < dx.abs() {
            *i += dx.signum();
            *j += dy.signum();
        }

        if index <  self.rope.len() - 1 {
            self.update_tail(index + 1);
        }
        
        //println!("head: {:?} tail: {:?}", self.head(), self.tail());

        self.tail()
    }

}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input: Vec<(Direction, usize)> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .filter_map(|line| {
            let (direction, num) = line.split_once(" ")?;

            let num = num.parse().ok()?;
            let d = match direction {
                "R" => Some(Direction::Right),
                "L" => Some(Direction::Left),
                "U" => Some(Direction::Up),
                "D" => Some(Direction::Down),
                _ => None,
            }?;

            Some((d, num))
        })
        .collect();

    let mut rope = Rope::new(2);

    for &(d, n) in input.iter() {
        rope.move_head(d, n);
    }

    let p1 = rope.tail_visited.len();
    println!("part 1: {p1}");


    let mut rope = Rope::new(10);
    for &(d, n) in input.iter() {
        rope.move_head(d, n);
    }
    let p2 = rope.tail_visited.len();
    println!("part 2: {p2}");
}
