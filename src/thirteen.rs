use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let (positions, folds) = read_input();
    let answer = get_num_dots_after_first_fold(&positions, &folds);

    println!("{}", answer);
}

pub fn part_two() {
    let (positions, folds) = read_input();
    print_dots_after_folds(&&positions, &folds);
}

struct Paper {
    grid: Vec<Vec<bool>>,
}

impl Paper {
    fn fold(&self, fold: &Fold) -> Paper {
        match fold {
            Fold::X(col) => {
                let mut new_grid = vec![vec![false; *col as usize]; self.grid.len()];

                for (y, row) in self.grid.iter().enumerate() {
                    for (x, v) in row.iter().enumerate() {
                        if x < *col as usize {
                            new_grid[y][x] = *v;
                        }
                    }
                }

                for c in *col as usize + 1..self.grid[0].len() {
                    for y in 0..self.grid.len() {
                        let src_x = c;
                        let dest_x = (col - (c as i32 - col)) as usize;

                        if dest_x > 100000000 {
                            continue;
                        }

                        new_grid[y][dest_x] |= self.grid[y][src_x];
                    }
                }

                Paper { grid: new_grid }
            }
            Fold::Y(row) => {
                let mut new_grid = vec![vec![false; self.grid[0].len()]; *row as usize];

                for (y, row_vec) in self.grid.iter().enumerate() {
                    for (x, v) in row_vec.iter().enumerate() {
                        if y < *row as usize {
                            new_grid[y][x] = *v;
                        }
                    }
                }

                for x in 0..self.grid[0].len() {
                    for y in *row as usize + 1..self.grid.len() {
                        let src_y = y;
                        let dest_y = (row - (y as i32 - row)) as usize;

                        if dest_y > 100000000 {
                            continue;
                        }

                        new_grid[dest_y][x] |= self.grid[src_y][x];
                    }
                }

                Paper { grid: new_grid }
            }
        }
    }

    fn get_num_dots(&self) -> i32 {
        let mut total = 0;
        for row in self.grid.iter() {
            for v in row.iter() {
                if *v {
                    total += 1;
                }
            }
        }

        total
    }

    fn from_dots(dots: &[Position]) -> Paper {
        let max_x = dots.iter().map(|p| p.x).max().unwrap() as usize;
        let max_y = dots.iter().map(|p| p.y).max().unwrap() as usize;

        let mut grid = vec![vec![false; max_x + 1]; max_y + 1];

        for pos in dots.iter() {
            grid[pos.y as usize][pos.x as usize] = true;
        }

        Paper { grid }
    }

    fn print(&self) {
        for row in self.grid.iter() {
            for v in row.iter() {
                match v {
                    true => print!("#"),
                    false => print!("."),
                }
            }
            println!();
        }
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn from_str(position_str: &str) -> Position {
        let parts: Vec<&str> = position_str.split(',').collect();

        let x: i32 = parts[0].parse().unwrap();
        let y: i32 = parts[1].parse().unwrap();

        Position { x, y }
    }
}

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

impl Fold {
    fn from_str(fold_str: &str) -> Fold {
        let parts: Vec<&str> = fold_str.split('=').collect();

        let dir: &str = parts[0].split("g ").collect::<Vec<&str>>()[1];
        let num: i32 = parts[1].parse().unwrap();

        match dir {
            "x" => Fold::X(num),
            "y" => Fold::Y(num),
            _ => panic!(),
        }
    }
}

fn read_input() -> (Vec<Position>, Vec<Fold>) {
    let stdin = io::stdin();

    let mut positions: Vec<Position> = vec![];
    let mut folds: Vec<Fold> = vec![];
    let mut done_with_positions = false;
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            done_with_positions = true;
            continue;
        }

        if !done_with_positions {
            positions.push(Position::from_str(&line));
        } else {
            folds.push(Fold::from_str(&line));
        }
    }

    (positions, folds)
}

fn get_num_dots_after_first_fold(positions: &[Position], folds: &[Fold]) -> i32 {
    let paper = Paper::from_dots(positions);
    let paper = paper.fold(&folds[0]);

    paper.get_num_dots()
}

fn print_dots_after_folds(positions: &[Position], folds: &[Fold]) {
    let mut paper = Paper::from_dots(positions);

    for fold in folds.iter() {
        paper = paper.fold(fold);
    }

    paper.print();
}
