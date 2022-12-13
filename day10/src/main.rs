use anyhow::Result;
use std::fs::read_to_string;

enum Instruction {
    Noop,
    Addx(isize),
}
struct Cpu {
    pub x: isize,
    queue: Option<isize>,
    pc: usize,
    cycle: usize,
    instructions: Vec<Instruction>,
    max_cycle: usize,
}
impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        let max_cycle = instructions.len();
        Self { x: 1 , queue: None, pc: 0, cycle: 0, instructions, max_cycle}
    }
    fn execute_instruction(&mut self) -> Option<(usize, isize)> {
        if self.pc == self.max_cycle { return None }
        let old = self.x;
        if let Some(val) = self.queue {
            self.x += val;
            self.cycle += 1;
            self.queue = None;
            return Some((self.cycle, old));
        }
        match self.instructions[self.pc] {
            Instruction::Addx(val) => {self.queue = Some(val); self.pc += 1;},
            Instruction::Noop => self.pc += 1,
        }
        self.cycle += 1;
        Some((self.cycle, old))
    }
}
fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let instructions = input.lines().map(|line| line.into()).collect::<Vec<Instruction>>();
    let mut cpu = Cpu::new(instructions);
    let cycles = [20, 60, 100, 140, 180, 220, 100000000];
    let mut intr = 0;
    let mut signal_strength:Vec<isize> = vec![];
    while let Some((cycle, x)) = cpu.execute_instruction() {
     if cycle == cycles[intr] {
         println!("{}*{}={}",cycle,x,cycle as isize * x);
         signal_strength.push(cycle as isize * x);
         intr += 1;
     }
    }
    print!("Day 10 Part 1: {}", signal_strength.iter().sum::<isize>());

    Ok(())
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split(" ").collect();
        match split.len() {
            1 => Instruction::Noop,
            2 => Instruction::Addx(split[1].parse::<isize>().unwrap()),
            _ => Instruction::Noop
        }
    }
}
