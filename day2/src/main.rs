use anyhow::Result;
use const_format::concatcp;
use std::{env, fs::read_to_string};

const FILENAME: &str = "input.txt";

const ROCK_VAL: usize = 1;
const PAPER_VAL: usize = 2;
const SCISSORS_VAL: usize = 3;

const ROCK: &str = "X";
const PAPER: &str = "Y";
const SCISSORS: &str = "Z";

const EN_ROCK: &str = "A";
const EN_PAPER: &str = "B";
const EN_SCISSORS: &str = "C";

const LOSE: usize = 0;
const DRAW: usize = 3;
const WIN: usize = 6;

const SHOULD_LOSE: &str = "X";
const SHOULD_DRAW: &str = "Y";
const SHOULD_WIN: &str = "Z";

fn extract_choices(round: &str) -> (&str, &str) {
    let enemy_choice = round.split(" ").nth(0).unwrap_or("");
    let my_choice = round.split(" ").nth(1).unwrap_or("");
    (enemy_choice, my_choice)
}

fn score_by_choice(round: &str) -> usize {
    let (_, my_choice) = extract_choices(round);
    // calc score based on choice
    match my_choice {
        ROCK => ROCK_VAL,
        PAPER => PAPER_VAL,
        SCISSORS => SCISSORS_VAL,
        _ => 0,
    }
}

fn score_by_win(round: &str) -> usize {
    let (enemy_choice, my_choice) = extract_choices(round);
    match (enemy_choice, my_choice) {
        (EN_ROCK, PAPER) | (EN_PAPER, SCISSORS) | (EN_SCISSORS, ROCK) => WIN,
        (EN_ROCK, SCISSORS) | (EN_PAPER, ROCK) | (EN_SCISSORS, PAPER) => LOSE,
        (EN_ROCK, ROCK) | (EN_PAPER, PAPER) | (EN_SCISSORS, SCISSORS) => DRAW,
        _ => 0,
    }
}

fn transform_input(round: &str) -> &str {
    let (enemy_choice, wanted_outcome) = extract_choices(round);
    match (enemy_choice, wanted_outcome) {
        (EN_ROCK, SHOULD_LOSE) => concatcp!(EN_ROCK," ",SCISSORS),
        (EN_ROCK, SHOULD_DRAW) => concatcp!(EN_ROCK," ",ROCK),
        (EN_ROCK, SHOULD_WIN) =>  concatcp!(EN_ROCK," ",PAPER),
        (EN_PAPER, SHOULD_LOSE) => concatcp!(EN_PAPER," ",ROCK),
        (EN_PAPER, SHOULD_DRAW) => concatcp!(EN_PAPER," ",PAPER),
        (EN_PAPER, SHOULD_WIN) => concatcp!(EN_PAPER," ",SCISSORS),
        (EN_SCISSORS, SHOULD_LOSE) => concatcp!(EN_SCISSORS," ",PAPER),
        (EN_SCISSORS, SHOULD_DRAW) => concatcp!(EN_SCISSORS," ",SCISSORS),
        (EN_SCISSORS, SHOULD_WIN) => concatcp!(EN_SCISSORS," ",ROCK),
        (_,_) => ""
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() >= 2 { &args[1] } else { FILENAME };
    let input = read_to_string(path)?;
    let score: usize = input
        .lines()
        .map(|round| score_by_choice(round) + score_by_win(round))
        .sum();
    println!("Day 2 part 1: {score}");

    let score: usize = input
        .lines()
        .map(|line| transform_input(line))
        .map(|round| score_by_choice(round)+score_by_win(round))
        .sum();
    println!("Day 2 part 2: {score}");

    Ok(())
}
