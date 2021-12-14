use std::collections::BTreeMap;
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let (template, rules) = read_input();
    let answer = solve_1(&template, &rules);

    println!("{}", answer);
}

pub fn part_two() {
    let (template, rules) = read_input();
    let answer = solve_2(&template, &rules);

    println!("{}", answer);
}

struct Rule {
    start: char,
    end: char,
    fill: char,
}

impl Rule {
    fn from_tuple(abc: &(char, char, char)) -> Rule {
        let (a, b, c) = abc;

        Rule {
            start: *a,
            end: *b,
            fill: *c,
        }
    }

    fn applies(&self, a: char, b: char) -> bool {
        a == self.start && b == self.end
    }
}

fn read_input() -> (String, Vec<Rule>) {
    let stdin = io::stdin();

    let mut template: Option<String> = None;
    let mut rules: Vec<Rule> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if template.is_none() {
            template = Some(line.to_string());
            continue;
        } else if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(" -> ").collect();
        let parts_ab: Vec<char> = parts[0].chars().collect();

        let a = parts_ab[0];
        let b = parts_ab[1];
        let c = parts[1].chars().last().unwrap();

        rules.push(Rule::from_tuple(&(a, b, c)));
    }

    (template.unwrap(), rules)
}

fn solve_1(template: &str, rules: &[Rule]) -> i32 {
    let mut current_string = template.to_string();

    for _ in 0..10 {
        let mut new_chars: Vec<char> = vec![current_string.chars().collect::<Vec<char>>()[0]];

        for (cur, next) in current_string.chars().zip(current_string.chars().skip(1)) {
            for rule in rules.iter() {
                if rule.applies(cur, next) {
                    new_chars.push(rule.fill);
                }
            }

            new_chars.push(next);
        }

        current_string = new_chars.iter().collect::<String>();
    }

    let mut frequency_table: BTreeMap<char, i32> = BTreeMap::new();
    for c in current_string.chars() {
        if frequency_table.contains_key(&c) {
            frequency_table.insert(c, frequency_table[&c] + 1);
        } else {
            frequency_table.insert(c, 1);
        }
    }

    let most_common_count = frequency_table.values().max().unwrap();
    let least_common_count = frequency_table.values().min().unwrap();

    most_common_count - least_common_count
}

fn to_pairs(string: &str) -> BTreeMap<(char, char), i64> {
    let mut counts: BTreeMap<(char, char), i64> = BTreeMap::new();
    for ab in string.chars().zip(string.chars().skip(1)) {
        if counts.contains_key(&ab) {
            counts.insert(ab, counts[&ab] + 1);
        } else {
            counts.insert(ab, 1);
        }
    }

    counts
}

fn solve_2(template: &str, rules: &[Rule]) -> i64 {
    let mut current_pairs = to_pairs(template);

    for _ in 0..40 {
        let mut new_pairs = BTreeMap::new();

        for ((cur, next), count) in current_pairs.iter() {
            let mut any_applied = false;
            for rule in rules.iter() {
                if rule.applies(*cur, *next) {
                    any_applied = true;

                    let np = (*cur, rule.fill);
                    if new_pairs.contains_key(&np) {
                        new_pairs.insert(np, new_pairs[&np] + 1 * count);
                    } else {
                        new_pairs.insert(np, 1 * count);
                    }

                    let np = (rule.fill, *next);
                    if new_pairs.contains_key(&np) {
                        new_pairs.insert(np, new_pairs[&np] + 1 * count);
                    } else {
                        new_pairs.insert(np, 1 * count);
                    }
                }
            }

            if !any_applied {
                let ab = (*cur, *next);
                if new_pairs.contains_key(&ab) {
                    new_pairs.insert(ab, new_pairs[&ab] + 1 * count);
                } else {
                    new_pairs.insert(ab, 1 * count);
                }
            }
        }

        current_pairs = new_pairs;
    }

    let mut frequency_table: BTreeMap<char, i64> = BTreeMap::new();
    for ((c, _), count) in current_pairs.iter() {
        if frequency_table.contains_key(&c) {
            frequency_table.insert(*c, frequency_table[&c] + 1 * count);
        } else {
            frequency_table.insert(*c, 1 * count);
        }
    }

    let most_common_count = frequency_table.values().max().unwrap();
    let least_common_count = frequency_table.values().min().unwrap();

    most_common_count - least_common_count + 1
}
