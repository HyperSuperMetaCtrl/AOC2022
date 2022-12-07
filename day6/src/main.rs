use anyhow::Result;
use std::{env, fs::File, io::Read};

const FILENAME: &str = "input.txt";

struct BitSet(u32);

impl BitSet {
    fn new() -> Self {
        BitSet(0)
    }
    fn set(&mut self, bit: u32) {
        self.0 |= bit;
    }
    fn check(&self, bit: u32) -> bool {
        if self.0 & bit > 1 {
            true
        } else {
            false
        }
    }
}

fn find_marker(data: &Vec<u8>, marker_length: usize) -> Option<usize> {
    if let Some(index) = data
        .iter()
        .map(|b| 2_u32.pow((*b as u32).saturating_sub(97))) // convert ASCII-range 97-122 to 0-25
        .collect::<Vec<u32>>()
        .as_slice()
        .windows(marker_length)
        .enumerate()
        .filter(|(_, s)| {
            let mut bs = BitSet::new();
            for bit in s.into_iter() {
                if !bs.check(*bit) {
                    bs.set(*bit);
                } else {
                    return false;
                }
            }
            true
        })
        .map(|(i, _)| i)
        .collect::<Vec<_>>()
        .get(0)
    {
        Some(index + marker_length)
    } else {
        None
    }
}
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() >= 2 { &args[1] } else { FILENAME };

    let mut file = File::open(path)?;
    let mut input = Vec::new();
    file.read_to_end(&mut input)?;

    println!("Day 6 Part 1: {}", find_marker(&input, 4).unwrap());
    println!("Day 6 Part 2: {}", find_marker(&input, 14).unwrap());
    Ok(())
}
