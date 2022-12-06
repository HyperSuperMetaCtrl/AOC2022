use std::error::Error;

use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{env, fmt::Display, fs::read_to_string};

const FILENAME: &str = "input.txt";
const NUM_STACKS: usize = 9;

#[derive(Debug, Clone)]
struct Layer(Vec<MaybeCrate>);

impl TryFrom<&str> for Layer {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            //shoud fail at compile if Regex is not correct
            static ref RE: Regex = Regex::new(r"(\[(?:[A-Z])\]|\s(?:\s)\s|\s(?:\d)\s)\s?").unwrap();
        };
        Ok(Layer(
            RE.captures_iter(value)
                .filter_map(|x| {
                    MaybeCrate::try_from(x[1].chars().nth(1).expect("Regex didn't match")).ok()
                })
                .collect(),
        ))
    }
}

#[derive(Debug, Clone)]
struct Layers(Vec<Layer>);

impl Layers {
    fn stack(self) -> Stacks {
        let mut v: Vec<Stack> = Vec::new();
        for _ in 0..NUM_STACKS {
            v.push(Stack(Vec::new()))
        }
        for layer in self.0.iter().rev() {
            for (i, cr) in layer.0.iter().enumerate() {
                if let MaybeCrate(Some(c)) = cr {
                    v[i].0.push(Crate(*c))
                }
            }
        }
        Stacks(v)
    }
}

impl FromIterator<Layer> for Layers {
    fn from_iter<T: IntoIterator<Item = Layer>>(iter: T) -> Self {
        let mut v = Vec::new();

        for item in iter {
            v.push(item);
        }
        Layers(v)
    }
}
#[derive(Debug, Clone)]
struct Stack(Vec<Crate>);
#[derive(Debug, Clone)]
struct Stacks(Vec<Stack>);

impl Stacks {
    fn rearrange(mut self, instructions: &Vec<Instruction>) -> Self {
        for instruction in instructions {
            let from = instruction.from;
            let to = instruction.to;
            for _ in 0..instruction.how_many {
                let item = self.0[from].0.pop();
                self.0[to].0.push(item.unwrap());
            }
        }
        self
    }

    fn rearrange_9001(mut self, instructions: &Vec<Instruction>) -> Self {
        for instruction in instructions {
            let from = instruction.from;
            let to = instruction.to;
            let mut temp = Vec::new();
            for _ in 0..instruction.how_many {
                temp.push(self.0[from].0.pop().unwrap());
            }
            for item in temp.into_iter().rev() {
                self.0[to].0.push(item);
            }
        }
        self
    }
}
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

#[derive(Debug)]
struct Instruction {
    how_many: usize,
    from: usize,
    to: usize,
}

impl TryFrom<&str> for Instruction {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let instr: Vec<&str> = value.split(" ").collect();
        Ok(Instruction {
            how_many: str::parse::<usize>(instr[1])?,
            from: str::parse::<usize>(instr[3])? - 1, // -1 to convert from stack to vec index
            to: str::parse::<usize>(instr[5])? - 1,
        })
    }
}

fn result(stacks: Stacks, part: u8) {
    print!("Day 5 Part {part}: ");
    for s in stacks.0 {
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

    let mut layers: Layers = start_configuration
        .lines()
        .filter_map(|line| Layer::try_from(line).ok())
        .collect();
    layers.0.pop(); //remove the bottom layer

    let instructions: Vec<Instruction> = instructions
        .lines()
        .filter_map(|line| Instruction::try_from(line).ok())
        .collect();

    let stacks = layers.stack();

    let rearranged = stacks.clone().rearrange(&instructions);
    result(rearranged, 1);

    let rearranged = stacks.clone().rearrange_9001(&instructions);
    result(rearranged, 2);

    Ok(())
}
