use anyhow::Result;
use std::fs::read_to_string;

fn main() -> Result<()>{
    // open the file
    let input = read_to_string("input.txt")?;
    // part 1
    let mut calories = vec![0];
    for line in input.lines() {
        match line {
            "" => calories.push(0),
            s => {
                if let Some(number) = calories.last_mut() {
                    *number += s.parse::<i32>()?
                }
            }
        }
    }
    println!(
        "Day 1 Part 1: {}",
        calories.iter().max().expect("no maximum found")
    );

    calories.sort();
    let res = calories
        .iter()
        .rev()
        .take(3)
        .fold(0 ,|acc, c| acc + c);

    print!("Day 1 Part 2: {}", res);

    Ok(())
}
