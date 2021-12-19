use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let scanners = read_input();
    let answer = solve_1(&scanners);

    println!("{}", answer);
}

pub fn part_two() {
    let scanners = read_input();
    let answer = solve_2(&scanners);

    println!("{}", answer);
}

type ScannerId = i32;

#[derive(Clone, Debug)]
struct Scanner {
    id: ScannerId,
    beacons: HashSet<RelativePosition>,
}

impl Scanner {
    fn from_lines(lines: &[String]) -> Scanner {
        let id_str = lines[0].split("scanner ").collect::<Vec<&str>>()[1]
            .split(' ')
            .collect::<Vec<&str>>()[0];

        let id: i32 = id_str.parse().unwrap();

        let beacons: HashSet<RelativePosition> = lines[1..lines.len()]
            .iter()
            .map(|l| RelativePosition::from_str(l))
            .collect();

        Scanner { id, beacons }
    }

    fn rotated_beacons(&self, rotation: &Rotation) -> HashSet<RelativePosition> {
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

    fn times(&self, other: &Sign) -> Sign {
        use Sign::*;

        match (self, other) {
            (Positive, Positive) => Positive,
            (Negative, Negative) => Positive,
            _ => Negative,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Rotation {
    axes: [(Dir, Sign); 3],
}

impl Rotation {
    fn rotated(&self, other: &Rotation) -> Rotation {
        let mut axes = [(Dir::X, Sign::Positive); 3];

        let mut new_x = self.get_axis_value(&other.axes[0].0);
        new_x.1 = new_x.1.times(&other.axes[0].1);

        let mut new_y = self.get_axis_value(&other.axes[1].0);
        new_y.1 = new_y.1.times(&other.axes[1].1);

        let mut new_z = self.get_axis_value(&other.axes[2].0);
        new_z.1 = new_z.1.times(&other.axes[2].1);

        axes[0] = new_x;
        axes[1] = new_y;
        axes[2] = new_z;

        Rotation { axes }
    }

    fn get_axis_value(&self, axis: &Dir) -> (Dir, Sign) {
        match axis {
            Dir::X => self.axes[0],
            Dir::Y => self.axes[1],
            Dir::Z => self.axes[2],
        }
    }

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

fn calc_beacon_neighbors(
    beacons: &HashSet<RelativePosition>,
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
    beacons_1: &HashSet<RelativePosition>,
    beacons_2: &HashSet<RelativePosition>,
) -> Vec<RelativePosition> {
    let beacon_neighbors_1 = calc_beacon_neighbors(beacons_1, 3);
    let beacon_neighbors_2 = calc_beacon_neighbors(beacons_2, 3);

    let mut possible_offsets = vec![];
    for (b_1, neighbors_1) in beacon_neighbors_1.iter() {
        for (b_2, neighbors_2) in beacon_neighbors_2.iter() {
            if neighbors_1 == neighbors_2 {
                let offset = b_2.dist(b_1).to_pos();

                possible_offsets.push(offset);
            }
        }
    }

    possible_offsets
}

fn find_best_offset(
    beacons_1: &HashSet<RelativePosition>,
    beacons_2: &HashSet<RelativePosition>,
) -> Option<(RelativePosition, i32)> {
    for offset in find_potential_offsets(beacons_1, beacons_2) {
        let mut num_matches = 0;
        for b_1 in beacons_1.iter() {
            for b_2 in beacons_2.iter() {
                let b_2 = b_2.offsetted(&offset);

                if *b_1 == b_2 {
                    num_matches += 1;
                }
            }
        }

        if num_matches >= 12 {
            return Some((offset, num_matches));
        }
    }

    None
}

fn find_relations(
    scanners: &[Scanner],
) -> HashMap<ScannerId, Vec<(Rotation, RelativePosition, ScannerId)>> {
    let mut relations = HashMap::new();

    for scanner_1 in scanners.iter() {
        for scanner_2 in scanners.iter() {
            if scanner_1.id == scanner_2.id {
                continue;
            }

            for r_2 in Rotation::all().iter() {
                let beacons_2 = scanner_2.rotated_beacons(r_2);

                match find_best_offset(&scanner_1.beacons, &beacons_2) {
                    Some((shift, _)) => {
                        relations
                            .entry(scanner_1.id)
                            .or_insert_with(Vec::new)
                            .push((*r_2, shift, scanner_2.id));
                    }
                    None => (),
                }
            }
        }
    }

    relations
}

fn get_beacons(
    src: ScannerId,
    visited: &HashSet<ScannerId>,
    relations: &HashMap<ScannerId, Vec<(Rotation, RelativePosition, ScannerId)>>,
    scanners: &HashMap<ScannerId, Scanner>,
) -> HashSet<RelativePosition> {
    let mut beacons = scanners[&src].beacons.clone();
    for (rotation, offset, neighbor) in relations[&src].iter() {
        if visited.contains(neighbor) {
            continue;
        }

        let mut new_visited = visited.clone();
        new_visited.insert(src);

        let neighbor_beacons = get_beacons(*neighbor, &new_visited, relations, scanners);

        for b in neighbor_beacons {
            let b = b.rotated(rotation).offsetted(offset);

            beacons.insert(b);
        }
    }

    beacons
}

fn build_complete_map(
    scanners: &[Scanner],
    relations: &HashMap<ScannerId, Vec<(Rotation, RelativePosition, ScannerId)>>,
) -> Scanner {
    let mut visited = HashSet::new();
    visited.insert(0);

    let mut scanners_map = HashMap::new();
    for scanner in scanners.iter() {
        scanners_map.insert(scanner.id, scanner.clone());
    }

    let all_beacons = get_beacons(0, &visited, relations, &scanners_map);

    let mut all_beacons_vec: Vec<RelativePosition> = all_beacons.iter().cloned().collect();
    all_beacons_vec.sort_by_key(|p| (p.x, p.y, p.z));

    Scanner {
        id: 0,
        beacons: all_beacons,
    }
}

fn solve_1(scanners: &[Scanner]) -> i32 {
    let scanner_relations = find_relations(scanners);
    let total_scanner = build_complete_map(scanners, &scanner_relations);

    total_scanner.beacons.len() as i32
}

fn get_scanner_positions(
    src: ScannerId,
    src_pos: &RelativePosition,
    src_rotation: &Rotation,
    relations: &HashMap<ScannerId, Vec<(Rotation, RelativePosition, ScannerId)>>,
    scanner_positions: &mut HashMap<ScannerId, RelativePosition>,
) {
    for (rotation, offset, neighbor) in relations[&src].iter() {
        if scanner_positions.contains_key(neighbor) {
            continue;
        }

        let new_pos = src_pos.offsetted(&offset.rotated(src_rotation));
        scanner_positions.insert(*neighbor, new_pos.clone());

        let neighbor_beacons = get_scanner_positions(
            *neighbor,
            &new_pos,
            &rotation.rotated(src_rotation),
            relations,
            scanner_positions,
        );
    }
}

fn find_scanner_positions(
    scanners: &[Scanner],
    relations: &HashMap<ScannerId, Vec<(Rotation, RelativePosition, ScannerId)>>,
) -> HashMap<ScannerId, RelativePosition> {
    let mut scanner_positions: HashMap<ScannerId, RelativePosition> = HashMap::new();
    get_scanner_positions(
        0,
        &RelativePosition::new(0, 0, 0),
        &Rotation::all()[0],
        relations,
        &mut scanner_positions,
    );

    scanner_positions
}

fn solve_2(scanners: &[Scanner]) -> i32 {
    let scanner_relations = find_relations(scanners);

    let scanner_positions: HashMap<ScannerId, RelativePosition> =
        find_scanner_positions(scanners, &scanner_relations);

    let mut largest_dist = 0;
    for (id_1, pos_1) in scanner_positions.iter() {
        for (id_2, pos_2) in scanner_positions.iter() {
            if id_1 == id_2 {
                continue;
            }

            let dist = pos_1.dist(pos_2).manhattan();
            if dist > largest_dist {
                largest_dist = dist;
            }
        }
    }

    largest_dist
}
