use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let scanners = read_input();
    let answer = solve_1(&scanners);

    println!("{}", answer);
}

pub fn part_two() {
    let scanners = read_input();
    //let answer = get_life_support_rating(&scanners);

    //println!("{}", answer);
}

type ScannerId = i32;

#[derive(Debug)]
struct Scanner {
    id: ScannerId,
    beacons: Vec<RelativePosition>,
}

impl Scanner {
    fn from_lines(lines: &[String]) -> Scanner {
        let id_str = lines[0].split("scanner ").collect::<Vec<&str>>()[1]
            .split(' ')
            .collect::<Vec<&str>>()[0];

        let id: i32 = id_str.parse().unwrap();

        let beacons: Vec<RelativePosition> = lines[1..lines.len()]
            .iter()
            .map(|l| RelativePosition::from_str(l))
            .collect();

        Scanner { id, beacons }
    }
}

#[derive(Debug)]
struct RelativePosition {
    x: i32,
    y: i32,
    z: i32,
}

impl RelativePosition {
    fn dist(&self, other: &RelativePosition) -> RelativeDistance {
        RelativeDistance::new(other.x - self.x, other.y - self.y, other.z - self.z)
    }

    fn from_str(pos_str: &str) -> RelativePosition {
        let parts: Vec<&str> = pos_str.split(',').collect();

        let x: i32 = parts[0].parse().unwrap();
        let y: i32 = parts[1].parse().unwrap();
        let z: i32 = parts[2].parse().unwrap();

        RelativePosition { x, y, z }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct RelativeDistance {
    x: i32,
    y: i32,
    z: i32,
}

impl RelativeDistance {
    fn new(x: i32, y: i32, z: i32) -> RelativeDistance {
        RelativeDistance { x, y, z }
    }

    fn possible_matches(&self, other: &RelativeDistance) -> Vec<(Rotation, Rotation)> {
        let mut possible_matches = vec![];
        for (self_rot, self_dist) in self.get_rotations() {
            for (other_rot, other_dist) in other.get_rotations() {
                if self_dist == other_dist {
                    possible_matches.push((self_rot, other_rot));
                }
            }
        }

        possible_matches
    }

    fn get_rotations(&self) -> Vec<(Rotation, RelativeDistance)> {
        panic!()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Sign {
    Positive,
    Negative,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Rotation {
    axes: [(Dir, Sign); 3],
}

fn read_input() -> Vec<Scanner> {
    let stdin = io::stdin();

    let mut scanners: Vec<Scanner> = Vec::new();
    let mut scanner_lines: Vec<String> = vec![];
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            scanners.push(Scanner::from_lines(&scanner_lines));
            scanner_lines = vec![];
        } else {
            scanner_lines.push(line.to_string());
        }
    }

    scanners
}

fn find_possible_scanner_relations(
    scanner_1: &Scanner,
    scanner_2: &Scanner,
) -> HashMap<(Rotation, Rotation), i32> {
    let mut possible_relations = HashMap::new();

    for beacon_1 in scanner_1.beacons.iter() {
        for beacon_2 in scanner_2.beacons.iter() {
            let possible_beacon_relations: Vec<(Rotation, Rotation)> = panic!();

            for (r1, r2) in possible_beacon_relations.iter() {
                *possible_relations.entry((*r1, *r2)).or_insert(0) += 1;
            }
        }
    }

    possible_relations
}

fn find_relations(
    scanners: &[Scanner],
) -> HashMap<ScannerId, Vec<(Rotation, Rotation, ScannerId)>> {
    let mut relations: HashMap<ScannerId, Vec<(Rotation, Rotation, ScannerId)>> = HashMap::new();

    for scanner_1 in scanners.iter() {
        for scanner_2 in scanners.iter() {
            let mut possible_relations: Vec<((Rotation, Rotation), i32)> =
                find_possible_scanner_relations(scanner_1, scanner_2)
                    .iter()
                    .map(|(a, b)| (*a, *b))
                    .collect();

            possible_relations = possible_relations
                .into_iter()
                .filter(|((_, _), count)| *count >= 12)
                .collect();

            if possible_relations.len() > 2 {
                panic!(); // bad, shouldn't get here, if so then algorithm is insufficient
            } else if possible_relations.len() == 1 {
                let ((rotation_1, rotation_2), _) = possible_relations[0];
                relations.entry(scanner_1.id).or_insert(vec![]).push((
                    rotation_1,
                    rotation_2,
                    scanner_2.id,
                ));
            }
        }
    }

    relations
}

fn solve_1(scanners: &[Scanner]) -> i32 {
    let scanner_relations = find_relations(scanners);

    // TODO: collapse relative possitions onto scanner 0

    // TODO: return count of beacons

    panic!()
}
