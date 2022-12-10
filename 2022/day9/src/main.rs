use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Rope {
    head: (isize, isize),
    tail: (isize, isize),
    tail_visited: HashSet<(isize, isize)>,
}

impl Rope {
    pub fn new() -> Self {
        let head = (0, 0);
        let tail = (0, 0);
//        let mut tail_visited = HashSet::new();
        let tail_visited = HashSet::new();
//        tail_visited.insert(tail);

        Self {
            head,
            tail,
            tail_visited,
        }
    }

    pub fn move_head(&mut self, direction: Direction, n: usize) -> (isize, isize) {
        if n < 1 {
            return self.head;
        }
        let (ref mut x, ref mut y) = self.head;
        let (ref mut i, ref mut j) = self.tail;
        match direction {
            Direction::Up => *y += 1,
            Direction::Down => *y -= 1,
            Direction::Right => *x += 1,
            Direction::Left => *x -= 1,
        };

        let (dy, dx) = (*y - *j, *x - *i);

    
        // todo rethink this
        self.tail_visited.insert(self.tail);

        self.move_head(direction, n - 1);

        self.head
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

    let mut rope = Rope::new();

    //dbg!(&rope);
    for &(d, n) in input.iter() {
        rope.move_head(d, n);
        //dbg!(&rope);
    }

    let p1 = rope.tail_visited.len();

    println!("part 1: {p1}");
}
