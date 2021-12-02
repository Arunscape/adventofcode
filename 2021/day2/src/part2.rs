use std::io::{self, BufRead};

fn main() {
    let (mut x, mut depth, mut aim) = (0, 0, 0);

    for line in io::stdin().lock().lines().flatten() {
        let (direction, num) = line.split_once(' ').unwrap();
        let num: usize = num.parse().unwrap();

        match direction {
            "forward" => {
                x += num;
                depth += aim * num;
            }
            "down" => aim += num,
            "up" => aim -= num,
            _ => unreachable!(),
        }
    }

    println!("Part 2: {}", x * depth);
}
