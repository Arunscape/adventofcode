use ndarray::Array2;

use std::io::{self, BufRead};

#[derive(Copy, Clone, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl std::fmt::Debug for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Floor => '.',
            Self::Empty => 'L',
            Self::Occupied => '#',
        };
        write!(f, "{}", c)
    }
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

#[derive(PartialEq)]
struct Plane {
    seats: Array2<Seat>,
}

impl std::fmt::Debug for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.seats.genrows() {
            for c in row {
                write!(f, "{:?}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
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
        match seat {
            Seat::Empty => {
                if self.adjacent(position) == 0 {
                    new_plane.seats[position] = Seat::Occupied;
                }
            }
            Seat::Occupied => {
                if self.adjacent(position) >= 5 {
                    new_plane.seats[position] = Seat::Empty;
                }
            }
            _ => {}
        }
    }

    fn adjacent(&self, position: (usize, usize)) -> usize {
        let (mut x, mut y) = position;
        let mut count = 0;

        // northwest
        if x != 0 && y != 0 {
            x -= 1;
            y -= 1;
            while let Some(s) = self.seats.get((x, y)) {
                match s {
                    Seat::Occupied => {
                        count += 1;
                        break;
                    }
                    Seat::Empty => break,
                    _ => {}
                };
                if x == 0 || y == 0 {
                    break;
                }
                x -= 1;
                y -= 1;
            }
        }

        // west and southwest
        x = position.0;
        y = position.1;
        if x != 0 {
            x -= 1;
            while let Some(s) = self.seats.get((x, y)) {
                match s {
                    Seat::Occupied => {
                        count += 1;
                        break;
                    }
                    Seat::Empty => break,
                    _ => {}
                };
                if x == 0 {
                    break;
                }
                x -= 1;
            }
            x = position.0 - 1;
            y = position.1 + 1;
            while let Some(s) = self.seats.get((x, y)) {
                match s {
                    Seat::Occupied => {
                        count += 1;
                        break;
                    }
                    Seat::Empty => break,
                    _ => {}
                };
                if x == 0 {
                    break;
                }
                x -= 1;
                y += 1;
            }
        }

        // north and northeast
        x = position.0;
        y = position.1;
        if y != 0 {
            y -= 1;
            while let Some(s) = self.seats.get((x, y)) {
                match s {
                    Seat::Occupied => {
                        count += 1;
                        break;
                    }
                    Seat::Empty => break,
                    _ => {}
                };
                if y == 0 {
                    break;
                }
                y -= 1;
            }
            x = position.0 + 1;
            y = position.1 - 1;
            while let Some(s) = self.seats.get((x, y)) {
                match s {
                    Seat::Occupied => {
                        count += 1;
                        break;
                    }
                    Seat::Empty => break,
                    _ => {}
                };
                if y == 0 {
                    break;
                }
                x += 1;
                y -= 1;
            }
        }

        // east
        x = position.0 + 1;
        y = position.1;
        while let Some(s) = self.seats.get((x, y)) {
            match s {
                Seat::Occupied => {
                    count += 1;
                    break;
                }
                Seat::Empty => break,
                _ => {}
            };
            x += 1;
        }

        // southeast
        x = position.0 + 1;
        y = position.1 + 1;
        while let Some(s) = self.seats.get((x, y)) {
            match s {
                Seat::Occupied => {
                    count += 1;
                    break;
                }
                Seat::Empty => break,
                _ => {}
            };
            x += 1;
            y += 1;
        }

        // south
        x = position.0;
        y = position.1 + 1;
        while let Some(s) = self.seats.get((x, y)) {
            match s {
                Seat::Occupied => {
                    count += 1;
                    break;
                }
                Seat::Empty => break,
                _ => {}
            };
            y += 1;
        }

        count
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
