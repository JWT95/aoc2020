use anyhow::Result;

pub fn day_twenty_five() -> Result<()> {
    let card_pub = 8458505;
    let door_pub = 16050997;

    let mut loop_size = 0;
    let mut start = 1;
    for i in 0..100000000 {
        if start == door_pub {
            println!("door");
            break;
        }

        if start == card_pub {
            println!("card");
            break;
        }

        start *= 7;
        start = start % 20201227;
        loop_size += 1;
    }
    println!("{:?}", loop_size);

    let encryption_key = transform(card_pub, loop_size);

    println!("{:?}", encryption_key);

    Ok(())
}

fn transform(subject_number: usize, loop_size: usize) -> usize {
    let mut start = 1;

    for i in 0..loop_size {
        start *= subject_number;
        start = start % 20201227;
    }

    start
}
