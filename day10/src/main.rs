use anyhow::Result;
use std::{fmt::Display, fs::read_to_string};

const CRT_WITDH: usize = 40;
const CRT_HEIGHT: usize = 6;
const SPRITE_WIDTH: usize = 3;
#[derive(Clone)]
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
}
impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        let max_cycle = instructions.len();
        Self {
            x: 1,
            queue: None,
            pc: 0,
            cycle: 0,
            instructions,
        }
    }
    fn execute_instruction(&mut self) -> Option<(usize, isize)> {
        let old = self.x;
        if let Some(val) = self.queue {
            self.x += val;
            self.cycle += 1;
            self.queue = None;
            return Some((self.cycle, old));
        }
        match self.instructions.get(self.pc)? {
            Instruction::Addx(val) => {
                self.queue = Some(*val);
                self.pc += 1;
            }
            Instruction::Noop => self.pc += 1,
        }
        self.cycle += 1;
        Some((self.cycle, old))
    }
}
struct Screen {
    mem: Vec<bool>,
    cursor: usize,
    sprite_pos: usize,
}
impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Self {
            mem: vec![false; width * height],
            cursor: 0,
            sprite_pos: 1,
        }
    }
    fn advance_cursor(&mut self) {
        self.cursor += 1;
    }
    fn draw(&mut self) {
        let cursor_pos = self.cursor % CRT_WITDH;
        if (self.sprite_pos.saturating_sub(1)..=((self.sprite_pos + 1) % CRT_WITDH))
            .contains(&cursor_pos)
        {
            self.mem.insert(self.cursor, true);
        }
    }
    fn set_sprite_pos(&mut self, pos: usize) {
        self.sprite_pos = pos % CRT_WITDH;
    }
}
fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let instructions = input
        .lines()
        .map(|line| line.into())
        .collect::<Vec<Instruction>>();
    let mut cpu = Cpu::new(instructions.clone());
    let cycles = [20, 60, 100, 140, 180, 220];
    let mut signal_strength: Vec<isize> = vec![];
    while let Some((cycle, x)) = cpu.execute_instruction() {
        if cycles.contains(&cycle) {
            println!("{}*{}={}", cycle, x, cycle as isize * x);
            signal_strength.push(cycle as isize * x);
        }
    }
    println!("Day 10 Part 1: {}", signal_strength.iter().sum::<isize>());

    let mut cpu = Cpu::new(instructions.clone());
    let mut screen = Screen::new(CRT_WITDH, CRT_HEIGHT);

    while let Some((_, x)) = cpu.execute_instruction() {
        screen.set_sprite_pos(x as usize);
        screen.draw();
        screen.advance_cursor();
    }
    print!("{}", screen);
    Ok(())
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split(" ").collect();
        match split.len() {
            1 => Instruction::Noop,
            2 => Instruction::Addx(split[1].parse::<isize>().unwrap()),
            _ => Instruction::Noop,
        }
    }
}
const NONE: &str = ".";
const SOME: &str = "#";

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.mem.chunks(CRT_WITDH) {
            for c in row {
                if *c {
                    write!(f, "{}", SOME)?;
                } else {
                    write!(f, "{}", NONE)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
