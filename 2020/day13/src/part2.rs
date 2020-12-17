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
        .filter(|&(_, s)| s != "x")
        .map(|(p, s)| (p, s.parse().unwrap()))
        .collect();

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

    // told by the problem to start search here
    let mut time = 100000000000000;
    let mut increment = 1;

    // sieve is faster if n1 > n2 > .. > nk
    for (n, id) in busses.into_iter().rev() {
        while (time + n) % id != 0 {
            time += increment
        }
        increment *= id;
    }

    println!("{}", time);
}
