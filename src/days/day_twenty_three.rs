use crate::common::read_input;
use anyhow::Result;
use std::collections::HashSet;

pub fn day_twenty_three() -> Result<()> {
    let mut input: Vec<u32> = vec![9, 5, 2, 3, 1, 6, 4, 8, 7];

    let mut extra_input: Vec<u32> = (10..1000001).collect();

    input.append(&mut extra_input);
    println!("{:?}", input.len());

    let mut current_pos = 0;
    for i in 0..10000000 {
        let turn = play_turn(input, current_pos);
        input = turn.0;
        current_pos = turn.1;

        if i % 10000 == 0 {
            println!("{:?}", i);
        }
    }

    let target_index = input
        .iter()
        .enumerate()
        .filter(|(i, x)| **x == 1)
        .nth(0)
        .unwrap()
        .0;

    println!("Cup one {:?}", input[(target_index + 1) % 1000000]);
    println!("Cup two {:?}", input[(target_index + 2) % 1000000]);

    Ok(())
}

fn play_turn(mut input: Vec<u32>, current_pos: usize) -> (Vec<u32>, usize) {
    let current_cup = input[current_pos];

    let new_cups = vec![
        input[(current_pos + 1) % input.len()],
        input[(current_pos + 2) % input.len()],
        input[(current_pos + 3) % input.len()],
    ];

    // Remove the new cups
    for cup in new_cups.iter() {
        let index = input
            .iter()
            .enumerate()
            .filter(|(i, x)| *x == cup)
            .nth(0)
            .unwrap();
        input.remove(index.0);
    }

    // Find the destination cup
    let mut target_cup = current_cup;

    loop {
        target_cup = target_cup - 1;
        if target_cup == 0 {
            target_cup = 1000000
        }

        if !new_cups.contains(&target_cup) {
            break;
        }
    }

    // Find the index of the target cup
    let target_index = input
        .iter()
        .enumerate()
        .filter(|(i, x)| **x == target_cup)
        .nth(0)
        .unwrap()
        .0;

    // Add the cups to that index
    for i in (0..3).rev() {
        input.insert(target_index + 1, new_cups[i]);
    }

    // Come up with new pos
    let index = input
        .iter()
        .enumerate()
        .filter(|(i, x)| **x == current_cup)
        .nth(0)
        .unwrap()
        .0;

    (input, (index + 1) % 1000000)
}
