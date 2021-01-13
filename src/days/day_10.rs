use crate::common::read_input;
use anyhow::Result;

pub fn day_10() -> Result<()> {
    let mut inputs: Vec<u32> = read_input("input/day_10.txt")?
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    inputs.push(0);
    inputs.sort();
    inputs.push(inputs.last().unwrap() + 3);

    part_two(inputs);

    Ok(())
}

fn num_ways_to_end(index: usize, inputs: &[u32], ways_to_end_vec: &[u64]) -> u64 {
    if index == inputs.len() - 1 {
        1
    } else {
        let val = inputs[index];
        let mut ways_to_end = 0;
        for i in (index + 1)..(index + 4) {
            if i < inputs.len() && inputs[i] <= val + 3 {
                ways_to_end += ways_to_end_vec[i];
            }
        }
        ways_to_end
    }
}

fn part_two(inputs: Vec<u32>) {
    let mut ways_to_end = vec![0; inputs.len()];

    for i in (0..inputs.len()).rev() {
        ways_to_end[i] = num_ways_to_end(i, &inputs, &ways_to_end);
    }

    println!("{:?}", ways_to_end[0]);
}

fn _part_one(inputs: Vec<u32>) {
    let mut ones = 0;
    let mut threes = 0;
    for i in 0..(inputs.len() - 1) {
        if inputs[i + 1] - inputs[i] == 1 {
            println!("{}-{}", inputs[i + 1], inputs[i]);
            ones += 1;
        } else if inputs[i + 1] - inputs[i] == 3 {
            threes += 1;
        }
    }

    println!("{:?}", ones * threes);
}
