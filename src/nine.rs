use std::collections::BTreeSet;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let numbers = read_input();
    let answer = find_sum_risk_low_points(&numbers);

    println!("{}", answer);
}

pub fn part_two() {
    let numbers = read_input();
    let answer = find_basins(&numbers);

    println!("{}", answer);
}

fn read_input() -> Vec<Vec<i64>> {
    let stdin = io::stdin();

    let mut numbers: Vec<Vec<i64>> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts: Vec<char> = line.chars().collect();

        numbers.push(
            parts
                .iter()
                .map(|c| c.to_string().parse().unwrap())
                .collect(),
        );
    }

    numbers
}

fn get_neighbors(map: &[Vec<i64>], i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = vec![];

    if i > 0 {
        neighbors.push((i - 1, j));
    }
    if i < map.len() - 1 {
        neighbors.push((i + 1, j));
    }
    if j > 0 {
        neighbors.push((i, j - 1));
    }
    if j < map[0].len() - 1 {
        neighbors.push((i, j + 1));
    }

    neighbors
}

fn find_sum_risk_low_points(map: &[Vec<i64>]) -> i64 {
    let mut sum = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let mut any_lower = false;
            for (i2, j2) in get_neighbors(map, i, j) {
                if map[i2][j2] <= map[i][j] {
                    any_lower = true;
                }
            }

            if !any_lower {
                sum += map[i][j] + 1;
            }
        }
    }

    sum
}

fn get_basin_size(
    map: &[Vec<i64>],
    visited: &mut HashSet<(usize, usize)>,
    i: usize,
    j: usize,
    max: i64,
) -> i64 {
    let value = map[i][j];
    if value == max || visited.contains(&(i, j)) {
        0
    } else {
        visited.insert((i, j));

        let mut sum = 1;
        for (i2, j2) in get_neighbors(map, i, j) {
            let neighbor = map[i2][j2];
            if neighbor >= value {
                sum += get_basin_size(map, visited, i2, j2, max);
            }
        }

        sum
    }
}

fn find_basins(map: &[Vec<i64>]) -> i64 {
    let mut low_points: Vec<(usize, usize)> = vec![];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let mut any_lower = false;
            for (i2, j2) in get_neighbors(map, i, j) {
                if map[i2][j2] <= map[i][j] {
                    any_lower = true;
                }
            }

            if !any_lower {
                low_points.push((i, j));
            }
        }
    }

    let mut basin_sizes: BTreeSet<i64> = BTreeSet::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for (i, j) in low_points {
        let basin_size = get_basin_size(map, &mut visited, i, j, 9);
        basin_sizes.insert(-basin_size);
    }

    -basin_sizes.iter().take(3).product::<i64>()
}
