use anyhow::Result;
use std::collections::HashMap;

pub fn day_fifteen() -> Result<()> {
    let starting_inputs = vec![14, 1, 17, 0, 3, 20];

    let mut numbers_seen = HashMap::new();

    let mut last_number = 0;
    for i in 0..6 {
        last_number = starting_inputs[i];
        numbers_seen.insert(last_number, i);
    }

    last_number = 0;

    for i in 6..(30000000 - 1) {
        let next_number = if let Some(turn) = numbers_seen.get(&last_number) {
            i - turn
        } else {
            0
        };

        numbers_seen.insert(last_number, i);
        last_number = next_number;
    }

    println!("{:?}", last_number);

    Ok(())
}
