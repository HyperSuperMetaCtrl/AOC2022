use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    // open the file
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    // part 1
    let mut calories = vec![0];
    for line in reader.lines() {
        match line.unwrap().as_str() {
            "" => calories.push(0),
            s => {
                if let Some(number) = calories.last_mut() {
                    *number += s.parse::<i32>().unwrap()
                }
            }
        }
    }
    println!(
        "Solution to day 1, part 1: {}",
        calories.iter().max().unwrap()
    );
}
