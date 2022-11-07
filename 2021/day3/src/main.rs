use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut zeros_count = HashMap::new();
    let mut ones_count = HashMap::new();
    for line in io::stdin().lock().lines().flatten() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '0' => {
                    zeros_count.entry(i)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                },
                '1' => {
                    ones_count.entry(i)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                },
                _ => unreachable!(),
            }
        }
    }

    dbg!(&zeros_count);
    dbg!(&ones_count);


    let len = *zeros_count.keys().max().unwrap();

    dbg!(&len);

    let mut gamma = vec!['0'; len+1];
    let mut epsilon = vec!['0'; len+1];
    
    for i in 0..=len {
        if zeros_count[&i] == ones_count[&i] {
            panic!("UHHH I WASN'T EXPECTING THIS");
        } else if zeros_count[&i] < ones_count[&i] {
            epsilon[i] = '0';
            gamma[i] = '1';
        } else {
            gamma[i] = '0';
            epsilon[i] = '1';

        }
    }


    dbg!(&gamma);
    dbg!(&epsilon);

    let gamma: String = gamma.iter().collect();
    let epsilon: String = epsilon.iter().collect();


    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();

    println!("part1 | gamma: {gamma} | epsilon: {epsilon} | gamma*epsilon= {}", gamma*epsilon);

}
