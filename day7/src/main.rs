use anyhow::Result;

use std::{env, fs::read_to_string};

use day7::tree::*;

const FILENAME: &str = "input.txt";

#[derive(Debug)]
enum Command<'a> {
    LS(Vec<&'a str>),
    CD(&'a str),
}
fn parse_input(input: &str) -> Vec<Command> {
    let input: Vec<Vec<&str>> = input
        .split("$ ")
        .skip(1)
        .map(|x| x.lines().collect()) // split output at newlines
        .collect();

    input
        .into_iter()
        .map(|v| match v[0] {
            "ls" => Command::LS(v.into_iter().skip(1).collect::<Vec<&str>>()), //get files and dirs
            "cd .." => Command::CD(".."),
            _ => Command::CD(v[0].split(" ").nth(1).unwrap()), //get destination
        })
        .collect()
}
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() >= 2 { &args[1] } else { FILENAME };
    let input = read_to_string(path)?;
    let parsed = parse_input(&input);

    let mut pwd: Vec<String> = vec![];
    let mut pwd_id: NodeId;

    // for command in parsed {
    //     match command {
    //         Command::LS(children) => todo!(),
    //         Command::CD("..") => todo!(),
    //         Command::CD(directory) => todo!(),
    //     }
    // }

    let mut fs = Tree::<File>::new();

    Ok(())
}

#[derive(Debug)]
struct File<'a> {
    name: &'a str,
    size: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
