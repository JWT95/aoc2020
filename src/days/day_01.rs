use crate::common::read_input;
use anyhow::Result;
use itertools::Itertools as _;

fn _part_one(inputs: &[u32]) {
    for pair in inputs.iter().combinations(2) {
        let (x, y) = (pair[0], pair[1]);
        if x + y == 2020 {
            println!("x: {}, y: {}, multiplier: {}", x, y, x * y);
            return;
        }
    }
}

fn part_two(inputs: &[u32]) {
    for triple in inputs.iter().combinations(3) {
        let (x, y, z) = (triple[0], triple[1], triple[2]);
        if x + y + z == 2020 {
            println!("x: {}, y: {}, z: {}, multiplier: {}", x, y, z, x * y * z);
            return;
        }
    }
}

pub fn day_01() -> Result<()> {
    let inputs: Vec<u32> = read_input("input/day_01.txt")?
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    part_two(&inputs);

    Ok(())
}
