use std::collections::HashMap;
use std::io::{self, BufRead};

type Point = (isize, isize);

#[derive(Copy, Clone, Debug)]
enum Block {
    Sensor,
    Beacon,
    Empty,
}

#[derive(Debug)]
struct Grid {
    grid: HashMap<Point, Block>,
}

impl Grid {
    pub fn new() -> Self {
        let grid = HashMap::new();

        Self { grid }
    }

    pub fn insert_and_mark(&mut self, sensor_coord: Point, closest_beacon: Point) {
        let md = Self::manhattan_distance(sensor_coord, closest_beacon);

        self.grid.insert(closest_beacon, Block::Beacon);
        self.grid.insert(sensor_coord, Block::Sensor);

        for &point in self.within_manhattan_distance_of(sensor_coord, md).iter() {

            self.grid.entry(point).or_insert(Block::Empty);

        }
    }
    pub fn manhattan_distance((x1, y1): Point, (x2, y2): Point) -> isize {
        (x1 - x2).abs() + (y1 - y2).abs()
    }

    pub fn within_manhattan_distance_of(&self, (x, y): Point, distance: isize) -> Vec<Point> {
        let (minx, maxx) = (x - distance, x + distance);
        let (miny, maxy) = (y - distance, y + distance);

        let v = (minx..=maxx)
            .flat_map(|xx| (miny..=maxy).map(move |yy| (xx, yy)))
            .filter(|&point| Self::manhattan_distance((x, y), point) <= distance)
            .collect();
        v
    }
}

fn main() {
    let input: Vec<_> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .filter_map(|line| {
            let (sensor, beacon) = line.split_once(": closest beacon is at x=")?;

            let sensor = sensor.strip_prefix("Sensor at x=")?;
            let (sx, sy) = sensor.split_once(", y=")?;
            let sensor: Point = (sx.parse().ok()?, sy.parse().ok()?);

            let (bx, by) = beacon.split_once(", y=")?;
            let beacon: Point = (bx.parse().ok()?, by.parse().ok()?);

            Some((sensor, beacon))
        })
        .collect();

    let mut grid = Grid::new();

    for &(sensor, beacon) in input.iter() {
        grid.insert_and_mark(sensor, beacon);
    }

    let part1_testinput = grid
        .grid
        .iter()
        .filter(|(&(_, y), block)| y == 10 && matches!(block, Block::Empty | Block::Sensor))
        .count();
    dbg!(part1_testinput);

    let part1 = grid
        .grid
        .iter()
        .filter(|(&(_, y), block)| y == 2000000 && matches!(block, Block::Empty | Block::Sensor))
        .count();
    dbg!(part1);
}
