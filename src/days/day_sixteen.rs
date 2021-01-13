use crate::common::read_input;
use anyhow::{Error, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::str::FromStr;

lazy_static::lazy_static! {
    pub static ref FIELD: regex::Regex =
        Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)\z").unwrap();
}

lazy_static::lazy_static! {
    pub static ref HEIGHT_CM: regex::Regex =
        Regex::new(r"^([0-9]+)cm\z").unwrap();
}

#[derive(Debug)]
struct Field {
    name: String,
    range_one: Range<u32>,
    range_two: Range<u32>,
}

impl FromStr for Field {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = FIELD.captures(s).unwrap();

        Ok(Field {
            name: captures[1].to_string(),
            range_one: Range {
                start: captures[2].parse::<u32>()?,
                end: captures[3].parse::<u32>()? + 1,
            },
            range_two: Range {
                start: captures[4].parse::<u32>()?,
                end: captures[5].parse::<u32>()? + 1,
            },
        })
    }
}

fn ticket_is_invalid(ticket: &[u32], fields: &[Field]) -> Vec<u32> {
    let mut invalid_fields = vec![];
    for val in ticket {
        let mut valid = false;
        for field in fields.iter() {
            if field.range_one.contains(val) || field.range_two.contains(val) {
                valid = true;
            }
        }
        if valid == false {
            invalid_fields.push(*val);
        }
    }

    invalid_fields
}

fn _part_one(fields: &[Field], nearby: &Vec<Vec<u32>>) {
    let invalid: u32 = nearby
        .iter()
        .map(|x| ticket_is_invalid(&x, fields))
        .flatten()
        .sum();

    println!("{:?}", invalid);
}

fn part_two(fields: &[Field], nearby: &Vec<Vec<u32>>, yours: &[u32]) {
    let valid = nearby
        .iter()
        .filter(|x| ticket_is_invalid(&x, fields).is_empty());

    let num_fields = yours.len();

    let mut possible_fields: HashMap<u32, HashSet<u32>> = HashMap::new();

    // Get all the fields in there.
    for i in 0..num_fields {
        let mut fields = HashSet::new();
        for j in 0..num_fields {
            fields.insert(j as u32);
        }
        possible_fields.insert(i as u32, fields);
    }

    // Now go through each valid ticket in turn. For each val, eliminate it from the relevant ranges.
    for ticket in valid {
        for i in 0..num_fields {
            for j in 0..num_fields {
                if !fields[j].range_one.contains(&ticket[i])
                    && !fields[j].range_two.contains(&ticket[i])
                {
                    possible_fields
                        .get_mut(&(j as u32))
                        .unwrap()
                        .remove(&(i as u32));
                }
            }
        }
    }

    // Now reduce
    let mut not_done = { possible_fields.values().find(|v| v.len() > 1).is_some() };

    while not_done {
        let singletons: Vec<(u32, u32)> = possible_fields
            .iter()
            .filter(|(_k, v)| v.len() == 1)
            .map(|(k, v)| (k.clone(), v.iter().nth(0).unwrap().clone()))
            .collect();

        for singleton in singletons {
            for (_k, v) in possible_fields
                .iter_mut()
                .filter(|(k, _v)| singleton.0 != **k)
            {
                v.remove(&singleton.1);
            }
        }

        not_done = possible_fields.values().find(|v| v.len() > 1).is_some();
    }

    // Get the first six values from the ticket
    let answer = (0..6)
        .map(|x| possible_fields[&x].iter().nth(0).unwrap())
        .map(|x| yours[*x as usize])
        .fold(1 as u64, |acc, x| acc * (x as u64));

    println!("{:?}", answer);
}

pub fn day_sixteen() -> Result<()> {
    let lines: Vec<_> = read_input("input/day_sixteen.txt")?.collect();

    // Find the blank lines
    let blanks: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, b)| **b == String::new())
        .map(|(a, _)| a)
        .collect();

    // Split the vec up by the blanks
    let (lines, nearby) = lines.split_at(blanks[1]);
    let (fields, yours) = lines.split_at(blanks[0]);

    let fields: Vec<_> = fields.iter().map(|x| Field::from_str(x).unwrap()).collect();

    let nearby = nearby[2..]
        .iter()
        .map(|x| {
            x.split(',')
                .map(|y| y.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    let yours = yours[2]
        .split(',')
        .map(|y| y.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    part_two(&fields, &nearby, &yours);

    Ok(())
}
