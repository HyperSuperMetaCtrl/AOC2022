use anyhow::Result;
use std::{env, fs::read_to_string};

mod filesystem;

use filesystem::*;

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

fn add_files_and_dirs(fs: &mut FileSystem, files: Vec<&str>) -> Result<()> {
    for file in files {
        let file: Vec<_> = file.split(" ").collect();
        match file[0] {
            "dir" => fs.mkdir(file[1])?,
            _ => fs.mkfile(file[1], file[0].parse::<usize>().unwrap())?,
        };
    }
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() >= 2 { &args[1] } else { FILENAME };
    let input = read_to_string(path)?;
    let parsed = parse_input(&input);

    let mut fs = FileSystem::new();

    for command in parsed {
        match command {
            Command::LS(files) => add_files_and_dirs(&mut fs, files)?,
            Command::CD(directory) => fs.cd(directory)?,
        }
    }
    dbg!(fs);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {}
}
