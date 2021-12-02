use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let numbers = read_input();
    let answer = get_final_position(&numbers);

    println!("{}", answer.depth * answer.horizontal);
}

pub fn part_two() {
    let numbers = read_input();
    let answer = get_final_position_2(&numbers);

    println!("{}", answer.depth * answer.horizontal);
}

fn read_input() -> Vec<(String, i32)> {
    let stdin = io::stdin();

    let mut commands: Vec<(String, i32)> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        let direction = parts[0].to_string();
        let number: i32 = parts[1].parse().unwrap();

        commands.push((direction, number));
    }

    commands
}

enum Direction {
    Up,
    Down,
    Forward,
}

impl Direction {
    fn from_str(direction_str: &str) -> Option<Direction> {
        match direction_str {
            "up" => Some(Direction::Up),
            "down" => Some(Direction::Down),
            "forward" => Some(Direction::Forward),
            _ => None,
        }
    }
}

struct Position {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

impl Position {
    fn move_direction(&mut self, direction: Direction, distance: i32) {
        match direction {
            Direction::Up => self.depth -= distance,
            Direction::Down => self.depth += distance,
            Direction::Forward => self.horizontal += distance,
        }
    }

    fn move_direction_2(&mut self, direction: Direction, distance: i32) {
        match direction {
            Direction::Up => self.aim -= distance,
            Direction::Down => self.aim += distance,
            Direction::Forward => {
                self.horizontal += distance;
                self.depth += self.aim * distance;
            }
        }
    }
}

fn get_final_position(moves: &[(String, i32)]) -> Position {
    let mut position = Position {
        depth: 0,
        horizontal: 0,
        aim: 0,
    };

    for m in moves.iter() {
        let direction = Direction::from_str(&m.0).unwrap();
        let distance = m.1;

        position.move_direction(direction, distance);
    }

    position
}

fn get_final_position_2(moves: &[(String, i32)]) -> Position {
    let mut position = Position {
        depth: 0,
        horizontal: 0,
        aim: 0,
    };

    for m in moves.iter() {
        let direction = Direction::from_str(&m.0).unwrap();
        let distance = m.1;

        position.move_direction_2(direction, distance);
    }

    position
}
