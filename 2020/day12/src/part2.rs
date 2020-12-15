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
    position: Point,
    waypoint: Point,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            position: Point(0, 0),
            waypoint: Point(10, 1),
        }
    }

    pub fn sail(&mut self, direction: Direction) -> Point {
        match direction {
            Direction::North(n) => self.waypoint += Point(0, 1) * n,
            Direction::South(n) => self.waypoint += Point(0, -1) * n,
            Direction::East(n) => self.waypoint += Point(1, 0) * n,
            Direction::West(n) => self.waypoint += Point(-1, 0) * n,
            Direction::Left(n) => self.waypoint = self.waypoint.rotate(n as f64),
            Direction::Right(n) => self.waypoint = self.waypoint.rotate(-n as f64),
            Direction::Forward(n) => self.position += self.waypoint * n,
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
