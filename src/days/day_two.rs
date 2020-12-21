use crate::common::read_input;
use anyhow::Result;
use regex::Regex;
use std::str::FromStr;

lazy_static::lazy_static! {
    pub static ref REGEX: regex::Regex =
        Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
}

#[derive(Debug)]
struct Password {
    range: std::ops::Range<usize>,
    character: char,
    password: String,
}

impl Password {
    fn is_valid_part_one(&self) -> bool {
        self.range.contains(
            &self
                .password
                .chars()
                .filter(|character| character == &self.character)
                .count(),
        )
    }

    fn is_valid_part_two(&self) -> bool {
        (self.password.chars().nth(self.range.start - 1).unwrap() == self.character)
            != (self.password.chars().nth(self.range.end - 2).unwrap() == self.character)
    }
}

pub fn day_two() -> Result<()> {
    let passwords = read_input("input/day_two.txt")?.map(|line| {
        let captures = REGEX.captures(&line).unwrap();
        Password {
            range: std::ops::Range {
                start: usize::from_str(captures.get(1).unwrap().as_str()).unwrap(),
                end: usize::from_str(captures.get(2).unwrap().as_str()).unwrap() + 1,
            },
            character: char::from_str(captures.get(3).unwrap().as_str()).unwrap(),
            password: captures.get(4).unwrap().as_str().to_string(),
        }
    });

    let valid = passwords
        .filter(|password| password.is_valid_part_two())
        .count();
    println!("{}", valid);
    Ok(())
}

#[test]
fn test_regex() {
    let input = "1-2 a: abcde";

    assert!(REGEX.is_match(input));
}

#[test]
fn test_captures() {
    let input = "1-2 a: abcde";
    let captures = REGEX.captures(input).unwrap();

    assert_eq!("1", captures.get(1).unwrap().as_str());
}
