use std::io::{self, BufRead};

fn find_row(s: &str) -> u16 {
    s.chars()
        .map(|c| match c {
            'F' => 0,
            'B' => 1,
            _ => unreachable!(),
        })
        .fold(0, |acc, b| acc * 2 + b)
}

fn find_column(s: &str) -> u16 {
    s.chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => unreachable!(),
        })
        .fold(0, |acc, b| acc * 2 + b)
}

pub fn find_id(s: String) -> u16 {
    find_row(&s[..7]) * 8 + find_column(&s[7..])
}

fn main() {
    let ans = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(find_id)
        .max()
        .unwrap();
    println!("{}", ans)
}

#[cfg(test)]
mod test {
    use super::*;

    fn calculate(s: String) -> (u16, u16, u16) {
        (find_row(&s[..7]), find_column(&s[7..]), find_id(s))
    }

    #[test]
    fn test() {
        assert_eq!(calculate("FBFBBFFRLR".into()), (44, 5, 357));
        assert_eq!(calculate("BFFFBBFRRR".into()), (70, 7, 567));
        assert_eq!(calculate("FFFBBBFRRR".into()), (14, 7, 119));
        assert_eq!(calculate("BBFFBBFRLL".into()), (102, 4, 820));
    }
}
