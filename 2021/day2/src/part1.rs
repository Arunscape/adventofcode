use std::io::{self, BufRead};

fn main() {

    let (mut x, mut depth) = (0, 0);

    for line in io::stdin().lock().lines().flatten() {
        let (direction, num) = line.split_once(' ').unwrap();
        let num: usize = num.parse().unwrap();

        match direction {
            "forward" => x += num,
            "down" => depth += num,
            "up" => depth -= num,
            _ => unreachable!(),
        }
    }

    println!("Part 1: {}", x * depth);
}
