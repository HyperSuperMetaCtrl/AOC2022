use anyhow::Result;
use std::fs::read_to_string;

use ndarray::prelude::*;
use ndarray::Array2;

fn read_input_to_array() -> Result<(Array2<i32>, Array2<i32>)> {
    let input = read_to_string("input.txt")?;
    let input: Vec<Vec<i32>> = input
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let rows = input.len();
    let colums = input[0].len();

    let mut array = Array2::<i32>::zeros((rows, colums));
    let mut array1 = Array2::<i32>::zeros((rows, colums));

    for (i, row) in input.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            array[[i, j]] = *column;
        }
    }
    Ok((array, array1))
}

fn main() -> Result<()> {
    let (array, mut array1) = read_input_to_array()?;

    array.indexed_iter().for_each(|(a, b)| {
        if *b
            > *array
                .slice(s![..a.0, a.1])
                .as_slice()
                .unwrap()
                .iter()
                .max()
                .unwrap()
            || *b
                > *array
                    .slice(s![a.0.., a.1])
                    .as_slice()
                    .unwrap()
                    .iter()
                    .max()
                    .unwrap()
        {
            array1[[a.0, a.1]] = 1;
        }
    });
    Ok(())
}
