use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let mut field = read_input();
    let answer = calc_num_flashes(&mut field, 100);

    println!("{}", answer);
}

pub fn part_two() {
    let mut field = read_input();
    let answer = calc_first_all_flash(&mut field);

    println!("{}", answer);
}

struct Field {
    octopi: [[i32; 10]; 10],
}

impl Field {
    fn from_lines(lines: &[String]) -> Field {
        let mut octopi = [[0; 10]; 10];

        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                octopi[i][j] = c.to_string().parse().unwrap();
            }
        }

        Field { octopi }
    }

    fn flash(&mut self, flashed: &mut HashSet<(usize, usize)>, i: usize, j: usize) -> i32 {
        let mut num_flashes = 1;

        flashed.insert((i, j));

        for (i2, j2) in self.get_neighbors(i, j) {
            let value = self.octopi[i2][j2];

            if !flashed.contains(&(i2, j2)) {
                if value >= 9 {
                    self.octopi[i2][j2] = 0;
                    num_flashes += self.flash(flashed, i2, j2);
                } else {
                    self.octopi[i2][j2] += 1;
                }
            }
        }

        num_flashes
    }

    fn step(&mut self) -> i32 {
        let mut num_flashes = 0;

        let mut flashed: HashSet<(usize, usize)> = HashSet::new();
        for i in 0..self.octopi.len() {
            for j in 0..self.octopi[0].len() {
                let value = self.octopi[i][j];

                if !flashed.contains(&(i, j)) {
                    if value >= 9 {
                        self.octopi[i][j] = 0;
                        num_flashes += self.flash(&mut flashed, i, j);
                    } else {
                        self.octopi[i][j] += 1;
                    }
                }
            }
        }

        num_flashes
    }

    fn get_neighbors(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let i = i as i32;
        let j = j as i32;

        let mut neighbors: Vec<(usize, usize)> = vec![];
        for i_delta in -1..2 {
            for j_delta in -1..2 {
                let n_i = i + i_delta;
                let n_j = j + j_delta;

                if i_delta == 0 && j_delta == 0 {
                    continue;
                }

                if n_i >= 0
                    && n_i < self.octopi.len() as i32
                    && n_j >= 0
                    && n_j < self.octopi[0].len() as i32
                {
                    neighbors.push((n_i as usize, n_j as usize));
                }
            }
        }

        neighbors
    }

    /*fn print(&self) {
        for i in 0..self.octopi.len() {
            for j in 0..self.octopi[0].len() {
                print!("{}", self.octopi[i][j]);
            }
            println!();
        }
        println!("=================================");
    }*/
}

fn calc_num_flashes(field: &mut Field, steps: u32) -> i32 {
    (0..steps).map(|_| field.step()).sum()
}

fn calc_first_all_flash(field: &mut Field) -> i32 {
    let mut step = 0;
    loop {
        let num_flashes = field.step();
        step += 1;

        if num_flashes == 100 {
            return step;
        }
    }
}

fn read_input() -> Field {
    let stdin = io::stdin();

    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().to_string())
        .collect();

    Field::from_lines(&lines)
}
