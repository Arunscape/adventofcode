mod point;
use point::Point;
use std::io::{self, BufRead};

enum Direction {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

impl From<String> for Direction {
    fn from(s: String) -> Self {
        let direction = s.chars().next().unwrap();
        let n = s[1..].parse().unwrap();

        match direction {
            'N' => Self::North(n),
            'S' => Self::South(n),
            'E' => Self::East(n),
            'W' => Self::West(n),
            'L' => Self::Left(n),
            'R' => Self::Right(n),
            'F' => Self::Forward(n),
            _ => unreachable!(),
        }
    }
}

struct Ship {
    direction: Direction,
    position: Point,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            direction: Direction::East(0),
            position: Point(0, 0),
        }
    }

    pub fn sail(&mut self, direction: Direction) -> Point {
        match direction {
            Direction::North(n) => self.position += Point(0, 1) * n,
            Direction::South(n) => self.position += Point(0, -1) * n,
            Direction::East(n) => self.position += Point(1, 0) * n,
            Direction::West(n) => self.position += Point(-1, 0) * n,
            Direction::Left(n) => match n {
                90 => match self.direction {
                    Direction::North(_) => self.direction = Direction::West(0),
                    Direction::South(_) => self.direction = Direction::East(0),
                    Direction::West(_) => self.direction = Direction::South(0),
                    Direction::East(_) => self.direction = Direction::North(0),
                    _ => unreachable!(),
                },
                180 => match self.direction {
                    Direction::North(_) => self.direction = Direction::South(0),
                    Direction::South(_) => self.direction = Direction::North(0),
                    Direction::West(_) => self.direction = Direction::East(0),
                    Direction::East(_) => self.direction = Direction::West(0),
                    _ => unreachable!(),
                },
                270 => match self.direction {
                    Direction::North(_) => self.direction = Direction::East(0),
                    Direction::South(_) => self.direction = Direction::West(0),
                    Direction::West(_) => self.direction = Direction::North(0),
                    Direction::East(_) => self.direction = Direction::South(0),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            Direction::Right(n) => match n {
                90 => match self.direction {
                    Direction::North(_) => self.direction = Direction::East(0),
                    Direction::South(_) => self.direction = Direction::West(0),
                    Direction::West(_) => self.direction = Direction::North(0),
                    Direction::East(_) => self.direction = Direction::South(0),
                    _ => unreachable!(),
                },
                180 => match self.direction {
                    Direction::North(_) => self.direction = Direction::South(0),
                    Direction::South(_) => self.direction = Direction::North(0),
                    Direction::West(_) => self.direction = Direction::East(0),
                    Direction::East(_) => self.direction = Direction::West(0),
                    _ => unreachable!(),
                },
                270 => match self.direction {
                    Direction::North(_) => self.direction = Direction::West(0),
                    Direction::South(_) => self.direction = Direction::East(0),
                    Direction::West(_) => self.direction = Direction::South(0),
                    Direction::East(_) => self.direction = Direction::North(0),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            Direction::Forward(n) => match self.direction {
                Direction::North(_) => self.position += Point(0, 1) * n,
                Direction::South(_) => self.position += Point(0, -1) * n,
                Direction::East(_) => self.position += Point(1, 0) * n,
                Direction::West(_) => self.position += Point(-1, 0) * n,
                _ => unreachable!(),
            },
        }
        self.position
    }
}

fn main() {
    let mut ship = Ship::new();

    for line in io::stdin().lock().lines().flatten() {
        let direction = Direction::from(line);
        ship.sail(direction);
    }

    let ans = Point::manhattan_distance(Point(0, 0), ship.position);
    println!("{}", ans);
}
