use anyhow::Result;
use std::{env, fs::File, io::Read};

const FILENAME: &str = "input.txt";

fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    let path = if args.len() >= 2 { &args[1] } else { FILENAME };

    let mut file = File::open(path)?;
    let mut input = Vec::new();
    file.read_to_end(&mut input)?;

    let index = input
        .into_iter()
        .as_slice()
        .windows(4)
        .enumerate()
        .filter(|(_, s)| {
            !((s[0] == s[1])
                || (s[0] == s[2])
                || (s[0] == s[3])
                || (s[1] == s[2])
                || (s[1] == s[3])
                || (s[2] == s[3])) //meh
        })
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    println!("Day 6 Part 1: {}", index[0] + 4);
    Ok(())
}
