use crate::common::read_input;
use anyhow::{Error, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

lazy_static::lazy_static! {
    pub static ref COLON: regex::Regex =
        Regex::new(r":").unwrap();
}

lazy_static::lazy_static! {
    pub static ref SINGLE: regex::Regex =
        Regex::new(r"^(\d+)\z").unwrap();
}

lazy_static::lazy_static! {
    pub static ref DOUBLE: regex::Regex =
        Regex::new(r"^(\d+) (\d+)\z").unwrap();
}

lazy_static::lazy_static! {
    pub static ref DOUBLE_OR: regex::Regex =
        Regex::new(r"^(\d+) (\d+) \| (\d+) (\d+)\z").unwrap();
}

lazy_static::lazy_static! {
    pub static ref SINGLE_OR: regex::Regex =
        Regex::new(r"^(\d+) \| (\d+)\z").unwrap();
}

type Rules = HashMap<String, String>;
type ParsedRules = HashMap<String, Rule>;
type ResolvedRules = HashMap<String, HashSet<String>>;

pub fn day_19() -> Result<()> {
    let (rules, input): (Vec<String>, Vec<String>) =
        read_input("input/day_19.txt")?.partition(|x| COLON.is_match(x));

    let input = input[1..].to_vec();

    let rules: Rules = rules
        .iter()
        .map(|x| COLON.split(x))
        .map(|mut iter| {
            (
                iter.nth(0).unwrap().trim().to_string(),
                iter.nth(0).unwrap().trim().to_string(),
            )
        })
        .collect();

    let resolved_rules = resolve_rules(rules);

    let left = &resolved_rules["42"];
    let right = &resolved_rules["31"];

    println!(
        "{:?}",
        input.iter().filter(|x| string_fits(x, left, right)).count(),
    );

    Ok(())
}

fn string_fits(a: &str, left: &HashSet<String>, right: &HashSet<String>) -> bool {
    if a.len() % 8 != 0 {
        return false;
    }

    for i in (a.len() / 16)..((a.len() / 8) - 1) {
        if string_fits_index(a, left, right, i) {
            return true;
        }
    }

    false
}

fn string_fits_index(
    a: &str,
    left: &HashSet<String>,
    right: &HashSet<String>,
    index: usize,
) -> bool {
    for i in 0..(a.len() / 8) {
        if i <= index {
            if !left.contains(&a[(8 * i)..(8 * (i + 1))]) {
                return false;
            }
        } else {
            if !right.contains(&a[(8 * i)..(8 * (i + 1))]) {
                return false;
            }
        }
    }

    true
}

#[derive(Debug)]
enum Rule {
    Single(String),
    Double {
        x: String,
        y: String,
    },
    SingleOr {
        x: String,
        y: String,
    },
    DoubleOr {
        left_x: String,
        left_y: String,
        right_x: String,
        right_y: String,
    },
}

impl Rule {
    fn constituents(&self) -> HashSet<String> {
        let mut hashset = HashSet::new();
        match self {
            Self::Single(x) => {
                hashset.insert(x.clone());
            }
            Self::Double { x, y } => {
                hashset.insert(x.clone());
                hashset.insert(y.clone());
            }
            Self::SingleOr { x, y } => {
                hashset.insert(x.clone());
                hashset.insert(y.clone());
            }
            Self::DoubleOr {
                left_x,
                left_y,
                right_x,
                right_y,
            } => {
                hashset.insert(left_x.clone());
                hashset.insert(left_y.clone());
                hashset.insert(right_x.clone());
                hashset.insert(right_y.clone());
            }
        }

        hashset
    }

    fn resolve(&self, resolved: &ResolvedRules) -> HashSet<String> {
        let mut hashset = HashSet::new();

        match self {
            Self::Single(x) => {
                hashset = resolved.get(x).unwrap().clone();
            }
            Self::Double { x, y } => {
                hashset = combine_hash_sets(&resolved.get(x).unwrap(), &resolved.get(y).unwrap())
            }
            Self::SingleOr { x, y } => {
                for left in resolved.get(x).unwrap() {
                    hashset.insert(left.clone());
                }

                for right in resolved.get(y).unwrap() {
                    hashset.insert(right.clone());
                }
            }
            Self::DoubleOr {
                left_x,
                left_y,
                right_x,
                right_y,
            } => {
                hashset =
                    combine_hash_sets(resolved.get(left_x).unwrap(), resolved.get(left_y).unwrap());

                hashset = hashset
                    .union(&combine_hash_sets(
                        resolved.get(right_x).unwrap(),
                        resolved.get(right_y).unwrap(),
                    ))
                    .map(|x| x.clone())
                    .collect();
            }
        }
        hashset
    }
}

fn combine_hash_sets(leftset: &HashSet<String>, rightset: &HashSet<String>) -> HashSet<String> {
    let mut hashset = HashSet::new();

    for left in leftset {
        for right in rightset {
            let mut left = left.clone();
            left.push_str(&right);
            hashset.insert(left);
        }
    }

    hashset
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = DOUBLE_OR.captures(s) {
            return Ok(Self::DoubleOr {
                left_x: captures[1].to_string(),
                left_y: captures[2].to_string(),
                right_x: captures[3].to_string(),
                right_y: captures[4].to_string(),
            });
        }

        if let Some(captures) = SINGLE_OR.captures(s) {
            return Ok(Self::SingleOr {
                x: captures[1].to_string(),
                y: captures[2].to_string(),
            });
        }

        if let Some(captures) = DOUBLE.captures(s) {
            return Ok(Self::Double {
                x: captures[1].to_string(),
                y: captures[2].to_string(),
            });
        }

        if let Some(captures) = SINGLE.captures(s) {
            return Ok(Self::Single(captures[1].to_string()));
        }

        anyhow::bail!("Couldn't parse")
    }
}

fn resolve_rules(mut rules: Rules) -> ResolvedRules {
    // Strategy - first resolve the singleton rules. Then resolve the keep going.
    let mut resolved_rules = ResolvedRules::new();

    for (k, v) in rules.iter() {
        if v == "\"a\"" {
            let mut hashset = HashSet::new();
            hashset.insert("a".to_string());
            resolved_rules.insert(k.clone(), hashset);
        }

        if v == "\"b\"" {
            let mut hashset = HashSet::new();
            hashset.insert("b".to_string());
            resolved_rules.insert(k.clone(), hashset);
        }
    }

    for key in resolved_rules.keys() {
        rules.remove(key);
    }

    let mut rules: ParsedRules = rules
        .iter()
        .map(|(k, v)| (k.clone(), Rule::from_str(v).unwrap()))
        .collect();

    // Remove rules 0, 8 and 11
    rules.remove("0");
    rules.remove("8");
    rules.remove("11");

    while !rules.is_empty() {
        for key in resolved_rules.keys() {
            rules.remove(key);
        }

        for (key, rule) in &rules {
            if rule
                .constituents()
                .iter()
                .all(|x| resolved_rules.contains_key(x))
            {
                let options = rule.resolve(&resolved_rules);
                resolved_rules.insert(key.clone(), options);
            }
        }
    }

    resolved_rules
}
