use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let numbers = read_input();
    let answer = get_power_consumption(&numbers);

    println!("{}", answer);
}

pub fn part_two() {
    let numbers = read_input();
    let answer = get_life_support_rating(&numbers);

    println!("{}", answer);
}

fn read_input() -> Vec<Vec<i32>> {
    let stdin = io::stdin();

    let mut numbers: Vec<Vec<i32>> = Vec::new();
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

fn get_power_consumption(numbers: &[Vec<i32>]) -> i32 {
    let mut digits: Vec<i32> = vec![];
    for i in 0..numbers[0].len() {
        let mut num_zeros = 0;
        let mut num_ones = 0;
        for number in numbers.iter() {
            match number[i] {
                0 => num_zeros += 1,
                1 => num_ones += 1,
                _ => panic!(),
            }
        }

        if num_zeros > num_ones {
            digits.push(0);
        } else {
            digits.push(1);
        }
    }

    let inverted = invert_bits(&digits);

    let gamma_rate = bits_to_i32(&digits);
    let epsilon_rate = bits_to_i32(&inverted);

    gamma_rate * epsilon_rate
}

fn invert_bits(bits: &[i32]) -> Vec<i32> {
    bits.iter()
        .map(|b| match b {
            0 => 1,
            1 => 0,
            _ => panic!(),
        })
        .collect()
}

fn bits_to_i32(bits: &[i32]) -> i32 {
    let mut num = 0;
    for bit in bits.iter() {
        num <<= 1;
        num += bit;
    }

    num
}

fn get_most_common_digit(numbers: &[Vec<i32>], position: usize) -> i32 {
    let mut num_zeros = 0;
    let mut num_ones = 0;
    for number in numbers.iter() {
        match number[position] {
            0 => num_zeros += 1,
            1 => num_ones += 1,
            _ => panic!(),
        }
    }

    if num_zeros > num_ones {
        0
    } else {
        1
    }
}

fn get_least_common_digit(numbers: &[Vec<i32>], position: usize) -> i32 {
    let mut num_zeros = 0;
    let mut num_ones = 0;
    for number in numbers.iter() {
        match number[position] {
            0 => num_zeros += 1,
            1 => num_ones += 1,
            _ => panic!(),
        }
    }

    if num_zeros > num_ones {
        1
    } else {
        0
    }
}

fn get_life_support_rating(numbers: &[Vec<i32>]) -> i32 {
    let mut oxygen_rating_candidates: Vec<Vec<i32>> = numbers.to_vec();
    for i in 0..numbers[0].len() {
        if oxygen_rating_candidates.len() == 1 {
            break;
        }

        let most_common_digit = get_most_common_digit(&oxygen_rating_candidates, i);

        oxygen_rating_candidates = oxygen_rating_candidates
            .iter()
            .filter(|n| n[i] == most_common_digit)
            .cloned()
            .collect();
    }

    let oxygen_generator_rating = bits_to_i32(oxygen_rating_candidates.first().unwrap());

    let mut co2_rating_candidates: Vec<Vec<i32>> = numbers.to_vec();
    for i in 0..numbers[0].len() {
        if co2_rating_candidates.len() == 1 {
            break;
        }

        let least_common_digit = get_least_common_digit(&co2_rating_candidates, i);

        co2_rating_candidates = co2_rating_candidates
            .iter()
            .filter(|n| n[i] == least_common_digit)
            .cloned()
            .collect();
    }

    let co2_scrubber_rating = bits_to_i32(co2_rating_candidates.first().unwrap());

    oxygen_generator_rating * co2_scrubber_rating
}
