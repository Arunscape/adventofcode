use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in io::stdin().lock().lines().flatten() {
        let mut line = line.split(" contain ");

        let parent_bag: String = line.next().unwrap().split(" bag").take(1).collect();

        let children_bags = line.next().unwrap();
        if children_bags.contains("no other bags") {
            continue;
        }

        let children_bags: Vec<(String, u32)> = children_bags
            .split(", ")
            .map(|bag| bag.split(" bag").take(1).collect::<String>())
            .map(|bag| {
                (
                    bag.chars().skip(2).collect(),
                    bag.chars().next().unwrap().to_digit(10).unwrap(),
                )
            })
            .collect();

        for (bag, _) in children_bags {
            map.entry(bag)
                .and_modify(|e| e.push(parent_bag.clone()))
                .or_insert_with(|| vec![parent_bag.clone()]);
        }
    }

    fn recurse_count(map: &HashMap<String, Vec<String>>, set: &mut HashSet<String>, key: &str) {
        set.insert(key.into());
        if let Some(vec) = map.get(key) {
            for bag in vec {
                recurse_count(map, set, bag);
            }
        }
    }
    let mut set = HashSet::new();
    recurse_count(&map, &mut set, "shiny gold");
    println!("{}", set.len() - 1);
}
