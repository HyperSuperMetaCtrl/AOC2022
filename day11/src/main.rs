use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit0, digit1, space0, space1},
    error::{Error as NomError, *},
    multi::separated_list0,
    sequence::tuple,
    Err::Failure,
    IResult,
};
use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;
impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("seen", &self.seen)
            .field("id", &self.id)
            .field("starting_items", &self.starting_items)
            .field("true_monkey", &self.true_monkey)
            .field("false_monkey", &self.false_monkey)
            .finish()
    }
}
struct Monkey {
    seen: usize,
    id: usize,
    starting_items: VecDeque<BigUint>,
    operation: Box<dyn Fn(&BigUint) -> BigUint>,
    test: Box<dyn Fn(&BigUint) -> bool>,
    true_monkey: usize,
    false_monkey: usize,
}
#[derive(Clone)]
enum Op {
    Mul(BigUint),
    Sqr,
    Add(BigUint),
}

fn div_by(n: BigUint) -> Box<dyn Fn(&BigUint) -> bool> {
    Box::new(move |item| item % n.clone() == BigUint::zero())
}
fn op(op: Op, _divby: BigUint) -> Box<dyn Fn(&BigUint) -> BigUint> {
    let divby = 3.to_biguint().unwrap();
    match op {
        Op::Add(constant) => Box::new(move |old| ((old + constant.clone()) / divby.clone())),
        Op::Mul(constant) => Box::new(move |old| ((old * constant.clone()) / divby.clone())),
        Op::Sqr => Box::new(move |old| (old * old) / divby.clone()),
    }
}

fn monkey_parser(s: &str) -> IResult<&str, usize> {
    let (s, (_, id)) = tuple((tag("Monkey "), digit0))(s)?;
    Ok((s, id.parse::<usize>().unwrap()))
}

fn items_parser(input: &str) -> IResult<&str, VecDeque<BigUint>> {
    let (input, _) = tag("Starting items: ")(input)?;
    let (input, nums) = separated_list0(tag(", "), digit1)(input)?;
    Ok((
        input,
        nums.into_iter()
            .map(|num| num.parse::<BigUint>().unwrap())
            .collect(),
    ))
}

fn operation_parser(s: &str) -> IResult<&str, Op> {
    let (s, (_, op_str, _, num_str)) = tuple((
        tag("Operation: new = old "),
        anychar,
        space1,
        alt((digit1, tag("old"))),
    ))(s)?;
    let op;

    match op_str {
        '+' => op = Op::Add(num_str.parse::<BigUint>().unwrap()),
        '*' => match num_str {
            "old" => op = Op::Sqr,
            n => op = Op::Mul(n.parse::<BigUint>().unwrap()),
        },
        _ => return Err(Failure(NomError::new(s, ErrorKind::Fail))),
    }
    Ok((s, op))
}

fn test_parser(s: &str) -> IResult<&str, BigUint> {
    let (s, (_, div_by)) = tuple((tag("Test: divisible by "), digit1))(s)?;
    Ok((s, div_by.parse::<BigUint>().unwrap()))
}

fn if_parser(s: &str) -> IResult<&str, usize> {
    let (s, (_, _, _, monkey)) = tuple((
        tag("If "),
        alt((tag("true"), tag("false"))),
        tag(": throw to monkey "),
        digit1,
    ))(s)?;
    Ok((s, monkey.parse::<usize>().unwrap()))
}
fn parse_input() -> Result<Vec<RefCell<Monkey>>> {
    let raw_input = std::fs::read_to_string("input.txt")?;
    let input: Vec<String> = raw_input.split("\n\n").map(|s| s.to_string()).collect();
    let input: Vec<Vec<String>> = input
        .into_iter()
        .map(|monkey| monkey.lines().map(|line| line.trim().to_string()).collect())
        .collect();

    let mut monkeys: Vec<RefCell<Monkey>> = Vec::with_capacity(input.len());

    for monkey in input.into_iter() {
        let (_, id) = monkey_parser(&monkey[0]).unwrap();
        let (_, starting_items) = items_parser(&monkey[1]).unwrap();
        let (_, operation) = operation_parser(&monkey[2]).unwrap();
        let (_, test) = test_parser(&monkey[3]).unwrap();
        let operation = op(operation, test.clone());
        let test = div_by(test);
        let (_, true_monkey) = if_parser(&monkey[4]).unwrap();
        let (_, false_monkey) = if_parser(&monkey[5]).unwrap();

        monkeys.push(RefCell::new(Monkey {
            seen: 0,
            id,
            starting_items,
            operation,
            test,
            true_monkey,
            false_monkey,
        }))
    }
    Ok(monkeys)
}
fn main() -> Result<()> {
    let mut monkeys = parse_input()?;
    let rounds: usize = 20;

    //dbg!(&monkeys);
    for round in 0..rounds {
        for monkey in &monkeys {
            let seen = monkey.borrow().starting_items.len();
            monkey.borrow_mut().seen += seen;
            let new_items: Vec<BigUint> = monkey
                .borrow()
                .starting_items
                .iter()
                .map(|item| (monkey.borrow().operation)(&item))
                .collect();
            let positions: Vec<bool> = new_items
                .iter()
                .map(|item| (monkey.borrow().test)(&item))
                .collect();
            monkey.borrow_mut().starting_items.clear();
            for (position, item) in positions.into_iter().zip(new_items.into_iter()) {
                if position {
                    monkeys[monkey.borrow().true_monkey]
                        .borrow_mut()
                        .starting_items
                        .push_back(item);
                } else {
                    monkeys[monkey.borrow().false_monkey]
                        .borrow_mut()
                        .starting_items
                        .push_back(item);
                }
            }
        }
    }
    monkeys.sort_by(|a, b| a.borrow().seen.cmp(&b.borrow().seen));
    let monkey_business = monkeys
        .iter()
        .rev()
        .take(2)
        .fold(1, |acc, x| acc * x.borrow().seen);
    println!("Day 11 part 1: {}", monkey_business);

    Ok(())
}
