use std::fmt;
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let numbers = read_input();

    println!("{:?}", numbers);
    let answer = solve_1(numbers);

    println!("{}", answer);
}

pub fn part_two() {
    let numbers = read_input();
    //let answer = get_life_support_rating(&numbers);

    //println!("{}", answer);
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

#[derive(Clone, Debug)]
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
        //write!(f, "[{},{}]", self.x, self.y)
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
        //println!("------");
        //println!("self={:?}", self);
        //println!("path={:?}", path);

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
        //println!("------");
        //println!("self={:?}", self);
        //println!("path={:?}", path);

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
                println!("EXPLODE: {:?}", p);
                self.explode(&p);

                any_applied = true;
                continue;
            } else if let Some(p) = self.find_splitable() {
                println!("SPLIT: {:?}", p);
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

        println!("self = {}", self);
        *self.get_mut(path).unwrap() = Pair::Pair(
            Box::new(Pair::Element(new_left_value)),
            Box::new(Pair::Element(new_right_value)),
        );
        println!("self = {}", self);
    }

    fn explode(&mut self, path: &Path) {
        let exploding_pair = self.get(path).unwrap();
        let left_value = exploding_pair.left().unwrap().value().unwrap();
        let right_value = exploding_pair.right().unwrap().value().unwrap();

        let next_left_value_path = self.find_next_value_path(path, Dir::Left);
        if let Some(p) = next_left_value_path {
            println!("[left] p = {:?}", p);

            let prev_value = self.get(&p).unwrap().value().unwrap();

            println!("self = {}", self);
            *self.get_mut(&p).unwrap() = Pair::Element(prev_value + left_value);
            println!("self = {}", self);
        }

        let next_right_value_path = self.find_next_value_path(path, Dir::Right);
        if let Some(p) = next_right_value_path {
            println!("[right] p = {:?}", p);

            let prev_value = self.get(&p).unwrap().value().unwrap();

            println!("self = {}", self);
            *self.get_mut(&p).unwrap() = Pair::Element(prev_value + right_value);
            println!("self = {}", self);

            //panic!();
        }

        println!("self = {}", self);
        *self.get_mut(path).unwrap() = Pair::Element(0);
        println!("self = {}", self);

        //panic!()
    }

    fn find_next_value_path(&self, path: &Path, dir: Dir) -> Option<Path> {
        println!("find next, path = {:?}, dir = {:?}", path, dir);

        let mut new_path = path.clone(); // = path.pop();

        while !new_path.is_empty() {
            println!("path_part = {:?}", new_path);
            if let Some(d) = new_path.last() {
                println!("d = {:?}", d);
                new_path = new_path.pop();
                if d != dir {
                    println!("found! = {:?} -> {:?}", new_path, new_path.add(dir));
                    //return Some(new_path.add(dir));
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
            println!("===================");
            println!("self={}", self);
            println!("path={:?}", path);
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
        /*
        if pair_str.starts_with('[') {
            println!("{}", pair_str);
            let parts: (&str, &str) = pair_str.split_once(',').unwrap();

            let left_str = parts.0.chars().skip(1).collect::<String>();
            let right_str = parts.1.chars().collect::<String>()[0..(parts.1.len() - 1)].to_string();

            println!("left={}", left_str);
            println!("right={}", right_str);
            println!("r_0={}", parts.1.chars().collect::<String>());

            let left = Pair::from_str(&left_str);
            let right = Pair::from_str(&right_str);

            Pair::Pair(Box::new(left), Box::new(right))
        } else {
            println!("{}", pair_str);
            Pair::Element(pair_str.parse().unwrap())
        }
        */
        let mut stack: Vec<Result<Pair, Vec<Pair>>> = vec![];
        let mut i = 0;
        while i < pair_str.len() {
            if pair_str.chars().nth(i).unwrap() == '[' {
                // println!("[");
                stack.push(Err(vec![]));
                // println!("{:?}", stack);

                i += 1;
            } else if pair_str.chars().nth(i).unwrap() == ']' {
                // println!("]");
                // println!("{:?}", stack);
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

                        // println!("{:?}", stack.last());

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
                // println!("{:?}", stack);

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
                // println!("{}", part_str);
                let pair = Pair::Element(part_str.parse().unwrap());
                // println!("{:?}", stack);

                match stack.last_mut().unwrap() {
                    Err(p) => p.push(pair),
                    _ => panic!(),
                };
                // println!("{:?}", stack);

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
        println!("*********************");
        println!("pair = {}", pair);
        println!("p = {}", p);
        pair = pair.add(p);
    }

    pair.magnitude()
}
