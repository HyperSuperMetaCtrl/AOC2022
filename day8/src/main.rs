use anyhow::Result;
use std::fs::read_to_string;

use ndarray::prelude::*;
use ndarray::Array2;

const ROWS: usize = 99;
const COLS: usize = 99;

fn read_input_to_array() -> Result<(Array2<i32>, Array2<i32>)> {
    let input = read_to_string("input.txt")?;
    let input: Vec<Vec<i32>> = input
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let mut array = Array2::<i32>::zeros((ROWS, COLS));
    let array1 = Array2::<i32>::zeros((ROWS, COLS));

    for (i, row) in input.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            array[[i, j]] = *column;
        }
    }
    Ok((array, array1))
}

fn main() -> Result<()> {
    let (array, mut array1) = read_input_to_array()?;

    for (idx, elem) in array.indexed_iter() {
        let slice_right = array.slice(s![..idx.0, idx.1]);
        let slice_left = array.slice(s![idx.0 + 1.., idx.1]);
        let slice_up = array.slice(s![idx.0, ..idx.1]);
        let slice_down = array.slice(s![idx.0, idx.1 + 1..]);

        if *elem > *slice_right.iter().max().unwrap_or(&-1)
            || *elem > *slice_left.iter().max().unwrap_or(&-1)
            || *elem > *slice_up.iter().max().unwrap_or(&-1)
            || *elem > *slice_down.iter().max().unwrap_or(&-1)
        {
            array1[[idx.0, idx.1]] = 1;
        }
    }
    let sum = array1.sum();
    println!("Day 8 part1: {}", sum);
    let mut scores = Vec::new();
    for (idx, elem) in array.indexed_iter() {
        let slice_left = array.slice(s![..idx.0;-1, idx.1]);
        let slice_right = array.slice(s![(idx.0 + 1).., idx.1]);
        let slice_up = array.slice(s![idx.0, ..idx.1;-1]);
        let slice_down = array.slice(s![idx.0, (idx.1 + 1)..]);

        let mut right = 0;
        let mut left = 0;
        let mut up = 0;
        let mut down = 0;
        for (i, other) in slice_right.iter().enumerate() {
            right = i + 1;
            if elem <= other {
                break;
            }
        }
        for (i, other) in slice_left.iter().enumerate() {
            left = i + 1;
            if elem <= other {
                break;
            }
        }
        for (i, other) in slice_up.iter().enumerate() {
            up = i + 1;
            if elem <= other {
                break;
            }
        }
        for (i, other) in slice_down.iter().enumerate() {
            down = i + 1;
            if elem <= other {
                break;
            }
        }
        scores.push(left * right * up * down);
    }
    println!("Day 8 part 2: {}", scores.iter().max().unwrap());

    Ok(())
}
