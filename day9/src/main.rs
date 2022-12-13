use anyhow::Result;
use std::{
    cell::{RefCell, RefMut},
    collections::HashSet,
    error::Error,
    fs::read_to_string,
};

mod point;
use point::*;
const UP: Point = Point { x: 0, y: 1 };
const DOWN: Point = Point { x: 0, y: -1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };

#[derive(Debug, Copy, Clone)]
enum Direction {
    L,
    R,
    U,
    D,
}
impl TryFrom<&str> for Direction {
    type Error = &'static str;
    fn try_from(direction: &str) -> Result<Self, Self::Error> {
        match direction {
            "L" => Ok(Direction::L),
            "R" => Ok(Direction::R),
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            _ => Err("Invalid Direction"),
        }
    }
}

impl TryFrom<&str> for Command {
    type Error = Box<dyn Error>;
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let command: Vec<_> = line.split(" ").collect();
        let direction = Direction::try_from(*command.get(0).ok_or("could not get direction")?)?;
        let amount = command
            .get(1)
            .ok_or("could not get amount")?
            .parse::<usize>()?;

        Ok(Command { direction, amount })
    }
}
struct Rope {
    head: Point,
    tail: Vec<RefCell<Point>>,
    seen: HashSet<Point>,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    amount: usize,
}

impl Rope {
    pub fn new(length: usize) -> Self {
        let mut seen = HashSet::new();
        seen.insert((0, 0).into());
        Rope {
            head: (0, 0).into(),
            tail: vec![RefCell::new((0, 0).into()); length - 1],
            seen,
        }
    }
    pub fn move_head(&mut self, command: Command) {
        for _ in 0..command.amount {
            match command.direction {
                Direction::L => self.head += LEFT,
                Direction::R => self.head += RIGHT,
                Direction::U => self.head += UP,
                Direction::D => self.head += DOWN,
            }
            self.tail.iter().fold(self.head, |last, knot| {
                self.move_tail(knot.borrow_mut(), last)
            });
            self.seen.insert(*self.tail.last().unwrap().borrow());
        }
    }
    fn move_tail<'a>(&'a self, mut knot: RefMut<'a, Point>, last: Point) -> Point {
        let dist = knot.dist(last);

        if knot.abs_dist(last) >= 2 {
            *knot += dist.sign();
        }
        knot.clone()
    }
    pub fn seen(&self) -> usize {
        self.seen.len()
    }
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    for (i, len) in [2, 10].iter().enumerate() {
        let mut rope = Rope::new(*len);
        input
            .lines()
            .map(|line| Command::try_from(line).unwrap())
            .collect::<Vec<Command>>()
            .into_iter()
            .for_each(|command| rope.move_head(command));

        println!("Day 9 part {}: {}", i, rope.seen());
    }
    Ok(())
}
