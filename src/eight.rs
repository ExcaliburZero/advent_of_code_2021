use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let entries = read_input();
    let answer = get_num_1_4_7_8(&entries);

    println!("{}", answer);
}

pub fn part_two() {
    let entries = read_input();
    //let answer = get_life_support_rating(&numbers);

    //println!("{}", answer);
}

fn read_input() -> Vec<(Vec<String>, Vec<String>)> {
    let stdin = io::stdin();

    let mut entries: Vec<(Vec<String>, Vec<String>)> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" | ").collect();

        let signal_patterns: Vec<String> = parts[0].split(' ').map(|s| s.to_string()).collect();
        let output_value: Vec<String> = parts[1].split(' ').map(|s| s.to_string()).collect();

        entries.push((signal_patterns, output_value));
    }

    entries
}

fn get_num_1_4_7_8(entries: &[(Vec<String>, Vec<String>)]) -> i32 {
    let interesting_digit_lengths: Vec<i32> = vec![2, 3, 4, 7];

    let mut count = 0;
    for (_, output_value) in entries.iter() {
        for pattern in output_value {
            if interesting_digit_lengths.contains(&(pattern.len() as i32)) {
                count += 1;
            }
        }
    }

    count
}
