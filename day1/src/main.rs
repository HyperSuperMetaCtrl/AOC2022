use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    // open the file
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    // part 1
    let mut calories = vec![0];
    for line in reader.lines() {
        if let Ok(s) = line {
            match s.as_str() {
                "" => calories.push(0),
                s => {
                    if let Some(number) = calories.last_mut() {
                        *number += s.parse::<i32>().expect("could not convert to i32")
                    }
                }
            }
        }
    }
    println!(
        "Solution to day 1, part 1: {}",
        calories.iter().max().expect("no maximum found")
    );

    calories.sort();
    let res = calories
        .iter()
        .rev()
        .take(3)
        .fold(0 ,|acc, c| acc + c);

    print!("Part 2: {}", res);
}
