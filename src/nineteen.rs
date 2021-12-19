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

    fn rotated_beacons(&self, rotation: &Rotation) -> Vec<RelativePosition> {
        self.beacons.iter().map(|p| p.rotated(rotation)).collect()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct RelativePosition {
    x: i32,
    y: i32,
    z: i32,
}

impl RelativePosition {
    fn dist(&self, other: &RelativePosition) -> RelativeDistance {
        RelativeDistance::new(other.x - self.x, other.y - self.y, other.z - self.z)
    }

    fn new(x: i32, y: i32, z: i32) -> RelativePosition {
        RelativePosition { x, y, z }
    }

    fn from_str(pos_str: &str) -> RelativePosition {
        let parts: Vec<&str> = pos_str.split(',').collect();

        let x: i32 = parts[0].parse().unwrap();
        let y: i32 = parts[1].parse().unwrap();
        let z: i32 = parts[2].parse().unwrap();

        RelativePosition { x, y, z }
    }

    fn rotated(&self, rotation: &Rotation) -> RelativePosition {
        let new_x = self.get_axis_value(&rotation.axes[0].0) * rotation.axes[0].1.to_i32();
        let new_y = self.get_axis_value(&rotation.axes[1].0) * rotation.axes[1].1.to_i32();
        let new_z = self.get_axis_value(&rotation.axes[2].0) * rotation.axes[2].1.to_i32();

        RelativePosition::new(new_x, new_y, new_z)
    }

    fn get_axis_value(&self, axis: &Dir) -> i32 {
        match axis {
            Dir::X => self.x,
            Dir::Y => self.y,
            Dir::Z => self.z,
        }
    }

    fn offsetted(&self, offset: &RelativePosition) -> RelativePosition {
        RelativePosition::new(self.x + offset.x, self.y + offset.y, self.z + offset.z)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RelativeDistance {
    x: i32,
    y: i32,
    z: i32,
}

impl RelativeDistance {
    fn new(x: i32, y: i32, z: i32) -> RelativeDistance {
        RelativeDistance { x, y, z }
    }

    fn to_pos(&self) -> RelativePosition {
        RelativePosition::new(self.x, self.y, self.z)
    }

    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Dir {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Sign {
    Positive,
    Negative,
}

impl Sign {
    fn to_i32(self) -> i32 {
        match self {
            Sign::Positive => 1,
            Sign::Negative => -1,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Rotation {
    axes: [(Dir, Sign); 3],
}

impl Rotation {
    fn all() -> Vec<Rotation> {
        let signs = vec![Sign::Positive, Sign::Negative];

        let mut rotations = vec![];
        for x_pos in 0..3 {
            for y_pos in 0..3 {
                if y_pos == x_pos {
                    continue;
                }

                for z_pos in 0..3 {
                    if z_pos == x_pos || z_pos == y_pos {
                        continue;
                    }

                    for x_sign in signs.iter() {
                        for y_sign in signs.iter() {
                            for z_sign in signs.iter() {
                                let mut axes = [(Dir::X, Sign::Positive); 3];

                                axes[x_pos] = (Dir::X, *x_sign);
                                axes[y_pos] = (Dir::Y, *y_sign);
                                axes[z_pos] = (Dir::Z, *z_sign);

                                rotations.push(Rotation { axes });
                            }
                        }
                    }
                }
            }
        }

        rotations
    }
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

    scanners.push(Scanner::from_lines(&scanner_lines));
    scanner_lines = vec![];

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

fn find_relations___old(
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

fn calc_beacon_neighbors(
    beacons: &[RelativePosition],
    num_neighbors: i32,
) -> HashMap<RelativePosition, Vec<RelativeDistance>> {
    let mut beacon_neighbors: HashMap<RelativePosition, Vec<RelativeDistance>> = HashMap::new();

    for b_1 in beacons.iter() {
        let mut neighbors = beacons
            .iter()
            .filter(|b_2| b_1 != *b_2)
            .map(|b_2| b_1.dist(b_2))
            .collect::<Vec<RelativeDistance>>();

        neighbors.sort_by_key(|dist| dist.manhattan());

        beacon_neighbors.insert(
            b_1.clone(),
            neighbors
                .iter()
                .cloned()
                .take(num_neighbors as usize)
                .collect(),
        );
    }

    beacon_neighbors
}

fn find_potential_offsets(
    beacons_1: &[RelativePosition],
    beacons_2: &[RelativePosition],
) -> Vec<RelativePosition> {
    let beacon_neighbors_1 = calc_beacon_neighbors(beacons_1, 3);
    let beacon_neighbors_2 = calc_beacon_neighbors(beacons_2, 3);

    /*for (b, neighbors) in beacon_neighbors_1.iter() {
        println!("    {:?} next to {:?}", b, neighbors);
    }*/

    let mut possible_offsets = vec![];
    for (b_1, neighbors_1) in beacon_neighbors_1.iter() {
        for (b_2, neighbors_2) in beacon_neighbors_2.iter() {
            if neighbors_1 == neighbors_2 {
                //let offset = b_1.dist(b_2).to_pos();
                let offset = b_2.dist(b_1).to_pos();

                println!("    --------");
                println!("    Potential offset = {:?}", offset);
                println!("    b_1 = {:?}, {:?}", b_1, neighbors_1);
                println!("    b_2 = {:?}, {:?}", b_2, neighbors_2);

                possible_offsets.push(offset);
            }
        }
    }

    possible_offsets
}

fn find_best_offset(
    beacons_1: &[RelativePosition],
    beacons_2: &[RelativePosition],
) -> Option<(RelativePosition, i32)> {
    /*for x_offset in -2000..2001 {
    println!("    x_offset = {}", x_offset);
    for y_offset in -2000..2001 {
        for z_offset in -2000..2001 {
            let offset = RelativePosition::new(x_offset, y_offset, z_offset);*/

    for offset in find_potential_offsets(beacons_1, beacons_2) {
        println!("    offset = {:?}", offset);
        let mut num_matches = 0;
        for b_1 in beacons_1.iter() {
            //let b_1 = b_1.offsetted(&offset);
            for b_2 in beacons_2.iter() {
                let b_2 = b_2.offsetted(&offset);

                if *b_1 == b_2 {
                    num_matches += 1;
                }
            }
        }

        println!("    num_matches = {}", num_matches);
        if num_matches >= 12 {
            println!("    ***");
            return Some((offset, num_matches));
        }
    }
    /*}
        }
    }*/

    None
}

fn find_relations(
    scanners: &[Scanner],
) -> HashMap<ScannerId, Vec<(Rotation, Rotation, RelativePosition, ScannerId)>> {
    let mut relations = HashMap::new();

    println!("Finding relations...");
    for scanner_1 in scanners.iter() {
        for scanner_2 in scanners.iter() {
            println!("Scanners = ({}, {})", scanner_1.id, scanner_2.id);
            if scanner_1.id == scanner_2.id {
                continue;
            }

            // TODO: don't have an r_1
            //for r_1 in Rotation::all().iter() {
            let r_1 = Rotation::all()[0];
            for r_2 in Rotation::all().iter() {
                println!("  Rotations = ({:?}, {:?})", r_1, r_2);
                let beacons_1 = scanner_1.rotated_beacons(&r_1);
                let beacons_2 = scanner_2.rotated_beacons(r_2);
                println!("  Applied rotations");
                println!("  ({:?} into {:?})", scanner_1.beacons[0], beacons_1[0]);
                println!("  ({:?} into {:?})", scanner_2.beacons[0], beacons_2[0]);

                match find_best_offset(&beacons_1, &beacons_2) {
                    Some((shift, _)) => {
                        relations
                            .entry(scanner_1.id)
                            .or_insert_with(Vec::new)
                            .push((r_1, *r_2, shift, scanner_2.id));
                    }
                    None => (),
                }
            }
            //}
        }
    }

    relations
}

fn solve_1(scanners: &[Scanner]) -> i32 {
    let scanner_relations = find_relations(scanners);

    //println!("scanner_relations = {:?}", scanner_relations);

    for (scanner_id_1, related_scanners) in scanner_relations {
        for (r_1, r_2, offset_2, scanner_id_2) in related_scanners.iter() {
            println!(
                "{} => {:?} {:?} {:?} {}",
                scanner_id_1, r_1, r_2, offset_2, scanner_id_2
            );
        }
    }

    // TODO: collapse relative possitions onto scanner 0

    // TODO: return count of beacons

    panic!()
}
