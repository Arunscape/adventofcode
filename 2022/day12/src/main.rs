#![feature(mixed_integer_ops)]
use ndarray::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};

struct Graph {
    graph: Array2<char>,
}

type Coord = (usize, usize);

impl Graph {
    fn possible_next(&self, coord: Coord) -> Vec<Coord> {
        let up = self.get_point(coord, (-1, 0));
        let down = self.get_point(coord, (1, 0));
        let left = self.get_point(coord, (0, -1));
        let right = self.get_point(coord, (0, 1));

        let curr_ch: char = *self.graph.get(coord).unwrap();

        [up, down, left, right]
            .iter()
            .flatten()
            .filter_map(|&c| {
                let next_ch = self.graph.get(c)?;
                if *next_ch as u32 - 1 <= curr_ch as u32 {
                    Some(c)
                } else {
                    None
                }
            })
            .collect()
    }
    fn get_point(&self, (i, j): Coord, offset: (isize, isize)) -> Option<Coord> {
        let m = i.checked_add_signed(offset.0)?;
        let n = j.checked_add_signed(offset.1)?;
        Some((m, n))
    }

    fn shortest_path_bfs(&self, start: Coord, target: Coord) -> Option<Vec<Coord>> {
        let mut visited: HashSet<Coord> = HashSet::new();
        let mut q = VecDeque::new();

        q.push_back(start);
        visited.insert(start);

        let mut parent: HashMap<Coord, Coord> = HashMap::new();

        let mut found: Option<Coord> = None;
        while let Some(v) = q.pop_front() {
            if v == target {
                println!("found it for starting point {start:?}");
                found = Some(v);
                break;
            }

            //println!("at {v:?}");
            for &w in self.possible_next(v).iter() {
                // println!(" considering {w:?}");
                if visited.contains(&w) {
                    continue;
                }

                visited.insert(w);
                parent.insert(w, v);
                q.push_back(w);
            }
        };


        let found = found?;
        //let mut ret = visited.iter().cloned().collect();
        let mut ret = Vec::new();

        let mut curr = target;

        while curr != start {
            let p = *parent.get(&curr).unwrap();
            ret.push(p);
            curr = p;
        }
        
        Some(ret)
    }
}

fn main() {
    let input: Vec<_> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let (rowlen, nrows) = (input[0].len(), input.len());
    let input = input.iter().cloned().flatten().collect();

    let mut input: Array2<char> = Array2::from_shape_vec((nrows, rowlen), input).unwrap();

    let (start, _) = input.indexed_iter().find(|(_, &c)| c == 'S').unwrap();
    let (end, _) = input.indexed_iter().find(|(_, &c)| c == 'E').unwrap();

    let s = input.get_mut(start).unwrap();
    *s = 'a';
    let e = input.get_mut(end).unwrap();
    *e = 'z';

    let g = Graph { graph: input.clone() };

    let path = g.shortest_path_bfs(start, end).unwrap();
    let pathlen = path.len();

    dbg!(&path);
    println!("part 1: {pathlen}");

    let p2 = input.indexed_iter().filter(|(_, &c)| c == 'a').filter_map(|(coord, c)| {
        //println!("start point: {coord:?}, {c}");
        let path = g.shortest_path_bfs(coord, end)?;
        Some(path.len())
    }).min().unwrap();
    println!("part 2: {p2}");


}
