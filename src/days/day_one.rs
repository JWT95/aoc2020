use crate::common::read_input;
use anyhow::Result;

fn part_one(inputs: Vec<i64>) {
    for x in &inputs {
        for y in &inputs {
            if x + y == 2020 {
                println!("x: {}, y: {}, multiplier: {}", x, y, x * y);
                return;
            }
        }
    }
}

fn part_two(inputs: Vec<i64>) {
    for x in &inputs {
        for y in &inputs {
            for z in &inputs {
                if x + y + z == 2020 {
                    println!(
                        "x: {}, y: {}, z: {},  multiplier: {}",
                        x,
                        y,
                        x * y,
                        x * y * z
                    );
                    return;
                }
            }
        }
    }
}

pub fn day_one() -> Result<()> {
    let inputs: Vec<i64> = read_input("input/day_one.txt")?
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    part_two(inputs);

    Ok(())
}