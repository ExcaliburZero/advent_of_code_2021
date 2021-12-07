use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

use num_bigint::BigInt;
use num_traits::sign::Signed;

pub fn part_one() {
    let numbers = read_input();
    let answer = calc_fuel_to_align_to_best_pos(&numbers);

    println!("{}", answer);
}

pub fn part_two() {
    let numbers = read_input();
    let answer = calc_fuel_to_align_to_best_pos_2(&numbers);

    println!("{}", answer);
}

fn read_input() -> Vec<BigInt> {
    let stdin = io::stdin();

    let line: String = stdin.lock().lines().last().unwrap().unwrap();

    return line.split(',').map(|n| n.parse().unwrap()).collect();
}

fn get_total_fuel_cost(p: BigInt, crabs: &[BigInt]) -> BigInt {
    crabs.iter().map(|cp| (cp - p.clone()).abs()).sum()
}

fn fuel_2(n: BigInt, cache: &mut HashMap<BigInt, BigInt>) -> BigInt {
    if n == BigInt::from(0) {
        return BigInt::from(0);
    }

    if cache.contains_key(&n) {
        return cache.get(&n).unwrap().clone();
    }

    let n2 = n.clone();
    let n3 = n.clone();
    let value = n + fuel_2(n2 - 1, cache);

    cache.insert(n3, value.clone());

    value
}

fn get_total_fuel_cost_2(p: BigInt, crabs: &[BigInt]) -> BigInt {
    let mut cache: HashMap<BigInt, BigInt> = HashMap::new();
    crabs
        .iter()
        .map(|cp| fuel_2((cp - p.clone()).abs(), &mut cache))
        .sum()
}

fn calc_fuel_to_align_to_best_pos(crabs: &[BigInt]) -> BigInt {
    let mut best_pos: BigInt =
        crabs.iter().map(|v| v.clone()).sum::<BigInt>() / BigInt::from(crabs.len()) - 1;

    let mut min_fuel = BigInt::from(999999999);
    let mut done = false;
    while !done {
        let new_min_fuel_down = get_total_fuel_cost(best_pos.clone() - 1, crabs);
        let new_min_fuel_up = get_total_fuel_cost(best_pos.clone() + 1, crabs);

        if new_min_fuel_down < min_fuel {
            min_fuel = new_min_fuel_down;
            best_pos -= 1;
        } else if new_min_fuel_up < min_fuel {
            min_fuel = new_min_fuel_up;
            best_pos += 1;
        } else {
            done = true;
        }
    }

    min_fuel
}

fn calc_fuel_to_align_to_best_pos_2(crabs: &[BigInt]) -> BigInt {
    let mut best_pos: BigInt =
        crabs.iter().map(|v| v.clone()).sum::<BigInt>() / BigInt::from(crabs.len()) - 1;

    let mut min_fuel = BigInt::from(2).pow(100000);
    let mut done = false;
    while !done {
        let new_min_fuel_down = get_total_fuel_cost_2(best_pos.clone() - 1, crabs);
        let new_min_fuel_up = get_total_fuel_cost_2(best_pos.clone() + 1, crabs);

        if new_min_fuel_down < min_fuel {
            min_fuel = new_min_fuel_down;
            best_pos -= 1;
        } else if new_min_fuel_up < min_fuel {
            min_fuel = new_min_fuel_up;
            best_pos += 1;
        } else {
            done = true;
        }
    }

    min_fuel
}
