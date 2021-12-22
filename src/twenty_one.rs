use std::cmp;
use std::collections::hash_map::Entry;
use std::collections::{BTreeMap, HashMap};
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let mut state = read_input();
    let answer = solve_1(&mut state);

    println!("{}", answer);
}

pub fn part_two() {
    let state = read_input();
    let answer = solve_2(&state);

    println!("{}", answer);
}

type PlayerId = i32;
type Position = i32;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct QuantumnState {
    game_state: GameState,
    current_player: PlayerId,
}

impl QuantumnState {
    fn new(state: &GameState, current_player: PlayerId) -> QuantumnState {
        QuantumnState {
            game_state: state.clone(),
            current_player,
        }
    }

    fn get_children(&self) -> Vec<QuantumnState> {
        let mut children: Vec<QuantumnState> = vec![];
        for r_1 in 1..=3 {
            for r_2 in 1..=3 {
                for r_3 in 1..=3 {
                    let mut state = self.clone();

                    state
                        .game_state
                        .advance_prerolled(state.current_player, &[r_1, r_2, r_3]);

                    state.current_player += 1;
                    if state.current_player > 2 {
                        state.current_player = 1;
                    }

                    children.push(state);
                }
            }
        }

        children
    }
}

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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

    fn advance_prerolled(&mut self, player: PlayerId, rolls: &[i32]) {
        let roll_total: i32 = rolls.iter().sum();

        let mut new_position = self.player_positions[&player] + roll_total;
        while new_position > 10 {
            new_position -= 10;
        }

        self.player_positions.insert(player, new_position);
        *self.scores.entry(player).or_default() += new_position;

        /*println!(
            "Player {} rolls {:?} and moves to {} for a total of {}",
            player, rolls, new_position, self.scores[&player]
        );*/
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

fn add_results(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
    (a.0 + b.0, a.1 + b.1)
}

fn calc_records(
    state: &QuantumnState,
    cache: &mut HashMap<QuantumnState, (u64, u64)>,
) -> (u64, u64) {
    if let Entry::Occupied(records) = cache.entry(state.clone()) {
        return *records.get();
    }

    if *state.game_state.scores.values().max().unwrap() >= 21 {
        let prev_player = (state.current_player) % 2 + 1;

        return match prev_player {
            1 => (1, 0),
            2 => (0, 1),
            _ => panic!(),
        };
    }

    let records = state
        .get_children()
        .iter()
        .map(|child| calc_records(child, cache))
        .fold((0, 0), add_results);

    cache.insert(state.clone(), records);

    records
}

fn solve_2(state: &GameState) -> u64 {
    let state = QuantumnState::new(state, 1);

    /*let mut records: HashMap<PlayerId, u64> = HashMap::new();
    records.insert(1, 0);
    records.insert(2, 0);*/

    let mut cache: HashMap<QuantumnState, (u64, u64)> = HashMap::new();
    let records = calc_records(&state, &mut cache);

    //*records.values().max().unwrap()
    cmp::max(records.0, records.1)
}
