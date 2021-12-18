use std::fmt;
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let numbers = read_input();
    let answer = solve_1(numbers);

    println!("{}", answer);
}

pub fn part_two() {
    let numbers = read_input();
    let answer = solve_2(numbers);

    println!("{}", answer);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Dir {
    Left,
    Right,
}

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

#[derive(Clone, Debug)]
struct Path {
    dirs: Vec<Dir>,
}

impl Path {
    fn new() -> Path {
        Path { dirs: vec![] }
    }

    fn add(&self, dir: Dir) -> Path {
        let mut new_path = self.clone();
        new_path.dirs.push(dir);

        new_path
    }

    fn pop_front(&self) -> (Dir, Path) {
        let mut new_path = self.clone();
        let front = new_path.dirs[0];

        new_path.dirs.remove(0);

        (front, new_path)
    }

    fn pop(&self) -> Path {
        let mut new_path = self.clone();
        new_path.dirs.pop();

        new_path
    }

    fn is_empty(&self) -> bool {
        self.dirs.is_empty()
    }

    fn len(&self) -> usize {
        self.dirs.len()
    }

    fn last(&self) -> Option<Dir> {
        self.dirs.last().cloned()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Pair {
    Element(i32),
    Pair(Box<Pair>, Box<Pair>),
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pair::Element(v) => write!(f, "{}", v),
            Pair::Pair(left, right) => {
                write!(f, "[")?;
                left.fmt(f)?;
                write!(f, ",")?;
                right.fmt(f)?;
                write!(f, "]")
            }
        }
    }
}

impl Pair {
    fn magnitude(&self) -> i32 {
        match self {
            Pair::Element(value) => *value,
            Pair::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn add(&self, other: &Pair) -> Pair {
        let mut new_pair = Pair::Pair(Box::new(self.clone()), Box::new(other.clone()));

        new_pair.reduce();

        new_pair
    }

    fn value(&self) -> Option<i32> {
        match self {
            Pair::Element(v) => Some(*v),
            _ => None,
        }
    }

    fn left(&self) -> Option<&Pair> {
        match self {
            Pair::Pair(l, _) => Some(l),
            _ => None,
        }
    }

    fn right(&self) -> Option<&Pair> {
        match self {
            Pair::Pair(_, r) => Some(r),
            _ => None,
        }
    }

    fn get(&self, path: &Path) -> Option<&Pair> {
        if path.is_empty() {
            return Some(self);
        }

        match self {
            Pair::Element(_) => None,
            Pair::Pair(left, right) => {
                let (d, new_path) = path.pop_front();
                match d {
                    Dir::Left => left.get(&new_path),
                    Dir::Right => right.get(&new_path),
                }
            }
        }
    }

    fn get_mut(&mut self, path: &Path) -> Option<&mut Pair> {
        if path.is_empty() {
            return Some(self);
        }

        match self {
            Pair::Element(_) => None,
            Pair::Pair(left, right) => {
                let (d, new_path) = path.pop_front();
                match d {
                    Dir::Left => left.get_mut(&new_path),
                    Dir::Right => right.get_mut(&new_path),
                }
            }
        }
    }

    fn is_element(&self) -> bool {
        match self {
            Pair::Element(_) => true,
            Pair::Pair(_, _) => false,
        }
    }

    fn is_pair_of_elements(&self) -> bool {
        match self {
            Pair::Element(_) => false,
            Pair::Pair(left, right) => left.as_ref().is_element() && right.as_ref().is_element(),
        }
    }

    fn reduce(&mut self) {
        let mut any_applied = true;
        while any_applied {
            any_applied = false;

            if let Some(p) = self.find_explodeable() {
                self.explode(&p);

                any_applied = true;
                continue;
            } else if let Some(p) = self.find_splitable() {
                self.split(&p);

                any_applied = true;
                continue;
            }
        }
    }

    fn split(&mut self, path: &Path) {
        let value = self.get(path).unwrap().value().unwrap();

        let new_left_value = value / 2;
        let new_right_value = (value / 2) + (value % 2);

        *self.get_mut(path).unwrap() = Pair::Pair(
            Box::new(Pair::Element(new_left_value)),
            Box::new(Pair::Element(new_right_value)),
        );
    }

    fn explode(&mut self, path: &Path) {
        let exploding_pair = self.get(path).unwrap();
        let left_value = exploding_pair.left().unwrap().value().unwrap();
        let right_value = exploding_pair.right().unwrap().value().unwrap();

        let next_left_value_path = self.find_next_value_path(path, Dir::Left);
        if let Some(p) = next_left_value_path {
            let prev_value = self.get(&p).unwrap().value().unwrap();

            *self.get_mut(&p).unwrap() = Pair::Element(prev_value + left_value);
        }

        let next_right_value_path = self.find_next_value_path(path, Dir::Right);
        if let Some(p) = next_right_value_path {
            let prev_value = self.get(&p).unwrap().value().unwrap();

            *self.get_mut(&p).unwrap() = Pair::Element(prev_value + right_value);
        }

        *self.get_mut(path).unwrap() = Pair::Element(0);
    }

    fn find_next_value_path(&self, path: &Path, dir: Dir) -> Option<Path> {
        let mut new_path = path.clone();

        while !new_path.is_empty() {
            if let Some(d) = new_path.last() {
                new_path = new_path.pop();
                if d != dir {
                    return self.find_next_value_path_down(&new_path.add(dir), dir.opposite());
                }
            }
        }

        None
    }

    fn find_next_value_path_down(&self, path: &Path, dir: Dir) -> Option<Path> {
        let mut new_path = path.clone();

        loop {
            if let Some(pair) = self.get(&new_path) {
                if pair.is_element() {
                    return Some(new_path);
                } else {
                    new_path = new_path.add(dir);
                    continue;
                }
            }

            return None;
        }
    }

    fn find_splitable(&self) -> Option<Path> {
        self.find_splitable_(&Path::new())
    }

    fn find_splitable_(&self, path: &Path) -> Option<Path> {
        if let Some(pair) = self.get(path) {
            match pair {
                Pair::Element(value) => {
                    if *value >= 10 {
                        return Some(path.clone());
                    } else {
                        return None;
                    }
                }
                Pair::Pair(_, _) => {
                    if let Some(p) = self.find_splitable_(&path.add(Dir::Left)) {
                        return Some(p);
                    }
                    if let Some(p) = self.find_splitable_(&path.add(Dir::Right)) {
                        return Some(p);
                    }

                    return None;
                }
            }
        }

        None
    }

    fn find_explodeable(&self) -> Option<Path> {
        self.find_explodeable_(&Path::new())
    }

    fn find_explodeable_(&self, path: &Path) -> Option<Path> {
        if path.len() >= 4 {
            let p = self.get(path).unwrap();

            if p.is_element() {
                return None;
            }

            if p.is_pair_of_elements() {
                return Some(path.clone());
            }

            panic!()
        } else {
            if let Pair::Element(_) = self.get(path).unwrap() {
                return None;
            }

            for d in &[Dir::Left, Dir::Right] {
                let new_path = path.add(*d);

                if let Some(p) = self.find_explodeable_(&new_path) {
                    return Some(p);
                }
            }

            None
        }
    }

    fn from_str(pair_str: &str) -> Pair {
        let mut stack: Vec<Result<Pair, Vec<Pair>>> = vec![];
        let mut i = 0;
        while i < pair_str.len() {
            if pair_str.chars().nth(i).unwrap() == '[' {
                stack.push(Err(vec![]));

                i += 1;
            } else if pair_str.chars().nth(i).unwrap() == ']' {
                let elements = stack.pop().unwrap();
                match elements {
                    Err(elements) => {
                        let new_pair = Pair::Pair(
                            Box::new(elements[0].clone()),
                            Box::new(elements[1].clone()),
                        );

                        if stack.last().map(|p| p.is_err()).unwrap_or(false) {
                            match stack.last_mut().unwrap() {
                                Err(ps) => ps.push(new_pair),
                                _ => panic!(),
                            }
                        } else {
                            stack.push(Ok(new_pair));
                        }
                    }
                    Ok(pair) => {
                        let elements = stack.pop().unwrap().err().unwrap();

                        let new_pair = Pair::Pair(Box::new(elements[0].clone()), Box::new(pair));

                        if stack.last().map(|p| p.is_err()).unwrap_or(false) {
                            match stack.last_mut().unwrap() {
                                Err(ps) => ps.push(new_pair),
                                _ => panic!(),
                            }
                        } else {
                            stack.push(Ok(new_pair));
                        }
                    }
                }

                i += 1;
            } else if pair_str.chars().nth(i).unwrap() == ',' {
                i += 1;
            } else {
                let end_i = pair_str
                    .chars()
                    .enumerate()
                    .skip(i)
                    .find(|(_, c)| vec![',', ']'].contains(c))
                    .unwrap()
                    .0;

                let part_str = pair_str[i..end_i].to_string();
                let pair = Pair::Element(part_str.parse().unwrap());

                match stack.last_mut().unwrap() {
                    Err(p) => p.push(pair),
                    _ => panic!(),
                };

                i = end_i;
            }
        }

        stack.pop().unwrap().unwrap()
    }
}

fn read_input() -> Vec<Pair> {
    let stdin = io::stdin();

    let mut pairs: Vec<Pair> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        pairs.push(Pair::from_str(&line));
    }

    pairs
}

fn solve_1(pairs: Vec<Pair>) -> i32 {
    let mut pair = pairs[0].clone();
    pair.reduce();

    for p in pairs.iter().skip(1) {
        pair = pair.add(p);
    }

    pair.magnitude()
}

fn solve_2(pairs: Vec<Pair>) -> i32 {
    let mut max_magnitude = 0;

    for p1 in pairs.iter() {
        for p2 in pairs.iter() {
            if p1 != p2 {
                let sum = p1.add(p2);
                let magnitude = sum.magnitude();

                if magnitude > max_magnitude {
                    max_magnitude = magnitude;
                }
            }
        }
    }

    max_magnitude
}
