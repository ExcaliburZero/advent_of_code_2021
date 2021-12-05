use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let lines = read_input();
    let answer = get_hori_verti_num_at_least_two_overlaps(&lines);

    println!("{}", answer);
}

pub fn part_two() {
    let lines = read_input();
    let answer = get_hori_verti_diag_num_at_least_two_overlaps(&lines);

    println!("{}", answer);
}

fn read_input() -> Vec<Line> {
    let stdin = io::stdin();

    let mut lines: Vec<Line> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        lines.push(Line::from_str(&line).unwrap());
    }

    lines
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_str(point_str: &str) -> Option<Point> {
        let parts: Vec<&str> = point_str.split(',').collect();

        let x: i32 = parts[0].parse().unwrap();
        let y: i32 = parts[1].parse().unwrap();

        Some(Point { x, y })
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn from_str(point_str: &str) -> Option<Line> {
        let parts: Vec<&str> = point_str.split(" -> ").collect();

        let start = Point::from_str(parts[0]).unwrap();
        let end = Point::from_str(parts[1]).unwrap();

        Some(Line { start, end })
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn mark_points(&self, grid: &mut HashMap<Point, i32>) {
        if self.is_horizontal() {
            let dir = sign(self.end.x - self.start.x);
            let y = self.start.y;
            let mut x = self.start.x;
            while true {
                let point = Point { x, y };
                if grid.contains_key(&point) {
                    *grid.get_mut(&point).unwrap() += 1;
                } else {
                    grid.insert(point, 1);
                }

                if x == self.end.x {
                    break;
                }
                x += dir;
            }
        } else if self.is_vertical() {
            let dir = sign(self.end.y - self.start.y);
            let x = self.start.x;
            let mut y = self.start.y;
            while true {
                let point = Point { x, y };
                if grid.contains_key(&point) {
                    *grid.get_mut(&point).unwrap() += 1;
                } else {
                    grid.insert(point, 1);
                }

                if y == self.end.y {
                    break;
                }
                y += dir;
            }
        } else {
            let delta_x = sign(self.end.x - self.start.x);
            let delta_y = sign(self.end.y - self.start.y);

            let mut x = self.start.x;
            let mut y = self.start.y;

            while true {
                let point = Point { x, y };
                if grid.contains_key(&point) {
                    *grid.get_mut(&point).unwrap() += 1;
                } else {
                    grid.insert(point, 1);
                }

                if y == self.end.y {
                    break;
                }
                y += delta_y;
                x += delta_x;
            }
        }
    }
}

fn sign(value: i32) -> i32 {
    if value > 0 {
        1
    } else if value < 0 {
        -1
    } else {
        panic!()
    }
}

fn get_hori_verti_num_at_least_two_overlaps(lines: &[Line]) -> i32 {
    let mut grid: HashMap<Point, i32> = HashMap::new();

    for line in lines.iter() {
        if line.is_horizontal() || line.is_vertical() {
            line.mark_points(&mut grid);
        }
    }

    grid.iter().filter(|(_k, v)| **v >= 2).count() as i32
}

fn get_hori_verti_diag_num_at_least_two_overlaps(lines: &[Line]) -> i32 {
    let mut grid: HashMap<Point, i32> = HashMap::new();

    for line in lines.iter() {
        line.mark_points(&mut grid);
    }

    grid.iter().filter(|(_k, v)| **v >= 2).count() as i32
}
