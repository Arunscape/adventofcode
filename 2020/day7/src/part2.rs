use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let mut map: HashMap<String, Vec<(String, u32)>> = HashMap::new();

    for line in io::stdin().lock().lines().flatten() {
        let mut line = line.split(" contain ");

        let parent_bag: String = line.next().unwrap().split(" bag").take(1).collect();

        let children_bags = line.next().unwrap();
        if children_bags.contains("no other bags") {
            map.insert(parent_bag, Vec::new());
            continue;
        }

        let mut children_bags: Vec<(String, u32)> = children_bags
            .split(", ")
            .map(|bag| bag.split(" bag").take(1).collect::<String>())
            .map(|bag| {
                (
                    bag.chars().skip(2).collect(),
                    bag.chars().next().unwrap().to_digit(10).unwrap(),
                )
            })
            .collect();

        map.entry(parent_bag)
            .and_modify(|e| e.append(&mut children_bags))
            .or_insert(children_bags);
    }

    fn recurse_count(map: &HashMap<String, Vec<(String, u32)>>, key: &str) -> u32 {
        let mut count = 1;
        if let Some(vec) = map.get(key) {
            for (bag, num) in vec {
                count += num * recurse_count(map, bag);
            }
        }
        count
    }

    let ans = recurse_count(&map, "shiny gold") - 1;
    println!("{}", ans);
}
