use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let mut numbers = read_input();
    //let answer = multiply_2020_addends(&mut numbers);

    //println!("{}", answer);
    println!("{:?}", numbers);
}

pub fn part_two() {
    let mut numbers = read_input();
    //let answer = multiply_2020_addends(&mut numbers);

    //println!("{}", answer);
    println!("{:?}", numbers);
}

fn read_input() -> Vec<i32> {
    let stdin = io::stdin();

    let mut numbers: Vec<i32> = Vec::new();
    for line in stdin.lock().lines() {
        let number: i32 = line.unwrap().parse().unwrap();

        numbers.push(number);
    }

    numbers
}
