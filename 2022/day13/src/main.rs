use serde_json::{json, Value};
use std::cmp::Ordering;
use std::io::{self, Read};

fn compare(a: &Value, b: &Value) -> Ordering {
    if let (Some(a), Some(b)) = (a.as_u64(), b.as_u64()) {
        a.cmp(&b)
    } else if let (Some(a), Some(b)) = (a.as_array(), b.as_array()) {
        if a.is_empty() || b.is_empty() {
            a.len().cmp(&b.len())
        } else {
            match compare(&a[0], &b[0]) {
                Ordering::Equal => compare(&json!(a[1..]), &json!(b[1..])),
                x => x,
            }
        }
    } else if let (Some(a), Some(b)) = (a.as_u64(), b.as_array()) {
        compare(&json!(vec![a]), &json!(b))
    } else if let (Some(a), Some(b)) = (a.as_array(), b.as_u64()) {
        compare(&json!(a), &json!(vec![b]))
    } else {
        Ordering::Greater
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    io::stdin().lock().read_to_string(&mut input)?;

    let pairs: Vec<(Value, Value)> = input
        .split("\n\n")
        .map(|pair| {
            let x: Vec<Value> = pair
                .lines()
                .filter_map(|line| serde_json::from_str(line).ok())
                .collect();
            (x[0].clone(), x[1].clone())
        })
        .collect();

    let p1: usize = pairs
        .iter()
        .map(|(pair1, pair2)| compare(&pair1, &pair2))
        .enumerate()
        .filter(|(_, p)| matches!(p, Ordering::Less))
        .map(|(i, _)| i + 1)
        .sum();


    println!("part 1: {p1}");

    let mut flat: Vec<Value> = pairs.iter().cloned().flat_map(|(a, b)| [a, b]).collect();
    let dp1 = json!([[2]]);
    let dp2 = json!([[6]]);
    flat.push(dp1.clone());
    flat.push(dp2.clone());

    flat.sort_by(|a, b| compare(a, b));

    let div1 = flat.iter().position(|p| p == &dp1).unwrap();
    let div2 = flat.iter().position(|p| p == &dp2).unwrap();

    println!("part 2: {}", (div1 + 1) * (div2 + 1));

    Ok(())
}
