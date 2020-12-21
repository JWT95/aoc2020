use crate::common::read_input;
use anyhow::{Error, Result};
use ndarray::{arr1, arr2, Array2};
use regex::Regex;
use std::str::FromStr;

const NORTH: char = 'N';
const SOUTH: char = 'S';
const EAST: char = 'E';
const WEST: char = 'W';
const LEFT: char = 'L';
const RIGHT: char = 'R';
const FORWARD: char = 'F';
const NORTH_DIR: (i8, i8) = (0, 1);
const EAST_DIR: (i8, i8) = (1, 0);
const SOUTH_DIR: (i8, i8) = (0, -1);
const WEST_DIR: (i8, i8) = (-1, 0);

lazy_static::lazy_static! {
    pub static ref REGEX: regex::Regex =
        Regex::new(r"([A-Z])(\d+)").unwrap();
    pub static ref ANTI_ROTATIONS: Vec<Array2<i8>> = vec![arr2(&[[0, -1], [1, 0]]), arr2(&[[-1, 0], [0, -1]]), arr2(&[[0, 1], [-1, 0]])];
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    number: u16,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matches = REGEX.captures(s).unwrap();

        let direction = match matches.get(1).unwrap().as_str().chars().nth(0).unwrap() {
            NORTH => Direction::North,
            SOUTH => Direction::South,
            EAST => Direction::East,
            WEST => Direction::West,
            LEFT => Direction::Left,
            RIGHT => Direction::Right,
            FORWARD => Direction::Forward,
            _ => unimplemented!(),
        };

        let number = matches.get(2).unwrap().as_str().parse::<u16>()?;

        Ok(Instruction { direction, number })
    }
}

#[derive(Debug)]
struct Ship {
    direction: (i8, i8),
    position: (i32, i32),
}

impl Ship {
    fn new() -> Ship {
        Ship {
            direction: (1, 0),
            position: (0, 0),
        }
    }

    fn move_ship(&mut self, Instruction { direction, number }: Instruction, waypoint: &mut Ship) {
        match direction {
            Direction::North => {
                waypoint.position.0 += number as i32 * NORTH_DIR.0 as i32;
                waypoint.position.1 += number as i32 * NORTH_DIR.1 as i32;
            }
            Direction::East => {
                waypoint.position.0 += number as i32 * EAST_DIR.0 as i32;
                self.position.1 += number as i32 * EAST_DIR.1 as i32;
            }
            Direction::South => {
                waypoint.position.0 += number as i32 * SOUTH_DIR.0 as i32;
                waypoint.position.1 += number as i32 * SOUTH_DIR.1 as i32;
            }
            Direction::West => {
                waypoint.position.0 += number as i32 * WEST_DIR.0 as i32;
                waypoint.position.1 += number as i32 * WEST_DIR.1 as i32;
            }
            Direction::Left => {
                let input = arr1(&[
                    (waypoint.position.0 - self.position.0) as i8,
                    (waypoint.position.1 - self.position.1) as i8,
                ]);

                let output = ANTI_ROTATIONS[(number / 90 - 1) as usize]
                    .dot(&input)
                    .into_raw_vec();

                waypoint.position.0 = self.position.0 + output[0] as i32;
                waypoint.position.1 = self.position.1 + output[1] as i32;
            }
            Direction::Right => {
                let input = arr1(&[
                    (waypoint.position.0 - self.position.0) as i8,
                    (waypoint.position.1 - self.position.1) as i8,
                ]);

                let output = ANTI_ROTATIONS[(3 - number / 90) as usize]
                    .dot(&input)
                    .into_raw_vec();

                waypoint.position.0 = self.position.0 + output[0] as i32;
                waypoint.position.1 = self.position.1 + output[1] as i32;
            }
            Direction::Forward => {
                let diff = (
                    waypoint.position.0 - self.position.0,
                    waypoint.position.1 - self.position.1,
                );

                self.position.0 += number as i32 * diff.0 as i32;
                self.position.1 += number as i32 * diff.1 as i32;

                waypoint.position.0 = self.position.0 + diff.0;
                waypoint.position.1 = self.position.1 + diff.1;
            }
        }
    }
}

pub fn day_twelve() -> Result<()> {
    let instructions: Vec<Instruction> = read_input("input/day_twelve.txt")?
        .map(|x| Instruction::from_str(&x).unwrap())
        .collect();

    let mut ship = Ship::new();
    let mut waypoint = Ship {
        position: (10, 1),
        ..Ship::new()
    };

    for instruction in instructions {
        ship.move_ship(instruction, &mut waypoint);
    }

    println!("{:?}", ship.position.0.abs() + ship.position.1.abs());

    Ok(())
}
