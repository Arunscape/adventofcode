use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let input: Vec<_> = stdin.lock().lines().flatten().enumerate().collect();

    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    let ans: usize = slopes
        .iter()
        .map(|&slope| ski(input.iter(), slope))
        .product();

    println!("{}", ans)
}

fn ski<'a>(it: impl Iterator<Item = &'a (usize, String)>, (rise, run): (usize, usize)) -> usize {
    it.step_by(rise)
        .filter(|&(lineno, line)| line.chars().cycle().nth(lineno / rise * run).unwrap() == '#')
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! ski_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
    let input: Vec<_> = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
        .lines()
        .map(|s| s.to_string())
        .enumerate()
        .collect();

                let (expected, slope) = $value;
                assert_eq!(expected, ski(input.iter(), slope));
            }
        )*
    }
    }

    ski_tests!(
    ski0: (2, (1,1)),
    ski1: (7, (1,3)),
    ski2: (3, (1,5)),
    ski3: (4, (1,7)),
    ski4: (2, (2,1)),
    );
}
