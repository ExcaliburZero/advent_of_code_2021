use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let range = read_input();
    let answer = solve_1(&range);

    println!("{}", answer);
}

pub fn part_two() {
    let range = read_input();
    let answer = solve_2(&range);

    println!("{}", answer);
}

#[derive(Debug)]
struct Range2D {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Range2D {
    fn contains(&self, position: &(i32, i32)) -> bool {
        position.0 >= self.y_min
            && position.0 <= self.y_max
            && position.1 >= self.x_min
            && position.1 <= self.x_max
    }

    fn is_beyond(&self, position: &(i32, i32)) -> bool {
        position.1 >= self.x_max || position.0 < self.y_min
    }

    fn from_str(range_str: &str) -> Range2D {
        let parts: Vec<&str> = range_str.split(", ").collect();

        let parts_x = parts[0].split(' ').collect::<Vec<&str>>()[2];
        let parts_y = parts[1];

        let x_min: i32 = parts_x.split("..").collect::<Vec<&str>>()[0]
            .split('=')
            .collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        let x_max: i32 = parts_x.split("..").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        let y_min: i32 = parts_y.split("..").collect::<Vec<&str>>()[0]
            .split('=')
            .collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        let y_max: i32 = parts_y.split("..").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        Range2D {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}

fn read_input() -> Range2D {
    let stdin = io::stdin();

    let line = stdin.lock().lines().last().unwrap().unwrap();

    Range2D::from_str(&line)
}

fn lands_in_range(mut velocity_x: i32, mut velocity_y: i32, range: &Range2D) -> Option<(u32, i32)> {
    let mut step = 0;

    let mut position = (0, 0);
    let mut highest_y = 0;
    loop {
        step += 1;

        position = (position.0 + velocity_y, position.1 + velocity_x);

        velocity_x = if velocity_x > 0 {
            velocity_x - 1
        } else if velocity_x < 0 {
            velocity_x + 1
        } else {
            0
        };
        velocity_y -= 1;

        if position.0 > highest_y {
            highest_y = position.0;
        }

        if range.contains(&position) {
            return Some((step, highest_y));
        }

        if range.is_beyond(&position) {
            break;
        }
    }

    None
}

fn solve_1(range: &Range2D) -> i32 {
    let mut highest_highest_y = 0;
    for vel_x in 0..400 {
        for vel_y in (0..400).rev() {
            match lands_in_range(vel_x, vel_y, range) {
                Some((_, highest_y)) => {
                    if highest_y > highest_highest_y {
                        highest_highest_y = highest_y;
                    }
                    return highest_y;
                }
                None => (),
            }
        }
    }

    highest_highest_y
}

fn solve_2(range: &Range2D) -> i32 {
    let mut count = 0;
    for vel_x in 0..400 {
        for vel_y in (-400..400).rev() {
            match lands_in_range(vel_x, vel_y, range) {
                Some((_, _)) => {
                    count += 1;
                }
                None => (),
            }
        }
    }

    count
}
