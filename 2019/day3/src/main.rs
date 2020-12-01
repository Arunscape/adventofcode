#![feature(unsized_locals)]

#[derive(PartialEq, Clone, Debug)]
struct Point(isize, isize);

fn manhattan_distance(p1: Point, p2: Point) -> isize {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

#[derive(Debug)]
struct Wire {
    points: Vec<Point>,
}

impl Wire {
    pub fn from(input: &str) -> Self {
        let mut points = vec![Point(0, 0)];
        for s in input.split(",") {
            let direction = match s.chars().nth(0).unwrap() {
                'R' => Point(1, 0),
                'L' => Point(-1, 0),
                'U' => Point(0, 1),
                'D' => Point(0, -1),
                _ => unreachable!(),
            };
            for i in 1..=s[1..].parse().unwrap() {
                points.push(Point(direction.0 * i, direction.1 * i))
            }
        }

        Self { points }
    }

    pub fn intersect(&self, other: &Self) -> Vec<Point> {
        self.points
            .iter()
            .filter(|p| other.points.contains(p))
            .cloned()
            .collect()
    }
}

fn main() {
    let wire1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
    let wire2 = "U62,R66,U55,R34,D71,R55,D58,R83";

    let wire1 = Wire::from(wire1);
    let wire2 = Wire::from(wire2);

    let intersections = wire1.intersect(&wire2);

    println!("{:#?}", intersections);

    let max_distance = intersections
        .into_iter()
        .map(|p| manhattan_distance(p, Point(0, 0)))
        .max()
        .unwrap();

    println!("max distance: {}", max_distance);
}
