use anyhow::Result;
use std::env;
use std::fs::read_to_string;
const FILENAME: &str = "input.txt";

#[derive(Clone, Copy)]
struct BitField(u64);

impl From<&str> for BitField {
    fn from(s: &str) -> Self {
        let bits_to_set: Vec<usize> = s.chars().map(|c| Self::char2index(c)).collect();
        BitField::from(bits_to_set)
    }
}

impl From<Vec<usize>> for BitField {
    fn from(v: Vec<usize>) -> Self {
        let mut bitfield = BitField::new();

        for bit in v {
            bitfield.set_bit(bit);
        }
        bitfield
    }
}

impl BitField {
    fn new() -> Self {
        BitField(0)
    }
    fn char2index(c: char) -> usize {
        match c {
            c if c.is_lowercase() => (c as usize) - 96,
            c if c.is_uppercase() => (c as usize) - 38,
            _ => 0,
        }
    }
    fn set_bit(&mut self, bit: usize) {
        let mut bits = self.0;
        bits = bits | 1 << bit;
        self.0 = bits;
    }

    fn prio(a: BitField, b: BitField) -> usize {
        let mut priority = a.0 & b.0;
        let mut count = 0;
        while priority > 1 {
            priority >>= 1;
            count += 1;
        }
        count
    }

    fn prio_3(a: BitField, b: BitField, c: BitField) -> usize {
        let mut priority = a.0 & b.0 & c.0;
        let mut count = 0;
        while priority > 1 {
            priority >>= 1;
            count += 1;
        }
        count
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() >= 2 { &args[1] } else { FILENAME };
    let input = read_to_string(path)?;

    let sum: usize = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| (BitField::from(left), BitField::from(right)))
        .map(|(a, b)| BitField::prio(a, b))
        .sum();
    println!("Day 3 Part 1: {sum}");
    let sum: usize = input
        .lines()
        .map(|line| BitField::from(line))
        .collect::<Vec<BitField>>()
        .chunks_exact(3)
        .map(|x| BitField::prio_3(x[0], x[1], x[2]))
        .sum();
    println!("Day 3 Part 2: {sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prio() {
        let a = BitField(2);
        let b = BitField(2);
        assert_eq!(1, BitField::prio(a, b));
    }
    #[test]
    fn splitting() {
        let s = "1234";
        let split = s.split_at(s.len() / 2);
        assert_eq!(("12", "34"), split);
    }
    #[test]
    fn set_bit() {
        let mut bs = BitField::new();
        bs.set_bit(1);
        assert_eq!(bs.0, 2);
    }
    #[test]
    fn char2usize() {
        assert_eq!('A' as usize, 65);
    }
}
