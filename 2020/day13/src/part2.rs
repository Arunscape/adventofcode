use std::collections::BTreeMap;
use std::io::{self, BufRead};

fn main() {
    let busses: BTreeMap<usize, usize> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .map(|(p, s)| (p, s.parse().unwrap_or(0)))
        .filter(|(_, n)| n > &0)
        .collect();

    let x = 5_usize.div_euclid(3);

    dbg!(x);

    // turns out the bus ids are prime numbers
    // something Chinese Remainder Theorem
    // something pairwise coprime
    //
    // let n1..nk be integers greater than 1 (the input I guess, and pairwise coprime)
    // let N = (n1..nk).product()
    // let a1..ak be integers such that 0<= ai <= ni for every i
    // then there is an x such that 0<=x < N
    // and the remainder of the Euclidean division of x by ni is ai for every i
    //
    // i.e. x = ai (mod ni)

    let product: usize = busses.iter().map(|(id, _)| id).product();
    // told by the problem to start search here
    let mut r: std::ops::Range<usize> = 100000000000000..product;

    // sieve is faster if n1 > n2 > .. > nk
    for (&id, &n) in busses.iter().rev() {}
}
