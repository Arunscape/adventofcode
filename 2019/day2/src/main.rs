#![feature(slice_patterns)]
#![feature(unsized_locals)]
#![feature(array_chunks)]

use std::any::Any;
use std::io::{self, BufRead, Read};

fn main() -> std::io::Result<()> {
    let mut numstr = String::new();

    io::stdin().read_to_string(&mut numstr)?;

    let mut nums: Vec<usize> = numstr
        //.split("99")
        //.take(1)
        //.collect::<String>()
        .split(',')
        .map(|s| s.parse::<usize>())
        .flatten()
        .collect();

    println!("{:?}", nums);

    /*
    nums.chunks_exact(4)
        .take_while(|&a| a[0] != 99)
        //.map(|a| (a[0], a[1], a[2], a[3]))
        .for_each(|a| match a[0] {
            1 => nums[a[3]] = nums[a[1]] + nums[a[2]],
            2 => nums[a[3]] = nums[a[1]] * nums[a[2]],
            _ => panic!("unrecognized opcode"),
        });
    */
    let mut nums2 = nums.clone();

    for &[op, n1, n2, dest] in nums.array_chunks::<4>() {
        match op {
            99 => break,
            1 => nums2[dest] = nums2[n1] + nums2[n2],
            2 => nums2[dest] = nums2[n1] * nums2[n2],
            _ => panic!("unrecognized opcode"),
        }
    }

    println!("{:?}", nums2);

    Ok(())
}
