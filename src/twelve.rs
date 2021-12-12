use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let graph = read_input();
    let answer = calc_num_paths(&graph);

    println!("{}", answer);
}

pub fn part_two() {
    let graph = read_input();
    let answer = calc_num_paths_allow_one_small_repeat(&graph);

    println!("{}", answer);
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Node {
    Start,
    End,
    Big(char),
    Small(char),
}

impl Node {
    fn from_str(node_name: &str) -> Node {
        if node_name == "start" {
            Node::Start
        } else if node_name == "end" {
            Node::End
        } else if node_name == node_name.to_uppercase() {
            Node::Big(node_name.chars().last().unwrap())
        } else {
            Node::Small(node_name.chars().last().unwrap())
        }
    }
}

#[derive(Debug)]
struct Graph {
    edges: HashMap<Node, Vec<Node>>,
}

impl Graph {
    fn from_edges(edges_slice: &[(String, String)]) -> Graph {
        let mut edges: HashMap<Node, Vec<Node>> = HashMap::new();

        for (src, dst) in edges_slice.iter() {
            let src = Node::from_str(src);
            let dst = Node::from_str(dst);

            match edges.get_mut(&src) {
                Some(entry) => entry.push(dst),
                None => {
                    edges.insert(src, vec![dst]);
                }
            }

            match edges.get_mut(&dst) {
                Some(entry) => entry.push(src),
                None => {
                    edges.insert(dst, vec![src]);
                }
            }
        }

        Graph { edges }
    }

    fn calc_num_paths(&self, src: Node, dst: Node, visited: &HashSet<Node>) -> i32 {
        if src == dst {
            return 1;
        }

        let neighbors = self.edges.get(&src);
        if neighbors.is_none() {
            return 0;
        }

        let mut total = 0;
        for n in neighbors.unwrap().iter() {
            if *n == Node::Start {
                continue;
            } else if let Node::Small(_) = n {
                if visited.contains(n) {
                    continue;
                }
            }

            let mut visited_2 = visited.clone();
            visited_2.insert(src);

            total += self.calc_num_paths(*n, dst, &visited_2);
        }

        total
    }

    fn calc_num_paths_allow_one_small_repeat(
        &self,
        src: Node,
        dst: Node,
        visited: &HashMap<Node, i32>,
        have_visited_small_twice: bool,
    ) -> i32 {
        if src == dst {
            return 1;
        }

        let neighbors = self.edges.get(&src);
        if neighbors.is_none() {
            return 0;
        }

        let mut total = 0;
        for n in neighbors.unwrap().iter() {
            let mut have_visited_this_twice = have_visited_small_twice;
            if *n == Node::Start {
                continue;
            } else if let Node::Small(_) = n {
                if visited.get(n).is_some() && have_visited_small_twice {
                    continue;
                }

                if visited.get(n).is_some() && *visited.get(n).unwrap() == 1 {
                    have_visited_this_twice = true;
                }
            }

            let mut visited_2 = visited.clone();
            visited_2.insert(src, visited_2.get(&src).or(Some(&0)).unwrap() + 1);

            total += self.calc_num_paths_allow_one_small_repeat(
                *n,
                dst,
                &visited_2,
                have_visited_this_twice,
            );
        }

        total
    }
}

fn read_input() -> Graph {
    let stdin = io::stdin();

    let mut edges: Vec<(String, String)> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('-').collect();

        edges.push((parts[0].to_string(), parts[1].to_string()));
    }

    Graph::from_edges(&edges)
}

fn calc_num_paths(graph: &Graph) -> i32 {
    graph.calc_num_paths(Node::Start, Node::End, &HashSet::new())
}

fn calc_num_paths_allow_one_small_repeat(graph: &Graph) -> i32 {
    graph.calc_num_paths_allow_one_small_repeat(Node::Start, Node::End, &HashMap::new(), false)
}
