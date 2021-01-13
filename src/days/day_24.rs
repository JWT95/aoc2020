use crate::common::read_input;
use anyhow::Result;
use multiset::HashMultiSet;
use std::collections::HashSet;

pub fn day_24() -> Result<()> {
    let input: Vec<HashMultiSet<Direction>> = read_input("input/day_24.txt")?
        .map(|x| simplify(convert(x)))
        .collect();

    let mut black = HashSet::new();
    for tile in input.iter() {
        // Convert into vec
        let tile_vec = to_vec(tile);

        if black.contains(&tile_vec) {
            black.remove(&tile_vec);
        } else {
            black.insert(tile_vec);
        }
    }

    // Start with all tiles plus neighbors
    let mut tiles: HashSet<Vec<_>> = input.iter().map(|x| to_vec(x)).collect();

    for i in 0..100 {
        println!("{:?}", i);

        let mut new_blacks = HashSet::new();

        // Take the current tiles plus all their neighbors
        for tile in tiles.clone() {
            // Get the neighbors
            for neigbor in neighbors(to_multi_set(&tile)) {
                tiles.insert(to_vec(&neigbor));
            }
        }

        for tile in tiles.iter() {
            // Get the neighbors
            let neighbors: Vec<Vec<_>> = neighbors(to_multi_set(tile))
                .iter()
                .map(|x| to_vec(x))
                .collect();

            // Get the neighbor count
            let blacks = neighbors.iter().filter(|x| black.contains(*x)).count();

            if black.contains(tile) {
                if !(blacks == 0 || blacks > 2) {
                    new_blacks.insert(tile.clone());
                }
            } else {
                if blacks == 2 {
                    new_blacks.insert(tile.clone());
                }
            }
        }

        black = new_blacks;
    }

    println!("{:?}", black.len());

    Ok(())
}

fn convert(input: String) -> HashMultiSet<Direction> {
    let mut string_iter = input.chars();
    let mut multiset = HashMultiSet::new();

    while let Some(x) = string_iter.nth(0) {
        if x == 'e' {
            multiset.insert(Direction::East);
            continue;
        } else if x == 'w' {
            multiset.insert(Direction::West);
            continue;
        }

        // Else it's a two digit thing
        let y = string_iter.nth(0).unwrap();
        if x == 'n' && y == 'w' {
            multiset.insert(Direction::Northwest);
        } else if x == 'n' && y == 'e' {
            multiset.insert(Direction::Northeast);
        } else if x == 's' && y == 'e' {
            multiset.insert(Direction::Southeast);
        } else if x == 's' && y == 'w' {
            multiset.insert(Direction::Southwest);
        } else {
            unreachable!();
        }
    }

    multiset
}

fn neighbors(input: HashMultiSet<Direction>) -> Vec<HashMultiSet<Direction>> {
    let mut neighbors = vec![];
    for direction in [
        Direction::East,
        Direction::West,
        Direction::Northwest,
        Direction::Northeast,
        Direction::Southwest,
        Direction::Southeast,
    ]
    .iter()
    {
        let mut neighbor = input.clone();
        neighbor.insert(direction.clone());
        neighbor = simplify(neighbor);
        neighbors.push(neighbor);
    }

    neighbors
}

fn to_multi_set(directions: &Vec<Direction>) -> HashMultiSet<Direction> {
    directions.into_iter().cloned().collect()
}

fn to_vec(directions: &HashMultiSet<Direction>) -> Vec<Direction> {
    let mut vec: Vec<_> = directions.iter().cloned().collect();

    vec.sort();

    vec
}

fn simplify(mut directions: HashMultiSet<Direction>) -> HashMultiSet<Direction> {
    // Now cancel out simplifications
    while contains_simplification(&directions) || contains_opposite(&directions) {
        if directions.contains(&Direction::West) && directions.contains(&Direction::West.opposite())
        {
            directions.remove(&Direction::West);
            directions.remove(&Direction::West.opposite());
        }
        if directions.contains(&Direction::Northwest)
            && directions.contains(&Direction::Northwest.opposite())
        {
            directions.remove(&Direction::Northwest);
            directions.remove(&Direction::Northwest.opposite());
        }
        if directions.contains(&Direction::Northeast)
            && directions.contains(&Direction::Northeast.opposite())
        {
            directions.remove(&Direction::Northeast);
            directions.remove(&Direction::Northeast.opposite());
        }

        if directions.contains(&Direction::West) && directions.contains(&Direction::Northeast) {
            directions.remove(&Direction::West);
            directions.remove(&Direction::Northeast);
            directions.insert(Direction::Northwest);
        }
        if directions.contains(&Direction::Northwest) && directions.contains(&Direction::East) {
            directions.remove(&Direction::Northwest);
            directions.remove(&Direction::East);
            directions.insert(Direction::Northeast);
        }
        if directions.contains(&Direction::Southwest) && directions.contains(&Direction::East) {
            directions.remove(&Direction::Southwest);
            directions.remove(&Direction::East);
            directions.insert(Direction::Southeast);
        }
        if directions.contains(&Direction::Southeast) && directions.contains(&Direction::West) {
            directions.remove(&Direction::Southeast);
            directions.remove(&Direction::West);
            directions.insert(Direction::Southwest);
        }
        if directions.contains(&Direction::Southeast) && directions.contains(&Direction::Northeast)
        {
            directions.remove(&Direction::Southeast);
            directions.remove(&Direction::Northeast);
            directions.insert(Direction::East);
        }
        if directions.contains(&Direction::Southwest) && directions.contains(&Direction::Northwest)
        {
            directions.remove(&Direction::Southwest);
            directions.remove(&Direction::Northwest);
            directions.insert(Direction::West);
        }
    }

    directions
}

fn contains_opposite(directions: &HashMultiSet<Direction>) -> bool {
    if directions.contains(&Direction::West) && directions.contains(&Direction::West.opposite()) {
        return true;
    }

    if directions.contains(&Direction::Northwest)
        && directions.contains(&Direction::Northwest.opposite())
    {
        return true;
    }

    if directions.contains(&Direction::Northeast)
        && directions.contains(&Direction::Northeast.opposite())
    {
        return true;
    }

    false
}

fn contains_simplification(directions: &HashMultiSet<Direction>) -> bool {
    if directions.contains(&Direction::West) && directions.contains(&Direction::Northeast) {
        return true;
    }

    if directions.contains(&Direction::Northwest) && directions.contains(&Direction::East) {
        return true;
    }

    if directions.contains(&Direction::Southwest) && directions.contains(&Direction::East) {
        return true;
    }

    if directions.contains(&Direction::Southeast) && directions.contains(&Direction::West) {
        return true;
    }

    if directions.contains(&Direction::Southeast) && directions.contains(&Direction::Northeast) {
        return true;
    }

    if directions.contains(&Direction::Southwest) && directions.contains(&Direction::Northwest) {
        return true;
    }

    false
}

#[derive(Eq, Hash, Debug, PartialEq, Clone, PartialOrd, Ord)]
enum Direction {
    Northwest,
    Northeast,
    Southeast,
    Southwest,
    West,
    East,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Self::Northwest => Self::Southeast,
            Self::Northeast => Self::Southwest,
            Self::Southeast => Self::Northwest,
            Self::Southwest => Self::Northeast,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}
