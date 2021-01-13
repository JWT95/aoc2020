use crate::common::read_input;
use anyhow::Result;

const TREE: char = '#';

type Slope = Vec<Vec<char>>;

fn count_trees(slope: &Slope, right: u8, down: u8) -> usize {
    let slope_width = slope[0].len();
    (0..(slope.len() / down as usize))
        .map(|i| (i * down as usize, i * right as usize))
        .map(|coords| slope[coords.0][coords.1 % slope_width])
        .filter(|x| *x == TREE)
        .count()
}

pub fn day_03() -> Result<()> {
    let slope: Slope = read_input("input/day_03.txt")?
        .map(|line| line.chars().collect())
        .collect();

    let output = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |acc, (right, down)| {
            acc * count_trees(&slope, *right, *down)
        });
    println!("{:?}", output);

    Ok(())
}
