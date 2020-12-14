use ndarray::Array2;

use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for Seat {
    fn from(item: char) -> Self {
        match item {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Plane {
    seats: Array2<Seat>,
}

impl From<Array2<Seat>> for Plane {
    fn from(item: Array2<Seat>) -> Self {
        Self { seats: item }
    }
}

impl Plane {
    fn process(&self) -> Self {
        let mut new_plane = Self {
            seats: self.seats.clone(),
        };

        for (position, seat) in self.seats.indexed_iter() {
            self.process_seat(position, seat, &mut new_plane);
        }
        new_plane
    }

    fn process_seat(&self, position: (usize, usize), seat: &Seat, new_plane: &mut Self) {
        let adjacent = self.adjacent(position);

        match seat {
            Seat::Empty => {
                if self
                    .adjacent(position)
                    .iter()
                    .filter(|&s| matches!(s, Seat::Occupied))
                    .count()
                    == 0
                {
                    new_plane.seats[position] = Seat::Occupied;
                }
            }
            Seat::Occupied => {
                if self
                    .adjacent(position)
                    .iter()
                    .filter(|&s| matches!(s, Seat::Occupied))
                    .count()
                    >= 4
                {
                    new_plane.seats[position] = Seat::Empty;
                }
            }
            _ => {}
        }
    }

    fn adjacent(&self, position: (usize, usize)) -> Vec<&Seat> {
        let (x, y) = position;
        [
            self.seats
                .get((usize::wrapping_sub(x, 1), usize::wrapping_sub(y, 1))),
            self.seats.get((usize::wrapping_sub(x, 1), y)),
            self.seats.get((usize::wrapping_sub(x, 1), y + 1)),
            self.seats.get((x, usize::wrapping_sub(y, 1))),
            self.seats.get((x, y + 1)),
            self.seats.get((x + 1, usize::wrapping_sub(y, 1))),
            self.seats.get((x + 1, y + 1)),
            self.seats.get((x + 1, y)),
        ]
        .iter()
        .flat_map(|&e| e)
        .collect()
    }
}

fn main() {
    let seats: Vec<String> = io::stdin().lock().lines().flatten().collect();

    let shape = (seats.len(), seats[0].len());

    let seats: Vec<Seat> = seats
        .iter()
        .flat_map(|line| line.chars().map(Seat::from).collect::<Vec<Seat>>())
        .collect();

    let seats = Array2::from_shape_vec(shape, seats).unwrap();

    let mut plane = Plane { seats };

    loop {
        let next_plane = plane.process();

        if next_plane == plane {
            let ans = plane
                .seats
                .iter()
                .filter(|&s| matches!(s, Seat::Occupied))
                .count();
            println!("{}", ans);
            return;
        }

        plane = next_plane;
    }
}
