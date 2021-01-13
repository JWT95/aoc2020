use crate::common::read_input;
use anyhow::{Error, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

lazy_static::lazy_static! {
    pub static ref FOOD: regex::Regex =
        Regex::new(r"^(.+) \(contains (.+)\)\z").unwrap();
}

pub fn day_twenty_one() -> Result<()> {
    let foods: Vec<Food> = read_input("input/day_twenty_one.txt")?
        .enumerate()
        .map(|(id, x)| Food {
            id,
            ..Food::from_str(&x).unwrap()
        })
        .collect();

    let mut allergens: HashMap<String, HashSet<String>> = foods
        .iter()
        .map(|x| x.allergens.clone())
        .flatten()
        .map(|x| (x, HashSet::new()))
        .collect();

    // For each food. If the food contains the ingredients intersect with the contenders.
    for food in foods.iter() {
        for allergen in food.allergens.iter() {
            if allergens[allergen].is_empty() {
                allergens.insert(allergen.clone(), food.ingredients.clone());
            } else {
                let intersection = food
                    .ingredients
                    .intersection(&allergens[allergen])
                    .cloned()
                    .collect();
                allergens.insert(allergen.clone(), intersection);
            }
        }
    }

    // Resolve
    while allergens.values().map(|x| x.len()).max().unwrap() > 1 {
        let ings_to_resolve: Vec<(String, String)> = allergens
            .iter()
            .filter(|(_x, v)| v.len() == 1)
            .map(|(x, v)| (x.clone(), v.iter().nth(0).unwrap().clone()))
            .collect();

        for (allergen, ing) in ings_to_resolve {
            for (other_allergen, ing_set) in allergens.iter_mut() {
                if &allergen != other_allergen {
                    ing_set.remove(&ing);
                }
            }
        }
    }

    // Ingredients that contain allergens
    let ing_all: HashSet<String> = allergens.values().flatten().cloned().collect();

    // Count all the ingredients
    let part_one_answer = foods
        .iter()
        .map(|x| x.ingredients.clone())
        .flatten()
        .filter(|x| !ing_all.contains(x))
        .count();

    println!("{:?}", part_one_answer);
    println!("{:?}", allergens);

    Ok(())
}

#[derive(Debug)]
struct Food {
    id: usize,
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl FromStr for Food {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let capture = FOOD.captures(s).unwrap();

        let ingredients = capture[1]
            .split(' ')
            .map(|x| x.trim().to_string())
            .collect();

        let allergens = capture[2]
            .split(',')
            .map(|x| x.trim().to_string())
            .collect();

        Ok(Food {
            id: 0,
            ingredients,
            allergens,
        })
    }
}
