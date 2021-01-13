use crate::common::read_input;
use anyhow::Result;
use itertools::Itertools;

const TARGET: u64 = 21806024;

fn _is_valid(previous: &[u64], number: u64) -> bool {
    previous
        .iter()
        .combinations(2)
        .filter(|combo| combo[0] != combo[1])
        .map(|combo| combo[0] + combo[1])
        .any(|x| x == number)
}

fn _part_one(inputs: Vec<u64>) {
    for i in 25..inputs.len() {
        if !_is_valid(&inputs[(i - 25)..i], inputs[i]) {
            println!("{:?}", inputs[i]);
            return;
        }
    }
}

fn part_two(inputs: Vec<u64>) {
    let answer = (0..inputs.len())
        .map(|x| (x, sums_to_target(&inputs, x)))
        .find(|(_start_index, (result, _end_index))| *result == true)
        .map(|(start_index, (_result, end_index))| {
            inputs[start_index..(end_index + 1)].iter().min().unwrap()
                + inputs[start_index..(end_index + 1)].iter().max().unwrap()
        })
        .unwrap();

    println!("{:?}", answer);
}

fn sums_to_target(numbers: &[u64], index: usize) -> (bool, usize) {
    let mut end_index = index + 1;
    let mut sum = numbers[index];
    loop {
        sum += numbers[end_index];
        if sum == TARGET {
            return (true, end_index);
        } else if sum > TARGET {
            return (false, 0);
        } else {
            end_index += 1;
        }
    }
}

pub fn day_09() -> Result<()> {
    let inputs: Vec<u64> = read_input("input/day_09.txt")?
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    part_two(inputs);

    Ok(())
}
