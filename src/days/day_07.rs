use crate::common::read_input;
use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};

const SHINY_GOLD: &'static str = "shiny gold";

lazy_static::lazy_static! {
    pub static ref BAG: regex::Regex =
        Regex::new(r"^(.+) bags contain").unwrap();

    pub static ref SUBBAG: Regex =  Regex::new(r"(\d) ([^,]+) bag").unwrap();
}

type Bags = HashMap<String, Subbags>;
type Bag = (String, Subbags);
type Subbags = HashMap<String, u32>;

pub fn day_07() -> Result<()> {
    let lines: Vec<String> = read_input("input/day_07.txt")?.collect();

    let bags: Bags = lines
        .iter()
        .map(|line| parse_bag_from_string(line))
        .collect();

    part_two(&bags);

    Ok(())
}

fn _part_one(bags: &Bags) {
    let gold_ancestors = _get_ancestors_for_bag(SHINY_GOLD, bags);
    println!("{:?}", gold_ancestors.len());
}

fn part_two(bags: &Bags) {
    let gold_subbags = bags.get(SHINY_GOLD).unwrap();
    let gold_children = get_contained_bags(gold_subbags, bags) - 1;
    println!("{:?}", gold_children);
}

fn parse_bag_from_string(bag_string: &str) -> Bag {
    // First get the bag
    let bag = BAG.captures(bag_string).unwrap().get(1).unwrap().as_str();

    // Now get the bit after contains
    let right_part = bag_string.split("contain").nth(1).unwrap();

    // Now process the bit after contains recursively. - use captures_iter
    (bag.into(), parse_subbags_from_string(right_part))
}

fn parse_subbags_from_string(subbag_string: &str) -> Subbags {
    // captures_iter
    SUBBAG
        .captures_iter(subbag_string)
        .map(|capture| {
            (
                String::from(capture.get(2).unwrap().as_str()),
                capture.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            )
        })
        .collect()
}

fn _get_parents_for_bag(bag: &str, bags: &Bags) -> HashSet<String> {
    bags.iter()
        .filter(|(_name, subbags)| subbags.contains_key(bag))
        .map(|(name, _subbags)| name.clone())
        .collect()
}

fn _get_ancestors_for_bag(bag: &str, bags: &Bags) -> HashSet<String> {
    let mut parents = _get_parents_for_bag(bag, bags);
    let mut ancestors = HashSet::new();
    while !parents.is_empty() {
        ancestors = ancestors.union(&parents).cloned().collect();
        let tmp_parents = parents.clone();
        parents = HashSet::new();
        for parent in tmp_parents {
            // Get parents of parent
            parents = parents
                .union(&_get_parents_for_bag(&parent, bags))
                .cloned()
                .collect();
        }
    }

    ancestors
}

fn get_contained_bags(bag: &Subbags, bags: &Bags) -> u32 {
    if bag.is_empty() {
        1
    } else {
        let contained_bags: u32 = bag
            .iter()
            .map(|(name, num)| num * get_contained_bags(bags.get(name).unwrap(), bags))
            .sum();
        contained_bags + 1
    }
}

#[test]
fn test_parse_bag_from_string() {
    let bag = "clear crimson bags contain 3 pale aqua bags, 4 plaid magenta bags, 3 dotted beige bags, 3 dotted black bags.";
    let parsed_bag = parse_bag_from_string(bag);
    let expected: (String, Subbags) = (
        String::from("clear crimson"),
        [
            (String::from("pale aqua"), 3),
            (String::from("dotted black"), 3),
            (String::from("dotted beige"), 3),
            (String::from("plaid magenta"), 4),
        ]
        .iter()
        .cloned()
        .collect(),
    );
    println!("{:?}", parsed_bag);
    assert_eq!(parsed_bag, expected);
}

#[test]
fn test_parse_empty_bag_from_string() {
    let bag = "shiny plum bags contain no other bags.";
    let parsed_bag = parse_bag_from_string(bag);
    println!("{:?}", parsed_bag);
    assert_eq!(parsed_bag, (String::from("shiny plum"), HashMap::new()));
}
