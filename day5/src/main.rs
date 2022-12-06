use anyhow::{anyhow, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{env, fs::read_to_string, fmt::Display};

const FILENAME: &str = "input.txt";
const NUM_STACKS: usize = 9;

#[derive(Debug)]
struct Layer(Vec<MaybeCrate>);

#[derive(Debug)]
struct Stack(Vec<Crate>);

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.last().unwrap())
    }
}

#[derive(Debug, Clone)]
struct MaybeCrate(Option<char>);

#[derive(Debug, Clone)]
struct Crate(char);
impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<char> for MaybeCrate {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_alphabetic() && value.is_uppercase() {
            Ok(MaybeCrate(Some(value)))
        } else if value.is_whitespace() {
            Ok(MaybeCrate(None))
        } else {
            Err("MaybeCrate has to be a Uppercase Letter or Space")
        }
    }
}

fn parse_crates(line: &str) -> Layer {
    lazy_static! {
        //shoud fail at compile if Regex is not correct
        static ref RE: Regex = Regex::new(r"(\[(?:[A-Z])\]|\s(?:\s)\s|\s(?:\d)\s)\s?").unwrap();
    };
    Layer(
        RE.captures_iter(line)
            .filter_map(|x| {
                MaybeCrate::try_from(x[1].chars().nth(1).expect("Regex didn't match")).ok()
            })
            .collect(),
    )
}

fn stack_layers(layers: Vec<Layer>) -> Vec<Stack> {
    let mut v: Vec<Stack> = Vec::new();
    for _ in 0..NUM_STACKS {
        v.push(Stack(Vec::new()))
    }
    for layer in layers.iter().rev() {
        for (i, cr) in layer.0.iter().enumerate() {
            if let MaybeCrate(Some(c)) = cr {
                v[i].0.push(Crate(*c))
            }
        }
    }
    v
}
#[derive(Debug)]
struct Instruction {
    how_many: usize,
    from: usize,
    to: usize,
}
fn parse_instruction(line: &str) -> Result<Instruction> {
    let instr: Vec<&str> = line.split(" ").collect();
    Ok(Instruction {
        how_many: str::parse::<usize>(instr[1])?,
        from: str::parse::<usize>(instr[3])? - 1, // -1 to convert from stack to vec index
        to: str::parse::<usize>(instr[5])? - 1,
    })
}
fn rearrange(stacks: &mut Vec<Stack>, instructions: Vec<Instruction>) {
    for instruction in instructions {
        let from = instruction.from;
        let to = instruction.to;
        for _ in 0..instruction.how_many {
            let item = stacks[from].0.pop();
            stacks[to].0.push(item.unwrap());
        }
    }
}

fn result_part1(stacks: Vec<Stack>) {
    print!("Day 5 Part 1:");
    for s in stacks {
        print!("{}", s);
    }
    println!();
}
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() >= 2 { &args[1] } else { FILENAME };

    let input = read_to_string(path)?;

    let (start_configuration, instructions) = input
        .split("\n\n")
        .collect_tuple()
        .expect("could not collect into tuple");

    let mut layers: Vec<Layer> = start_configuration
        .lines()
        .map(|line| parse_crates(line))
        .collect();
    layers.pop();

    let instructions: Vec<Instruction> = instructions
        .lines()
        .filter_map(|line| parse_instruction(line).ok())
        .collect();

    let mut stacks = stack_layers(layers);
    
    rearrange(&mut stacks, instructions);
    result_part1(stacks);

    Ok(())
}
