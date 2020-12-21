use crate::common::read_input;
use anyhow::Result;

const FLOOR: char = '.';
const OCCUPIED: char = '#';
const EMPTY: char = 'L';

pub fn day_eleven() -> Result<()> {
    let inputs: Vec<Vec<char>> = read_input("input/day_eleven.txt")?
        .map(|x| x.chars().collect())
        .collect();

    part_one(inputs);

    Ok(())
}

fn part_one(input: Vec<Vec<char>>) {
    let mut turn = take_turn(input);
    while turn.1 == true {
        turn = take_turn(turn.0)
    }

    let num_seats: usize = turn
        .0
        .iter()
        .map(|x| x.iter().filter(|y| **y == OCCUPIED).count())
        .sum();

    println!("{:?}", num_seats);
}

fn take_turn(input: Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
    let mut has_changed = false;
    // Create new vec
    let mut new_input: Vec<Vec<char>> = (0..(input.len()))
        .map(|_| vec![FLOOR; input[0].len()])
        .collect();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let new = assess_seat(x, y, &input);
            if new != input[y][x] {
                has_changed = true;
            }
            new_input[y][x] = new;
        }
    }

    (new_input, has_changed)
}

fn assess_seat(x: usize, y: usize, input: &Vec<Vec<char>>) -> char {
    match input[y][x] {
        FLOOR => FLOOR,
        _ => {
            let mut occ_count = 0;
            let dirs: [(i64, i64); 8] = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];
            // for dir in dirs.iter() {
            //     if x as i64 + dir.0 >= 0
            //         && x as i64 + dir.0 < input[0].len() as i64
            //         && y as i64 + dir.1 >= 0
            //         && y as i64 + dir.1 < input.len() as i64
            //     {
            //         if input[((y as i64) + dir.1) as usize][((x as i64) + dir.0) as usize]
            //             == OCCUPIED
            //         {
            //             occ_count += 1;
            //         }
            //     }
            // }

            for dir in dirs.iter() {
                let mut multiplier = 1;
                let mut check = (dir.0, dir.1);
                while x as i64 + check.0 >= 0
                    && x as i64 + check.0 < input[0].len() as i64
                    && y as i64 + check.1 >= 0
                    && y as i64 + check.1 < input.len() as i64
                {
                    match input[((y as i64) + check.1) as usize][((x as i64) + check.0) as usize] {
                        OCCUPIED => {
                            occ_count += 1;
                            break;
                        }
                        EMPTY => {
                            break;
                        }
                        _ => {}
                    }
                    multiplier += 1;
                    check = (dir.0 * multiplier, dir.1 * multiplier);
                }
            }

            match input[y][x] {
                EMPTY => {
                    if occ_count == 0 {
                        OCCUPIED
                    } else {
                        EMPTY
                    }
                }
                OCCUPIED => {
                    if occ_count >= 5 {
                        EMPTY
                    } else {
                        OCCUPIED
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
