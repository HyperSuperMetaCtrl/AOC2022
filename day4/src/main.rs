use std::{env, fs::read_to_string};
use anyhow::Result;

const FILENAME: &str = "input.txt";

fn transform(line: &str) -> (u32, u32, u32, u32) {
        let split: Vec<&str> = line.split(",").collect();
        let lhs: Vec<u32> = split[0].split("-").map(|x| x.parse::<u32>().unwrap()).collect();
        let rhs: Vec<u32> = split[1].split("-").map(|x| x.parse::<u32>().unwrap()).collect();
        (lhs[0],rhs[0],lhs[1],rhs[1])
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() >= 2 { &args[1] } else { FILENAME };
    
    let intervals: Vec<(u32, u32, u32, u32)> = read_to_string(path)?
        .lines()
        .map(|line| transform(line))
        .collect();

    let sum: u32 = intervals.iter().map(|interval| {
        match interval {
            (l1,l2,r1,r2) if l1 <= l2 && r1 >= r2 => 1,
            (l1,l2,r1,r2) if l1 >= l2 && r1 <= r2 => 1,
            _ => 0,
        }
    }).sum();

    println!("Day 4 Part 1: {sum}");

    Ok(())
}
