use std::collections::BTreeMap;
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let mut state = read_input();
    let answer = solve_1(&mut state);

    println!("{}", answer);
}

pub fn part_two() {
    let state = read_input();
    //let answer = get_life_support_rating(&state);

    //println!("{}", answer);
}

type PlayerId = i32;
type Position = i32;

trait Dice {
    fn roll(&mut self) -> i32;
    fn get_num_times_rolled(&self) -> i32;
}

struct DeterministicDice {
    next_value: i32,
    num_times_rolled: i32,
}

impl DeterministicDice {
    fn new() -> DeterministicDice {
        DeterministicDice {
            next_value: 1,
            num_times_rolled: 0,
        }
    }
}

impl Dice for DeterministicDice {
    fn roll(&mut self) -> i32 {
        let value = self.next_value;

        self.next_value += 1;
        if self.next_value > 100 {
            self.next_value = 1;
        }

        self.num_times_rolled += 1;

        value
    }

    fn get_num_times_rolled(&self) -> i32 {
        self.num_times_rolled
    }
}

#[derive(Debug)]
struct GameState {
    player_positions: BTreeMap<PlayerId, Position>,
    scores: BTreeMap<PlayerId, i32>,
}

impl GameState {
    fn from_lines(lines: &[String]) -> GameState {
        let mut player_positions: BTreeMap<PlayerId, Position> = BTreeMap::new();
        let mut scores: BTreeMap<PlayerId, i32> = BTreeMap::new();

        for line in lines.iter() {
            let parts: Vec<&str> = line.split(' ').collect();

            let player_id: PlayerId = parts[1].parse().unwrap();
            let position: Position = parts[4].parse().unwrap();

            player_positions.insert(player_id, position);
            scores.insert(player_id, 0);
        }

        GameState {
            player_positions,
            scores,
        }
    }

    fn advance(&mut self, dice: &mut Box<dyn Dice>, player: PlayerId) {
        let rolls: Vec<i32> = (0..3).map(|_| dice.roll()).collect();
        let roll_total: i32 = rolls.iter().sum();

        let mut new_position = self.player_positions[&player] + roll_total;
        while new_position > 10 {
            new_position -= 10;
        }

        self.player_positions.insert(player, new_position);
        *self.scores.entry(player).or_default() += new_position;

        println!(
            "Player {} rolls {:?} and moves to {} for a total of {}",
            player, rolls, new_position, self.scores[&player]
        );
    }
}

fn read_input() -> GameState {
    let stdin = io::stdin();

    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().to_string())
        .collect();

    GameState::from_lines(&lines)
}

fn solve_1(state: &mut GameState) -> i32 {
    let mut dice: Box<dyn Dice> = Box::new(DeterministicDice::new());

    let mut current_player = *state.scores.keys().min().unwrap();
    while *state.scores.values().max().unwrap() < 1000 {
        state.advance(&mut dice, current_player);

        current_player += 1;
        if current_player > *state.scores.keys().max().unwrap() {
            current_player = *state.scores.keys().min().unwrap();
        }
    }

    let losing_player_score = *state
        .scores
        .iter()
        .min_by_key(|(_, score)| *score)
        .unwrap()
        .1;

    println!("losing_player_score: {}", losing_player_score);

    losing_player_score * dice.get_num_times_rolled()
}
