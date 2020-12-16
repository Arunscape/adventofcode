use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock().lines().flatten();

    let time: usize = stdin.next().unwrap().parse().unwrap();

    let (id, wait) = stdin
        .next()
        .unwrap()
        .split(',')
        .flat_map(|s| s.parse())
        .map(|n| (n, (n..).step_by(n)))
        .map(|(n, r)| (n, r.skip_while(|&n| n <= time)))
        .map(|(n, mut m)| (n, m.next().unwrap() - time))
        .min_by(|(_, d1), (_, d2)| d1.cmp(d2))
        .unwrap();

    println!("{}", id * wait);
}
