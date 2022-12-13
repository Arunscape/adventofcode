#![feature(mixed_integer_ops)]
use ndarray::prelude::*;
use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque, HashSet};

struct Graph {
    graph: Array2<char>,
}

type Coord = (usize, usize);

impl Graph {
    fn possible_next(&self, coord: Coord) -> Vec<(Coord, char)> {
        let up = self.get_point(coord, (-1, 0));
        let down = self.get_point(coord, (1, 0));
        let left = self.get_point(coord, (0, -1));
        let right = self.get_point(coord, (0, 1));

        let ch: char = *self.graph.get(coord).unwrap();

        [up, down, left, right]
            .iter()
            .flatten()
            .filter_map(|&c| {
                if c.1 == 'E' {
                    Some(c)
                } else if c.1 == 'S' {
                    None
                } else if c.1 as u32 - 1 <= ch as u32 {
                    Some(c)
                } else {
                    None
                }
            })
            .collect()
    }
    fn get_point(&self, (i, j): Coord, offset: (isize, isize)) -> Option<(Coord, char)> {
        let m = i.checked_add_signed(offset.0)?;
        let n = j.checked_add_signed(offset.1)?;
        let c = self.graph.get((m, n))?;

        Some(((m, n), *c))
    }

    fn shortest_path_bfs(&self, start: Coord, target: Coord) -> Vec<Coord> {
        
        let mut visited: HashSet<Coord> = HashSet::new();
        let mut q = VecDeque::new();

        q.push_back(start);
        visited.insert(start);

        let mut parent: HashMap<Coord, Coord> = HashMap::new();

        while let Some(v) = q.pop_front() {
            if v == target {
                break;
            }

            for &(w, _) in self.possible_next(v).iter(){
                if visited.contains(&w) {
                    continue;
                }

                visited.insert(w);
                parent.insert(w, v);
            }
        }


        let mut ret = visited.iter().cloned().collect();

        // let mut curr = target;

        // return Vec::new();
        // while curr != start {
        //     let p = *parent.get(&curr).unwrap_or(&(69420, 69420));
        //     ret.push(p);
        //     curr = p;
        // }


        ret


    }
}

fn main() {
    let input: Vec<_> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| {
            line.chars().collect::<Vec<_>>()
        })
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

    let g = Graph {graph: input};


    let path = g.shortest_path_bfs(start, end);

    dbg!(&path);


}
