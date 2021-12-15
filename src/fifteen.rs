extern crate priority_queue;

use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;

use priority_queue::PriorityQueue;

pub fn part_one() {
    let graph = read_input(1);
    let answer = get_lowest_risk_path(&graph);

    println!("{}", answer);
}

pub fn part_two() {
    let graph = read_input(5);
    let answer = get_lowest_risk_path(&graph);

    println!("{}", answer);
}

type Node = (i32, i32);

#[derive(Debug)]
struct Graph {
    grid: Vec<Vec<i32>>,
    multiplier: i32,
}

impl Graph {
    fn from_lines(lines: &[String], multiplier: i32) -> Graph {
        let mut rows = vec![];

        for line in lines.iter() {
            let row: Vec<i32> = line
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect();

            rows.push(row);
        }

        Graph {
            grid: rows,
            multiplier,
        }
    }

    fn print(&self) {
        for y in 0..self.grid.len() * self.multiplier as usize {
            if y % self.grid.len() == 0 {
                println!("=======================================");
            }
            for x in 0..self.grid[0].len() * self.multiplier as usize {
                if x % self.grid[0].len() == 0 {
                    print!("|");
                }
                print!("{}", self.get_risk((y as i32, x as i32)));
            }
            println!();
        }
    }

    fn get_risk(&self, node: Node) -> i32 {
        let (y, x) = node;

        let dist = (y / self.grid.len() as i32) + (x / self.grid.len() as i32);

        (((self.grid[y as usize % self.grid.len()][x as usize % self.grid[0].len()] + dist) - 1)
            % 9)
            + 1
    }

    fn get_neighbors(&self, node: Node) -> Vec<Node> {
        let mut neighbors = vec![];
        for y_delta in -1..2 {
            for x_delta in -1..2 {
                if y_delta != 0 && x_delta != 0 {
                    continue;
                }
                if y_delta == 0 && x_delta == 0 {
                    continue;
                }

                let (y, x) = node;
                let new_pos = (y + y_delta, x + x_delta);

                if new_pos.0 < 0 || new_pos.0 >= self.grid.len() as i32 * self.multiplier {
                    continue;
                }
                if new_pos.1 < 0 || new_pos.1 >= self.grid[0].len() as i32 * self.multiplier {
                    continue;
                }

                neighbors.push(new_pos);
            }
        }

        neighbors
    }

    fn shortest_path(&self, src: Node, dest: Node) -> Option<Vec<Node>> {
        let mut distances: PriorityQueue<Node, i32> = PriorityQueue::new();
        let mut paths: HashMap<Node, Vec<Node>> = HashMap::new();
        let mut visited: HashSet<Node> = HashSet::new();

        if src == dest {
            return Some(vec![dest]);
        }

        paths.insert(src, vec![]);
        distances.push(src, 0);

        loop {
            let (cur, mut weight) = distances.pop().unwrap();
            weight *= -1;

            let path = paths.get(&cur).unwrap().clone();

            visited.insert(cur);

            for n in self.get_neighbors(cur) {
                if visited.contains(&n) {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push(n);
                let new_weight: i32 = new_path.iter().map(|n| self.get_risk(*n)).sum();

                if n == dest {
                    return Some(new_path);
                }

                if paths.contains_key(&n) {
                    let (_, &(mut prev_weight)) = distances.get(&n).unwrap();
                    prev_weight *= -1;

                    if new_weight < prev_weight {
                        paths.insert(n, new_path);
                        distances.push(n, -new_weight);
                    }
                } else {
                    paths.insert(n, new_path);
                    distances.push(n, -new_weight);
                }
            }
        }

        panic!()
    }
}

fn read_input(multiplier: i32) -> Graph {
    let stdin = io::stdin();

    let mut lines: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap().to_string();

        lines.push(line);
    }

    Graph::from_lines(&lines, multiplier)
}

fn get_lowest_risk_path(graph: &Graph) -> i32 {
    let path = graph
        .shortest_path(
            (0, 0),
            (
                graph.grid.len() as i32 * graph.multiplier - 1,
                graph.grid[0].len() as i32 * graph.multiplier - 1,
            ),
        )
        .unwrap();

    path.iter().map(|n| graph.get_risk(*n)).sum()
}
