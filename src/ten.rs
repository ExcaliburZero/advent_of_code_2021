use std::collections::BTreeSet;
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let lines = read_input();
    let answer = calc_syntax_error_score(&lines);

    println!("{}", answer);
}

pub fn part_two() {
    let numbers = read_input();
    let answer = calc_autocomplete_score(&numbers);

    println!("{}", answer);
}

fn read_input() -> Vec<String> {
    let stdin = io::stdin();

    let mut lines: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        lines.push(line);
    }

    lines
}

const PAIRS: [(char, char); 4] = [('{', '}'), ('(', ')'), ('[', ']'), ('<', '>')];

fn get_corresponding_opening(c: char) -> char {
    for (o2, c2) in PAIRS.iter() {
        if *c2 == c {
            return *o2;
        }
    }

    panic!()
}

fn get_bad_char_score(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

fn score_errors(line: &str) -> i64 {
    let openings: Vec<char> = PAIRS.iter().map(|(o, _)| *o).collect();
    let closings: Vec<char> = PAIRS.iter().map(|(_, c)| *c).collect();

    let mut stack: Vec<char> = vec![];

    for c in line.chars() {
        if openings.contains(&c) {
            stack.push(c);
        } else if closings.contains(&c) {
            match stack.pop() {
                Some(o) => {
                    if o == get_corresponding_opening(c) {
                        // good
                    } else {
                        return get_bad_char_score(c);
                    }
                }
                None => {}
            }
        }
    }

    0
}

fn score_autocomplete(line: &str) -> Option<i64> {
    let openings: Vec<char> = PAIRS.iter().map(|(o, _)| *o).collect();
    let closings: Vec<char> = PAIRS.iter().map(|(_, c)| *c).collect();

    let mut stack: Vec<char> = vec![];

    for c in line.chars() {
        if openings.contains(&c) {
            stack.push(c);
        } else if closings.contains(&c) {
            match stack.pop() {
                Some(o) => {
                    if o == get_corresponding_opening(c) {
                        // good
                    } else {
                        // ignore
                        return None;
                    }
                }
                None => {
                    return None;
                }
            }
        }
    }

    stack.reverse();
    let mut score = 0;
    for c in stack.iter() {
        score *= 5;
        score += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!(),
        };
    }

    Some(score)
}

fn calc_syntax_error_score(lines: &[String]) -> i64 {
    let mut total_score = 0;

    for line in lines.iter() {
        total_score += score_errors(line)
    }

    total_score
}

fn calc_autocomplete_score(lines: &[String]) -> i64 {
    let mut scores: BTreeSet<i64> = BTreeSet::new();
    for line in lines.iter() {
        match score_autocomplete(line) {
            Some(s) => {
                scores.insert(s);
            }
            None => {}
        }
    }

    scores.iter().cloned().collect::<Vec<i64>>()[scores.len() / 2]
}
