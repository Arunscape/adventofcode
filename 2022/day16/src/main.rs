use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

struct Graph {
    flow_rate: HashMap<String, usize>,
    neighbors: HashMap<String, HashSet<String>>,
}

impl Graph {
    pub fn new() -> Self {
        let flow_rate = HashMap::new();
        let neighbors = HashMap::new();

        Self { flow_rate, neighbors }
    }


    pub fn insert(&mut self, valve: String, flow_rate: usize, neighbors: HashSet<String>) {
        self.flow_rate.insert(valve.clone(), flow_rate);
        self.neighbors.insert(valve.clone(), neighbors);
    }

    pub fn neighbors(&self, valve: &str) -> &HashSet<String> {
        self.neighbors.get(valve).unwrap()
    }

    pub fn flow_rate(&self, valve: &str) -> usize {
        *self.flow_rate.get(valve).unwrap()
    }

    pub fn maximize_flow(&self) -> usize {

        let opened = self.flow_rate.keys().cloned().map(|v| (v, false)).collect();
        self.maximize_flow_helper(0, 30, opened, "AA")
    }

    pub fn maximize_flow_helper(&self, flow: usize, minutes_left: usize, opened: HashMap<String, bool>, current_node: &str) -> usize {

        let mut flow = flow + opened.iter().filter_map(|(v, o)| match o { true => Some(self.flow_rate(v)), false => None }).sum::<usize>();

        for unopened_valve in opened.iter_mut() {

            flow = flow + self.maximize_flow_helper(minutes_left - 2, 
            
        }



        flow
    }
}

fn main() {

    let input: Vec<(String, usize, HashSet<String>)> = io::stdin().lock().lines().flatten().filter_map(|line|{
        let line = line.strip_prefix("Valve ")?;
        let (valve, line) = line.split_once(" has flow rate=")?;
        let valve = valve.to_owned();
        let (flow_rate, line) = line.split_once("; tunnels lead to valves ")?;
        let flow_rate = flow_rate.parse().ok()?;
        let leads_to = line.split(", ").map(String::from).collect();

        Some((valve, flow_rate, leads_to))
    }).collect();


    let mut g = Graph::new();
    input.into_iter().for_each(|(v, f, n)| g.insert(v, f, n));

    let p1 = g.maximize_flow(0, 30, HashSet::new(), "AA");
    println!("part 1: {p1}");
}
