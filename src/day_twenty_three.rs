use crate::common::read_input;
use anyhow::Result;
use std::collections::HashSet;

pub fn day_twenty_three() -> Result<()> {
    let input: Vec<u8> = vec![9, 5, 2, 3, 1, 6, 4, 8, 7];

    Ok(())
}

fn play_turn(input: Vec<u8>, current_pos: usize) -> (Vec<u8>, usize) {
    let current_cup = input[current_pos];

    let new_cups = vec![input[(current_pos + 1) % input.len(), input[(current_pos + 2) % input.len(), input[(current_pos + 2) % input.len()]
}
