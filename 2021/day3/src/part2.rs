use std::collections::HashMap;
use std::io::{self, BufRead};

fn sum_column(input: &[Vec<u32>], i: usize) -> u32 {
   input.iter().map(|line| line.iter().nth(i).unwrap()).sum()
}


fn main() {
    let input: Vec<String> = io::stdin().lock().lines().flatten().collect();

    let input: Vec<Vec<_>> = input.iter().map(|line| {
        line.chars().filter_map(|c| c.to_digit(2)).collect()
    }).collect();

    let width = input[0].len();

    let oxygen = {
        let mut input = input.clone();

        for i in 0..width {
            let ones = sum_column(&input, i);
            let zeros = input.len() as u32 - ones;

            let x: u32 = if zeros <= ones { 1 } else { 0 };
            println!("\ni: {i} | zeros: {zeros} | ones: {ones} | x: {x} | height: {}", input.len());
            input = input.iter().cloned().filter(|l| *l.iter().nth(i).unwrap() == x).collect();
            dbg!(&input);
            if input.len() == 1 {
                break;
            }
        }

        input[0].clone()
    };


    dbg!(&oxygen);
    
    let co2 = {
        let mut input = input.clone();

        for i in 0..width {
            let ones = sum_column(&input, i);
            let zeros = input.len() as u32 - ones;

            let x: u32 = if zeros <= ones { 0 } else { 1 };
            println!("\ni: {i} | zeros: {zeros} | ones: {ones} | x: {x} | height: {}", input.len());
            input = input.iter().cloned().filter(|l| *l.iter().nth(i).unwrap() == x).collect();
            dbg!(&input);
            if input.len() == 1 {
                break;
            }
        }

        input[0].clone()
    };

    dbg!(&co2);


    let oxygen: u32 = oxygen.iter().rev().enumerate().map(|(place, bit)| bit << place).map(|x| x as u32).sum();
    let co2: u32 = co2.iter().rev().enumerate().map(|(place, bit)| bit << place).map(|x| x as u32).sum();


    dbg!(&oxygen);
    dbg!(&co2);

    println!("oxygen*co2={}", oxygen*co2);

}
