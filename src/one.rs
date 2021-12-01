use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let mut numbers = read_input();
    let answer = count_num_increases(&numbers);

    println!("{}", answer);
}

pub fn part_two() {
    let mut numbers = read_input();
    let answer = count_num_3_sum_increases(&numbers);

    println!("{}", answer);
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

fn count_num_increases(numbers: &[i32]) -> i32 {
    let mut prev = numbers.first().unwrap();
    let mut num_increases = 0;
    for num in numbers.iter() {
        if num > prev {
            num_increases += 1;
        }
        prev = num;
    }

    num_increases
}

fn count_num_3_sum_increases(numbers: &[i32]) -> i32 {
    let mut prev_sum: Option<i32> = None;
    let mut num_increases = 0;

    for i in 0..numbers.len() - 2 {
        let a = numbers[i];
        let b = numbers[i + 1];
        let c = numbers[i + 2];

        let sum = a + b + c;

        if let Some(prev) = prev_sum {
            if sum > prev {
                num_increases += 1;
            }
        }

        prev_sum = Some(sum)
    }

    num_increases
}
