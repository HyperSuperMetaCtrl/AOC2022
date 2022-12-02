use anyhow::Result;
use std::{env, fs::read_to_string};

const FILENAME: &str = "input.txt";
// how to represent moves?
// how to represent win/lose conditions
//
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

fn score_by_choice(round: &str) -> usize {
    let my_choice = round.split(" ").nth(1).unwrap_or("");
    // calc score based on choice
    match my_choice {
        ROCK => ROCK_VAL,
        PAPER => PAPER_VAL,
        SCISSORS => SCISSORS_VAL,
        _ => 0,
    }
}

fn score_by_win(round: &str) -> usize {
    let enemy_choice = round.split(" ").nth(0).unwrap_or("");
    let my_choice = round.split(" ").nth(1).unwrap_or("");
    match (enemy_choice, my_choice) {
        (EN_ROCK, PAPER) => WIN,
        (EN_PAPER, SCISSORS) => WIN,
        (EN_SCISSORS, ROCK) => WIN,

        (EN_ROCK, SCISSORS) => LOSE,
        (EN_PAPER, ROCK) => LOSE,
        (EN_SCISSORS, PAPER) => LOSE,

        (EN_ROCK, ROCK) | (EN_PAPER, PAPER) | (EN_SCISSORS, SCISSORS) => DRAW,
        _ => 0,
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() >= 2 { &args[1] } else { FILENAME };
    let score: usize = read_to_string(path)?
        .lines()
        .map(|round| score_by_choice(round) + score_by_win(round))
        .sum();

    println!("Day 2 part 1: {score}");
    Ok(())
}
