use crate::common::read_input;
use anyhow::Result;
use regex::Regex;

lazy_static::lazy_static! {
    pub static ref BRACKETED: regex::Regex =
        Regex::new(r"(\([^\()]*\))").unwrap();
}

lazy_static::lazy_static! {
    pub static ref UNBRACKETED: regex::Regex =
        Regex::new(r"\((.+)\)").unwrap();
}

lazy_static::lazy_static! {
    pub static ref NUMBER: regex::Regex =
        Regex::new(r"\d+").unwrap();
}

lazy_static::lazy_static! {
    pub static ref PLUS: regex::Regex =
        Regex::new(r"(\d+) \+ (\d+)").unwrap();
}

pub fn day_18() -> Result<()> {
    let result: u64 = read_input("input/day_18.txt")?
        .map(unbracket)
        .map(|x| x.unwrap())
        .sum();

    println!("{:?}", result);

    Ok(())
}

// Process. Replace each bracketed segment until there are no segments left
fn unbracket(mut input: String) -> Result<u64> {
    while let Some(bmatch) = BRACKETED.find(&input.clone()) {
        // Resolve the brackets
        let resolved =
            resolve_part_two(&UNBRACKETED.captures(bmatch.as_str()).unwrap()[1].to_string())?;
        // Replace with the result of the brackets
        input.replace_range(bmatch.range(), &format!("{:?}", resolved));
    }

    resolve_part_two(&input)
}

fn _resolve_part_one(input: &str) -> Result<u64> {
    let inner = input.split(" ");

    let mut result = 0;
    let mut operation = "+";

    for thing in inner {
        if NUMBER.is_match(thing) {
            if operation == "+" {
                result += thing.parse::<u64>()?
            } else {
                result *= thing.parse::<u64>()?
            }
        } else {
            operation = thing;
        }
    }

    Ok(result)
}

fn unplus(input: &str) -> Result<String> {
    let mut input = input.to_string();

    while let Some(plus_match) = PLUS.find(&input.clone()) {
        // Resolve the plus
        let plus_cap = &PLUS.captures(&input).unwrap();
        let resolved = plus_cap[1].parse::<u64>()? + plus_cap[2].parse::<u64>()?;
        // Replace with the result of the brackets
        input.replace_range(plus_match.range(), &format!("{:?}", resolved));
    }

    Ok(input)
}

fn resolve_part_two(input: &str) -> Result<u64> {
    let input = unplus(input)?;

    Ok(input
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap_or_else(|_| 1))
        .fold(1, |acc, x| x * acc))
}
