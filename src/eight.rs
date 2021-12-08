use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let entries = read_input();
    let answer = get_num_1_4_7_8(&entries);

    println!("{}", answer);
}

pub fn part_two() {
    let entries = read_input();
    let answer = solve_2(&entries);

    println!("{}", answer);
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

fn vec_to_set(vec: &[char]) -> HashSet<char> {
    vec.iter().copied().collect()
}

fn find_wire_segment_connections(entry: &(Vec<String>, Vec<String>)) -> HashMap<char, char> {
    let letters: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    // Enumerate all the possible connections, so we can later narrow them down
    let mut connection_possibilities: HashMap<char, HashSet<char>> = HashMap::new();
    for c1 in letters.iter() {
        connection_possibilities.insert(*c1, vec_to_set(&letters));
    }

    // Narrow down the possibilities based on unique line output digits
    let mut unique_digits_lookup: HashMap<i32, HashSet<char>> = HashMap::new();
    unique_digits_lookup.insert(2, vec_to_set(&['c', 'f'])); // 1 [2 segments]
    unique_digits_lookup.insert(3, vec_to_set(&['a', 'c', 'f'])); // 7 [3 segments]
    unique_digits_lookup.insert(4, vec_to_set(&['b', 'c', 'd', 'f'])); // 4 [4 segments]
                                                                       // 2, 3, 5 are non-constraining [5 segments]
                                                                       // 0, 6, 9 are non-constraining [6 segments]
                                                                       // 8 is non-constraining [7 segments]

    let (signal_patterns, _) = entry;
    for pattern in signal_patterns {
        if let Some(dest) = unique_digits_lookup.get(&(pattern.len() as i32)) {
            for source_c in pattern.chars() {
                let new_possibilities: HashSet<char> = connection_possibilities
                    .get_mut(&source_c)
                    .unwrap()
                    .intersection(dest)
                    .cloned()
                    .collect();

                connection_possibilities.insert(source_c, new_possibilities);
            }
        }
    }

    // Use a CSP solver to figure out the remaining uncertaininty in the connections
    let connections_vecd = solve_csp(&connection_possibilities, signal_patterns).unwrap();

    let mut connections: HashMap<char, char> = HashMap::new();
    for (c, ps) in connections_vecd.iter() {
        connections.insert(*c, *ps.iter().last().unwrap());
    }

    connections
}

fn meets_digits_constraints(
    connection_possibilities: &HashMap<char, HashSet<char>>,
    signal_patterns: &[String],
) -> bool {
    let mut connections: HashMap<char, char> = HashMap::new();
    for (c, ps) in connection_possibilities.iter() {
        connections.insert(*c, *ps.iter().last().unwrap());
    }

    let mut mapped_signals: Vec<Vec<char>> = signal_patterns
        .iter()
        .map(|p| p.chars().map(|c| *connections.get(&c).unwrap()).collect())
        .collect();

    let _: Vec<_> = mapped_signals
        .iter_mut()
        .map(|s| s.sort_unstable())
        .collect();

    let mut digit_mapping: HashMap<Vec<char>, i32> = HashMap::new();
    digit_mapping.insert(vec!['a', 'b', 'c', 'e', 'f', 'g'], 0);
    digit_mapping.insert(vec!['c', 'f'], 1);
    digit_mapping.insert(vec!['a', 'c', 'd', 'e', 'g'], 2);
    digit_mapping.insert(vec!['a', 'c', 'd', 'f', 'g'], 3);
    digit_mapping.insert(vec!['b', 'c', 'd', 'f'], 4);
    digit_mapping.insert(vec!['a', 'b', 'd', 'f', 'g'], 5);
    digit_mapping.insert(vec!['a', 'b', 'd', 'e', 'f', 'g'], 6);
    digit_mapping.insert(vec!['a', 'c', 'f'], 7);
    digit_mapping.insert(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'], 8);
    digit_mapping.insert(vec!['a', 'b', 'c', 'd', 'f', 'g'], 9);

    for digit in digit_mapping.keys() {
        if !mapped_signals.contains(digit) {
            return false;
        }
    }

    true
}

fn solve_csp(
    connection_possibilities: &HashMap<char, HashSet<char>>,
    signal_patterns: &[String],
) -> Option<HashMap<char, HashSet<char>>> {
    if connection_possibilities.values().all(|p| p.len() == 1) {
        if meets_digits_constraints(connection_possibilities, signal_patterns) {
            return Some(connection_possibilities.clone());
        } else {
            return None;
        }
    }

    let mut unconstrained: Vec<(&char, &HashSet<char>)> = connection_possibilities
        .iter()
        .filter(|(_, v)| v.len() > 1)
        .collect();

    unconstrained.sort_by(|(_, v1), (_, v2)| v1.len().cmp(&v2.len()));

    let already_matched: Vec<char> = connection_possibilities
        .iter()
        .filter(|(_, v)| v.len() == 1)
        .map(|(_, v)| *v.iter().last().unwrap())
        .collect();

    for (c, possibilities) in unconstrained.iter() {
        for p in possibilities.iter() {
            if !already_matched.contains(p) {
                let mut new_conn_poss = connection_possibilities.clone();
                new_conn_poss.insert(**c, vec_to_set(&[*p]));

                if let Some(result) = solve_csp(&new_conn_poss, signal_patterns) {
                    return Some(result);
                }
            }
        }
    }

    None
}

fn map_wires(output_value: &str, mappings: &HashMap<char, char>) -> String {
    output_value.chars().map(|c| mappings[&c]).collect()
}

fn solve_2(entries: &[(Vec<String>, Vec<String>)]) -> i32 {
    let mut digit_mapping: HashMap<Vec<char>, i32> = HashMap::new();
    digit_mapping.insert(vec!['a', 'b', 'c', 'e', 'f', 'g'], 0);
    digit_mapping.insert(vec!['c', 'f'], 1);
    digit_mapping.insert(vec!['a', 'c', 'd', 'e', 'g'], 2);
    digit_mapping.insert(vec!['a', 'c', 'd', 'f', 'g'], 3);
    digit_mapping.insert(vec!['b', 'c', 'd', 'f'], 4);
    digit_mapping.insert(vec!['a', 'b', 'd', 'f', 'g'], 5);
    digit_mapping.insert(vec!['a', 'b', 'd', 'e', 'f', 'g'], 6);
    digit_mapping.insert(vec!['a', 'c', 'f'], 7);
    digit_mapping.insert(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'], 8);
    digit_mapping.insert(vec!['a', 'b', 'c', 'd', 'f', 'g'], 9);

    let mut sum = 0;
    for entry in entries.iter() {
        let connections = find_wire_segment_connections(entry);

        let (_, output_values) = entry;
        let mut num = 0;
        for output_value in output_values.iter() {
            let output_value = map_wires(&output_value, &connections);
            let mut segments: Vec<char> = output_value.chars().collect();
            segments.sort_unstable();

            let digit = digit_mapping.get(&segments).unwrap();

            num *= 10;
            num += digit;
        }
        sum += num;
    }

    sum
}
