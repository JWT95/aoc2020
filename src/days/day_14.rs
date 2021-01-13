use crate::common::read_input;
use anyhow::{Error, Result};
use regex::Regex;
use std::char;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

lazy_static::lazy_static! {
    pub static ref MASK: regex::Regex =
        Regex::new(r"^mask = (.+)\z").unwrap();
    pub static ref MEM: regex::Regex =
        Regex::new(r"^mem\[(\d+)\] = (\d+)\z").unwrap();
}

type Memory = HashMap<usize, usize>;

#[derive(Debug)]
enum Input {
    Mask(String),
    Mem { address: usize, value: usize },
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(caps) = MASK.captures(s) {
            return Ok(Self::Mask(caps.get(1).unwrap().as_str().into()));
        }

        if let Some(caps) = MEM.captures(s) {
            return Ok(Self::Mem {
                address: caps.get(1).unwrap().as_str().parse::<usize>()?,
                value: caps.get(2).unwrap().as_str().parse::<usize>()?,
            });
        }

        anyhow::bail!("Couldn't parse")
    }
}

struct Computer {
    memory: Memory,
    current_mask: String,
}

impl Computer {
    fn _apply_input_part_one(&mut self, input: Input) {
        match input {
            Input::Mask(mask) => {
                self.current_mask = mask;
            }
            Input::Mem { address, value } => {
                self.memory
                    .insert(address, _apply_mask_to_value(&self.current_mask, value));
            }
        }
    }

    fn apply_input_part_two(&mut self, input: Input) {
        match input {
            Input::Mask(mask) => {
                self.current_mask = mask;
            }
            Input::Mem { address, value } => {
                for address in apply_mask_to_address(&self.current_mask, address) {
                    println!("{:?}", address);
                    self.memory.insert(address, value);
                }
            }
        }
    }
}

fn _apply_mask_to_value(mask: &String, value: usize) -> usize {
    let chars = mask.chars().collect::<Vec<_>>();

    let value_as_binary = format!("{:b}", value).chars().collect::<Vec<_>>();

    let mut new_value = ['0'; 36];

    let start_index = chars.len() - value_as_binary.len();

    for i in start_index..chars.len() {
        new_value[i] = value_as_binary[i - start_index];
    }

    for i in 0..chars.len() {
        if chars[i] == '1' {
            new_value[i] = '1';
        } else if chars[i] == '0' {
            new_value[i] = '0';
        }
    }

    usize::from_str_radix(&new_value.iter().collect::<String>(), 2).unwrap()
}

fn apply_mask_to_address(mask: &String, address: usize) -> impl Iterator<Item = usize> {
    let chars = mask.chars().collect::<Vec<_>>();

    let value_as_binary = format!("{:b}", address).chars().collect::<Vec<_>>();

    let mut new_value = ['0'; 36];

    let start_index = chars.len() - value_as_binary.len();

    for i in start_index..chars.len() {
        new_value[i] = value_as_binary[i - start_index];
    }

    let mut floating_bits = HashSet::new();
    for i in 0..chars.len() {
        if chars[i] == '1' {
            println!("Change this index {:?}", i);
            new_value[i] = '1';
        } else if chars[i] == 'X' {
            println!("{:?}", i);
            floating_bits.insert(i);
        }
    }

    let two: u64 = 2;
    let floating_bits_len = floating_bits.len();
    (0..two.pow(floating_bits_len as u32))
        .map(move |num| {
            let num = format!("{:b}", num).chars().collect::<Vec<char>>();
            let start_index = floating_bits_len - num.len();
            (0..floating_bits_len).map(move |i| {
                if i >= start_index {
                    num[i - start_index]
                } else {
                    '0'
                }
            })
        })
        .map(move |v| {
            let mut new_value = new_value.clone();
            for (value, index) in v.into_iter().zip(floating_bits.clone()) {
                println!("{:?}", (value, index));
                new_value[index] = value;
            }
            usize::from_str_radix(&new_value.iter().collect::<String>(), 2).unwrap()
        })
}

pub fn day_14() -> Result<()> {
    let inputs: Vec<Input> = read_input("input/day_14.txt")?
        .map(|s| Input::from_str(&s).unwrap())
        .collect();

    let mut computer = Computer {
        memory: Memory::new(),
        current_mask: String::new(),
    };

    for input in inputs {
        computer.apply_input_part_two(input);
    }

    let a: usize = computer.memory.values().sum();

    println!("{:?}", a);

    Ok(())
}

#[test]
fn parse_mask() {
    let input = "mask = 01101X10101011000101X1X0XXX101111110";

    assert!(MASK.is_match(input));
}
