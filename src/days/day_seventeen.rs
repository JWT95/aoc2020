use crate::common::read_input;
use anyhow::Result;
use std::collections::HashMap;

const INACTIVE: char = '.';
const ACTIVE: char = '#';

pub fn day_seventeen() -> Result<()> {
    let inputs: Vec<Vec<char>> = read_input("input/day_seventeen.txt")?
        .map(|x| x.chars().collect())
        .collect();

    let mut points = HashMap::new();

    for x in 0..inputs[0].len() {
        for y in 0..inputs.len() {
            points.insert((x as i32, y as i32, 0, 0), inputs[y][x]);
        }
    }

    for _i in 0..6 {
        points = take_turn(points);
    }

    let answer = points.values().filter(|x| **x == ACTIVE).count();

    println!("{:?}", answer);

    Ok(())
}

fn take_turn(input: HashMap<(i32, i32, i32, i32), char>) -> HashMap<(i32, i32, i32, i32), char> {
    // Create new hashmap
    let mut new_input = HashMap::new();

    // Work out the outer reaches of the map.
    let min_x: i32 = *input.keys().map(|(x, _y, _z, _w)| x).min().unwrap();
    let max_x: i32 = *input.keys().map(|(x, _y, _z, _w)| x).max().unwrap();
    let min_y: i32 = *input.keys().map(|(_x, y, _z, _w)| y).min().unwrap();
    let max_y: i32 = *input.keys().map(|(_x, y, _z, _w)| y).max().unwrap();
    let min_z: i32 = *input.keys().map(|(_x, _y, z, _w)| z).min().unwrap();
    let max_z: i32 = *input.keys().map(|(_x, _y, z, _w)| z).max().unwrap();
    let min_w: i32 = *input.keys().map(|(_x, _y, _z, w)| w).min().unwrap();
    let max_w: i32 = *input.keys().map(|(_x, _y, _z, w)| w).max().unwrap();

    for x in (min_x - 1)..(max_x + 2) {
        for y in (min_y - 1)..(max_y + 2) {
            for z in (min_z - 1)..(max_z + 2) {
                for w in (min_w - 1)..(max_w + 2) {
                    let new = assess(x, y, z, w, &input);
                    new_input.insert((x, y, z, w), new);
                }
            }
        }
    }

    new_input
}

fn assess(x: i32, y: i32, z: i32, w: i32, input: &HashMap<(i32, i32, i32, i32), char>) -> char {
    let neighbors_active = neighbors_active(x, y, z, w, &input);

    if input.get(&(x, y, z, w)).unwrap_or_else(|| &INACTIVE) == &ACTIVE {
        if neighbors_active == 2 || neighbors_active == 3 {
            ACTIVE
        } else {
            INACTIVE
        }
    } else {
        if neighbors_active == 3 {
            ACTIVE
        } else {
            INACTIVE
        }
    }
}

fn neighbors_active(
    x: i32,
    y: i32,
    z: i32,
    w: i32,
    input: &HashMap<(i32, i32, i32, i32), char>,
) -> u32 {
    let mut count = 0;
    for i in -1..2 {
        for j in -1..2 {
            for k in -1..2 {
                for ii in -1..2 {
                    if !(i == 0 && j == 0 && k == 0 && ii == 0) {
                        let val = input
                            .get(&(x + i, y + j, z + k, w + ii))
                            .unwrap_or_else(|| &INACTIVE);
                        if *val == ACTIVE {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}
