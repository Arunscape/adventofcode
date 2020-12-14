mod point;
use point::Point;

enum Direction {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
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
            Direction::Left(n) => todo!(),
            Direction::Right(n) => todo!(),
        }
        self.position
    }
}

fn main() {
    let ship = Ship::new();
}
