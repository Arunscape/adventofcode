use std::collections::HashMap;
use std::io::{self, BufRead};
use std::ops::Range;

type Point = (isize, isize);

#[derive(Copy, Clone, Debug)]
enum Block {
    Sensor(isize),
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

    pub fn insert(&mut self, sensor_coord: Point, closest_beacon: Point) {
        let md = Self::manhattan_distance(sensor_coord, closest_beacon);

        self.grid.insert(closest_beacon, Block::Beacon);
        self.grid.insert(sensor_coord, Block::Sensor(md));
    }

    #[deprecated]
    pub fn insert_and_mark(&mut self, sensor_coord: Point, closest_beacon: Point) {
        let md = Self::manhattan_distance(sensor_coord, closest_beacon);

        self.grid.insert(closest_beacon, Block::Beacon);
        self.grid.insert(sensor_coord, Block::Sensor(md));

        for &point in self
            .points_within_manhattan_distance_of(sensor_coord, md)
            .iter()
        {
            self.grid.entry(point).or_insert(Block::Empty);
        }
    }

    pub fn sensor_in_range(&self, point: Point) -> bool {
        for (&coord, &sensor) in self.grid.iter() {
            match sensor {
                Block::Sensor(dist) => {
                    if Self::manhattan_distance(coord, point) <= dist {
                        return true;
                    }
                }
                _ => continue,
            }
        }
        false
    }

    pub fn manhattan_distance((x1, y1): Point, (x2, y2): Point) -> isize {
        (x1 - x2).abs() + (y1 - y2).abs()
    }

    #[deprecated]
    pub fn points_within_manhattan_distance_of(
        &self,
        (x, y): Point,
        distance: isize,
    ) -> Vec<Point> {
        let (minx, maxx) = (x - distance, x + distance);
        let (miny, maxy) = (y - distance, y + distance);

        let v = (minx..=maxx)
            .flat_map(|xx| (miny..=maxy).map(move |yy| (xx, yy)))
            .filter(|&point| Self::manhattan_distance((x, y), point) <= distance)
            .collect();
        v
    }

    pub fn count_non_beacon_spaces_in_row(&self, y: isize) -> usize {
        let (minx, maxx) = self.x_range();

        (minx..=maxx)
            .map(|x| (x, y))
            .filter(|&point| self.sensor_in_range(point))
            .filter(|point| !self.grid.get(point).is_some())
            .count()
    }

    pub fn x_range(&self) -> (isize, isize) {
        let minx = self
            .grid
            .iter()
            .map(|(&(x, _), &block)| match block {
                Block::Sensor(d) => x - d,
                _ => x,
            })
            .min()
            .unwrap();
        let maxx = self
            .grid
            .iter()
            .map(|(&(x, _), &block)| match block {
                Block::Sensor(d) => x + d,
                _ => x,
            })
            .max()
            .unwrap();

        (minx, maxx)
    }

    pub fn find_distress_beacon(
        &self,
        min: isize,
        max: isize,
    ) -> Option<(Point, isize)> {
        let point = (min..=max)
            .flat_map(|x| (min..=max).map(move |y| (x, y)))
            .find(|point| !self.grid.contains_key(point) && !self.sensor_in_range(*point))?;

        let (x, y) = point;
        let tuning_freq = x * 4000000 + y;

        Some((point, tuning_freq))
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
        grid.insert(sensor, beacon);
    }

    let part1_testinput = grid.count_non_beacon_spaces_in_row(10);
    dbg!(part1_testinput);
    let part1 = grid.count_non_beacon_spaces_in_row(2000000);
    dbg!(part1);


    let part2_testinput = grid.find_distress_beacon(0, 20);
    dbg!(part2_testinput);
    let part2 = grid.find_distress_beacon(0, 4000000);
    dbg!(part2);
}
