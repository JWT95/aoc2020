use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input(input: &str) -> Result<impl Iterator<Item = String>> {
    let f = File::open(input)?;
    let f = BufReader::new(f);

    Ok(f.lines().map(|line| line.unwrap()))
}
