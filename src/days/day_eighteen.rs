use crate::common::read_input;
use anyhow::Result;

pub fn day_eighteen() -> Result<()> {
    let inputs: Vec<Vec<char>> = read_input("input/day_eighteen.txt")?
        .map(|x| x.chars().collect())
        .collect();

    Ok(())
}
