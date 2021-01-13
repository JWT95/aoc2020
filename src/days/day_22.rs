use crate::common::read_input;
use anyhow::Result;
use std::collections::HashSet;

pub fn day_22() -> Result<()> {
    let input: Vec<String> = read_input("input/day_22.txt")?.collect();

    let player_one_cards: Vec<usize> = input[1..26]
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let player_two_cards: Vec<usize> = input[28..53]
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    println!("{:?}", player_one_cards);
    println!("{:?}", player_two_cards);

    let (left, _right) = play_recursive_game(player_one_cards, player_two_cards);

    let answer: usize = left
        .iter()
        .rev()
        .enumerate()
        .map(|(x, y)| (x + 1) * *y)
        .sum();

    println!("{:?}", answer);

    Ok(())
}

fn _play_game(mut left: Vec<usize>, mut right: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    while !left.is_empty() && !right.is_empty() {
        let left_card = left.remove(0);
        let right_card = right.remove(0);

        if left_card > right_card {
            left.push(left_card);
            left.push(right_card);
        } else {
            right.push(right_card);
            right.push(left_card);
        }
    }

    (left, right)
}

fn play_recursive_game(mut left: Vec<usize>, mut right: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let mut previous_games = HashSet::new();

    while !left.is_empty() && !right.is_empty() {
        if previous_games.contains(&(left.clone(), right.clone())) {
            return (left, right);
        } else {
            previous_games.insert((left.clone(), right.clone()));
        }

        let left_card = left.remove(0);
        let right_card = right.remove(0);

        if left_card > left.len() || right_card > right.len() {
            if left_card > right_card {
                left.push(left_card);
                left.push(right_card);
            } else {
                right.push(right_card);
                right.push(left_card);
            }
        } else {
            let (sub_left, sub_right) =
                (left[0..left_card].to_vec(), right[0..right_card].to_vec());

            let (sub_left, _sub_right) = play_recursive_game(sub_left, sub_right);
            if sub_left.is_empty() {
                right.push(right_card);
                right.push(left_card);
            } else {
                left.push(left_card);
                left.push(right_card);
            }
        }
    }

    (left, right)
}
