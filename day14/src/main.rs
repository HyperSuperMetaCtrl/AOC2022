use anyhow::Result;
use itertools::Itertools;
use ndarray::Array2;

#[derive(Debug)]
struct Shape {
    coordinates: Vec<(usize, usize)>,
}

enum Material {
    Air,
    Solid,
    Sand,
}

struct CaveSystem {
    map: Array2<Material>,
    spawn: (usize, usize),
}

fn parse_input(path: &str) -> Result<Vec<Shape>> {
    let input = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = input.lines().collect();
    let splits: Vec<Vec<&str>> = lines
        .into_iter()
        .map(|line| line.split("->").collect::<Vec<&str>>())
        .collect();
    let trimmed: Vec<Vec<&str>> = splits
        .into_iter()
        .map(|x| x.into_iter().map(|y| y.trim()).collect())
        .collect();
    let tuples: Vec<Shape> = trimmed
        .into_iter()
        .map(|shape| Shape {
            coordinates: shape
                .into_iter()
                .map(|item| {
                    item.split(",")
                        .collect::<Vec<&str>>()
                        .into_iter()
                        .map(|item| item.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect::<Vec<(usize, usize)>>(),
        })
        .collect();
    Ok(tuples)
}

fn main() -> Result<()> {
    let parsed_input = parse_input("input.txt");
    dbg!(parsed_input);
    Ok(())
}
