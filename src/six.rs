use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let mut numbers = read_input();
    let answer = get_num_fish_after_n_days(&mut numbers, 80);

    println!("{}", answer);
}

pub fn part_two() {
    let mut numbers = read_input();
    let answer = get_num_fish_after_n_days_calc(&mut numbers, 256);

    println!("{}", answer);
}

fn read_input() -> Vec<i64> {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let numbers: Vec<i64> = line.split(',').map(|n| n.parse().unwrap()).collect();

        return numbers;
    }

    panic!()
}

fn simulate_day(fish: &mut Vec<i64>) {
    let mut new_fish: Vec<i64> = vec![];

    for f in fish.iter_mut() {
        if *f == 0 {
            *f = 6;
            new_fish.push(8);
        } else {
            *f -= 1;
        }
    }

    fish.append(&mut new_fish);
}

fn get_num_fish_after_n_days(fish: &mut Vec<i64>, num_days: i64) -> i64 {
    for _ in 0..num_days {
        simulate_day(fish);
    }

    fish.len() as i64
}

fn get_num_fish_after_n_days_calc(fish: &mut Vec<i64>, num_days: i64) -> i64 {
    let mut cache: HashMap<(i64, i64), i64> = HashMap::new();

    let mut total = 0;
    for f in fish.iter() {
        total += calc(&mut cache, num_days, *f);
    }

    total
}

fn calc(cache: &mut HashMap<(i64, i64), i64>, mut num_days: i64, mut f: i64) -> i64 {
    if num_days == 0 {
        return 1;
    }

    if f > 0 && num_days >= f {
        num_days -= f;
        f = 0;
    } else {
        f -= num_days;
        num_days = 0;
    }

    if cache.contains_key(&(num_days, f)) {
        return cache[&(num_days, f)];
    }

    if num_days > 0 {
        num_days -= 1;
        let value = calc(cache, num_days, 6) + calc(cache, num_days, 8);

        cache.insert((num_days + 1, f), value);

        value
    } else {
        1
    }
}
