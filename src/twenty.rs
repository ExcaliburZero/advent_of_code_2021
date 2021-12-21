use std::cmp;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let (algorithm, image) = read_input();
    let answer = solve_1(&algorithm, &image);

    println!("{}", answer);
}

pub fn part_two() {
    let (algorithm, image) = read_input();
    let answer = solve_2(&algorithm, &image);

    println!("{}", answer);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Color {
    Filled,
    Empty,
}

impl Color {
    fn from_char(color_char: char) -> Color {
        match color_char {
            '#' => Color::Filled,
            '.' => Color::Empty,
            _ => panic!(),
        }
    }

    fn to_char(self) -> char {
        match self {
            Color::Filled => '#',
            Color::Empty => '.',
        }
    }
}

#[derive(Debug)]
struct IEAlgorithm {
    settings: Vec<Color>,
}

impl IEAlgorithm {
    fn from_str(algorithm_str: &str) -> IEAlgorithm {
        let settings = algorithm_str.chars().map(Color::from_char).collect();

        IEAlgorithm { settings }
    }

    fn apply(&self, image: &Image) -> Image {
        let mut new_image = Image::new();

        for pos in image.get_dimensions().expanded(1).positions() {
            let current_values: [Color; 9] = image.get_neighborhood(&pos);

            let lookup_value = IEAlgorithm::calc_lookup_value(&current_values);
            let new_value = self.settings[lookup_value];

            new_image.set_pixel(&pos, new_value);
        }

        new_image.set_beyond(match image.get_beyond() {
            Color::Empty => self.settings[0],
            Color::Filled => self.settings[511],
        });

        new_image
    }

    fn calc_lookup_value(values: &[Color]) -> usize {
        let mut value = 0;

        for v in values.iter() {
            value <<= 1;
            value += match v {
                Color::Filled => 1,
                Color::Empty => 0,
            };
        }

        value
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    fn added(&self, other: &Position) -> Position {
        Position::new(self.x + other.x, self.y + other.y)
    }
}

#[derive(Clone, Copy, Debug)]
struct PixelRange {
    min: Position,
    max: Position,
}

impl PixelRange {
    fn new(min: &Position, max: &Position) -> PixelRange {
        PixelRange {
            min: *min,
            max: *max,
        }
    }

    fn expanded(&self, expand_amount: i32) -> PixelRange {
        let new_min = self
            .min
            .added(&Position::new(-expand_amount, -expand_amount));
        let new_max = self.max.added(&Position::new(expand_amount, expand_amount));

        PixelRange::new(&new_min, &new_max)
    }

    fn positions(&self) -> Vec<Position> {
        let mut positions = vec![];

        for y in self.min.y..=self.max.y {
            for x in self.min.x..=self.max.x {
                positions.push(Position::new(x, y));
            }
        }

        positions
    }
}

#[derive(Clone, Debug)]
struct Image {
    pixels: HashMap<Position, Color>,
    beyond: Color,
}

impl Image {
    fn new() -> Image {
        Image {
            pixels: HashMap::new(),
            beyond: Color::Empty,
        }
    }

    fn from_lines(lines: &[String]) -> Image {
        let mut pixels = HashMap::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let position = Position::new(x as i32, y as i32);
                let value = Color::from_char(c);

                pixels.insert(position, value);
            }
        }

        Image {
            pixels,
            beyond: Color::Empty,
        }
    }

    fn get_pixel(&self, position: &Position) -> Color {
        *self.pixels.get(position).unwrap_or(&self.beyond)
    }

    fn get_neighborhood(&self, center: &Position) -> [Color; 9] {
        let pixels: Vec<Color> = PixelRange::new(center, center)
            .expanded(1)
            .positions()
            .iter()
            .map(|p| self.get_pixel(p))
            .collect();

        [
            pixels[0], pixels[1], pixels[2], pixels[3], pixels[4], pixels[5], pixels[6], pixels[7],
            pixels[8],
        ]
    }

    fn get_dimensions(&self) -> PixelRange {
        let mut min_x = 99999999;
        let mut max_x = -99999999;
        let mut min_y = 99999999;
        let mut max_y = -99999999;

        for p in self.pixels.keys() {
            min_x = cmp::min(min_x, p.x);
            max_x = cmp::max(max_x, p.x);

            min_y = cmp::min(min_y, p.y);
            max_y = cmp::max(max_y, p.y);
        }

        let dims_min = Position::new(min_x, min_y);
        let dims_max = Position::new(max_x, max_y);

        PixelRange::new(&dims_min, &dims_max)
    }

    fn set_pixel(&mut self, position: &Position, color: Color) {
        self.pixels.insert(*position, color);
    }

    fn get_beyond(&self) -> Color {
        self.beyond
    }

    fn set_beyond(&mut self, value: Color) {
        self.beyond = value;
    }

    fn print(&self) {
        let dims = self.get_dimensions();
        for y in dims.min.y..=dims.max.y {
            for x in dims.min.x..=dims.max.x {
                let pos = Position::new(x, y);
                print!("{}", self.get_pixel(&pos).to_char());
            }
            println!();
        }
    }
}

fn read_input() -> (IEAlgorithm, Image) {
    let stdin = io::stdin();

    let mut algorithm: Option<IEAlgorithm> = None;
    let mut image_lines: Vec<String> = vec![];
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            continue;
        }

        if algorithm.is_none() {
            algorithm = Some(IEAlgorithm::from_str(&line));
            continue;
        }

        image_lines.push(line.to_string());
    }

    let image = Image::from_lines(&image_lines);

    (algorithm.unwrap(), image)
}

fn solve_1(algorithm: &IEAlgorithm, image: &Image) -> i32 {
    let mut image: Image = image.clone();

    image.print();
    println!("------------------");

    image = algorithm.apply(&image);
    image.print();
    println!("------------------");

    image = algorithm.apply(&image);
    image.print();
    println!("------------------");

    image
        .pixels
        .values()
        .filter(|c| **c == Color::Filled)
        .count() as i32
}

fn solve_2(algorithm: &IEAlgorithm, image: &Image) -> i32 {
    let mut image: Image = image.clone();

    image.print();
    println!("------------------");

    for _ in 0..50 {
        image = algorithm.apply(&image);
        image.print();
        println!("------------------");
    }

    image
        .pixels
        .values()
        .filter(|c| **c == Color::Filled)
        .count() as i32
}
