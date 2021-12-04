use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let (numbers, mut boards) = read_input();
    let answer = find_first_winning_board(&numbers, &mut boards).unwrap();

    println!("{}", answer);
}

pub fn part_two() {
    let (numbers, mut boards) = read_input();
    let answer = find_last_winning_board(&numbers, &mut boards).unwrap();

    println!("{}", answer);
}

#[derive(Debug)]
struct Board {
    tiles: [[i32; 5]; 5],
    markings: [[bool; 5]; 5],
}

impl Board {
    fn mark(&mut self, num: i32) {
        for r in 0..self.tiles.len() {
            for c in 0..self.tiles[0].len() {
                if self.tiles[r][c] == num {
                    self.markings[r][c] = true;
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        // Check rows
        for r in 0..self.tiles.len() {
            let mut num_marked = 0;
            for c in 0..self.tiles[0].len() {
                if self.markings[r][c] {
                    num_marked += 1;
                }
            }

            if num_marked == self.tiles[0].len() {
                return true;
            }
        }

        // Check columns
        for c in 0..self.tiles[0].len() {
            let mut num_marked = 0;
            for r in 0..self.tiles.len() {
                if self.markings[r][c] {
                    num_marked += 1;
                }
            }

            if num_marked == self.tiles.len() {
                return true;
            }
        }

        // Check diagonals
        /*let cases: Vec<(i32, i32, i32)> = vec![(0, 0, 1), (4, 4, -1)];
        for (start_r, start_c, dir) in cases.iter() {
            let mut num_marked = 0;
            let mut r = *start_r;
            let mut c = *start_c;

            while r >= 0 && r < self.tiles.len() as i32 && c >= 0 && c < self.tiles[0].len() as i32
            {
                if self.markings[r as usize][c as usize] {
                    num_marked += 1;
                }

                r += dir;
                c += dir;
            }

            if num_marked == self.tiles.len() {
                return true;
            }
        }*/

        false
    }

    fn sum_unmarked_nums(&self) -> i32 {
        let mut sum = 0;
        for r in 0..self.tiles.len() {
            for c in 0..self.tiles[0].len() {
                if !self.markings[r][c] {
                    sum += self.tiles[r][c];
                }
            }
        }

        sum
    }

    fn from_lines(lines: &[String]) -> Option<Board> {
        let mut tiles: [[i32; 5]; 5] = [[0; 5]; 5];
        for (r, line) in lines.iter().enumerate() {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            for (c, num) in numbers.iter().enumerate() {
                tiles[r][c] = *num;
            }
        }

        let markings = [[false; 5]; 5];

        Some(Board { tiles, markings })
    }
}

fn read_input() -> (Vec<i32>, Vec<Board>) {
    let stdin = io::stdin();

    let mut boards: Vec<Board> = Vec::new();
    let mut lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let numbers: Vec<i32> = lines
        .first()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    lines.remove(0);

    let mut current_board_lines: Vec<String> = vec![];
    for line in lines.iter() {
        if line.is_empty() {
            if !current_board_lines.is_empty() {
                boards.push(Board::from_lines(&current_board_lines).unwrap());

                current_board_lines.clear();
            }
            continue;
        }

        current_board_lines.push(line.to_string());
    }

    if !current_board_lines.is_empty() {
        boards.push(Board::from_lines(&current_board_lines).unwrap());

        current_board_lines.clear();
    }

    (numbers, boards)
}

fn find_first_winning_board(numbers: &[i32], boards: &mut [Board]) -> Option<i32> {
    for num in numbers.iter() {
        for b in boards.iter_mut() {
            b.mark(*num);

            if b.has_won() {
                return Some(b.sum_unmarked_nums() * num);
            }
        }
    }

    None
}

fn find_last_winning_board(numbers: &[i32], boards: &mut [Board]) -> Option<i32> {
    let mut last_win_number = 0;
    let mut won_boards: Vec<usize> = vec![];
    for num in numbers.iter() {
        for (i, b) in boards.iter_mut().enumerate() {
            b.mark(*num);

            if b.has_won() && !won_boards.contains(&i) {
                last_win_number = b.sum_unmarked_nums() * num;
                won_boards.push(i);
            }
        }
    }

    Some(last_win_number)
}
