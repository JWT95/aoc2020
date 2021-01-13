use crate::common::read_input;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

type AllTiles = HashMap<u32, HashSet<Tile>>;
type Matches = HashMap<u32, HashMap<Tile, HashSet<(Tile, Direction)>>>;

pub fn day_twenty() -> Result<()> {
    let mut tiles_iter = read_input("input/day_twenty.txt")?;

    let mut tiles = vec![];

    for _i in 0..144 {
        let mut tile = vec![];
        for _j in 0..11 {
            tile.push(tiles_iter.nth(0).unwrap());
        }

        let _ = tiles_iter.nth(0);
        tiles.push(tile);
    }

    let tiles: Vec<Tile> = tiles
        .into_iter()
        .map(|x| Tile {
            id: x[0].get(5..9).unwrap().parse::<u32>().unwrap(),
            tile: x[1..].iter().map(|y| y.chars().collect()).collect(),
        })
        .collect();

    let all_tiles: AllTiles = tiles
        .iter()
        .map(|tile| (tile.id, tile.all_variants()))
        .collect();

    let mut matches: Matches = HashMap::new();

    // For each tile, find which ones are compatible and in which direction.
    for (id, tiles) in all_tiles.iter() {
        let mut hashmap = HashMap::new();
        for tile in tiles {
            let mut hashset = HashSet::new();
            for (other_id, other_tiles) in all_tiles.iter() {
                if id != other_id {
                    for other_tile in other_tiles {
                        for direction in [
                            Direction::Up,
                            Direction::Down,
                            Direction::Left,
                            Direction::Right,
                        ]
                        .iter()
                        {
                            if tile.compatible_with(other_tile, direction.clone()) {
                                hashset.insert((other_tile.clone(), direction.clone()));
                            }
                        }
                    }
                }
            }
            hashmap.insert(tile.clone(), hashset);
        }
        matches.insert(*id, hashmap);
    }

    // Check if there are any tiles with max length of 2 for the matches
    let mut corners: Vec<u64> = vec![];
    for (id, result) in matches.iter() {
        if result.values().map(|v| v.len()).max().unwrap() == 2 {
            corners.push(*id as u64);
        }
    }

    println!("{:?}", corners);

    // Pick tile 3517 as the top left corner. Take an orientation with the matches of down and right.
    let top_left_matches = &matches[&3517];

    let (tile, _) = top_left_matches
        .iter()
        .filter(|(_tile, set)| {
            set.iter()
                .filter(|(_tile, direction)| {
                    *direction == Direction::Down || *direction == Direction::Right
                })
                .count()
                == 2
        })
        .nth(0)
        .unwrap();

    let mut tile_grid = vec![];

    tile_grid.push(vec![tile.clone()]);
    let mut current_tile = tile.clone();
    for y in 0..12 {
        for x in 0..12 {
            if x != 11 {
                // Get the current tile set and find the match to the right
                let current_tile_vec: Vec<Tile> = matches
                    .get(&current_tile.id)
                    .unwrap()
                    .get(&current_tile)
                    .unwrap()
                    .iter()
                    .filter(|(_tile, direction)| *direction == Direction::Right)
                    .map(|(tile, _direction)| tile)
                    .cloned()
                    .collect();

                assert_eq!(current_tile_vec.len(), 1);

                current_tile = current_tile_vec[0].clone();

                tile_grid[y].push(current_tile.clone());
            }

            if x == 11 && y < 11 {
                let row_left = &tile_grid[y][0];

                let current_tile_vec: Vec<Tile> = matches
                    .get(&row_left.id)
                    .unwrap()
                    .get(row_left)
                    .unwrap()
                    .iter()
                    .filter(|(_tile, direction)| *direction == Direction::Down)
                    .map(|(tile, _direction)| tile)
                    .cloned()
                    .collect();

                assert_eq!(current_tile_vec.len(), 1);

                current_tile = current_tile_vec[0].clone();

                tile_grid.push(vec![current_tile.clone()]);
            }
        }
    }

    let large_tile = Tile {
        id: 0,
        tile: build_large_grid(tile_grid),
    };

    // Total count
    let total_count: usize = large_tile
        .tile
        .iter()
        .map(|x| x.iter().filter(|x| **x == '#').count())
        .sum();

    // Doesn't always work. Sometimes fails to find any monsters. Perhaps
    // orientation (although I think I checked that).
    // Anyway, when it does find sea monsters it gets the right answer.
    // Also assumes the monsters don't overlap.
    println!("{:?}", total_count - large_tile.contains_monster() * 15);

    Ok(())
}

fn build_large_grid(tile_grid: Vec<Vec<Tile>>) -> Vec<Vec<char>> {
    // Build the large grid
    let mut grid: Vec<Vec<char>> = vec![];
    for _i in 0..96 {
        grid.push(vec!['.'; 96]);
    }

    for y in 0..12 {
        for x in 0..12 {
            let tile = tile_grid[y][x].clone();

            for iy in 1..9 {
                for ix in 1..9 {
                    grid[8 * y + iy - 1][8 * x + ix - 1] = tile.tile[iy][ix];
                }
            }
        }
    }

    grid
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Tile {
    id: u32,
    tile: Vec<Vec<char>>,
}

impl Tile {
    fn all_variants(&self) -> HashSet<Self> {
        let mut hashset = HashSet::new();

        hashset.insert(self.clone());
        hashset.insert(self.rotate_ninety());
        hashset.insert(self.rotate_ninety().rotate_ninety());
        hashset.insert(self.rotate_ninety().rotate_ninety().rotate_ninety());

        hashset.insert(self.transpose());
        hashset.insert(self.transpose().rotate_ninety());
        hashset.insert(self.transpose().rotate_ninety().rotate_ninety());
        hashset.insert(
            self.transpose()
                .rotate_ninety()
                .rotate_ninety()
                .rotate_ninety(),
        );

        hashset
    }

    fn flip_x(&self) -> Self {
        Tile {
            id: self.id,
            tile: self
                .tile
                .iter()
                .map(|x| x.iter().rev().map(|x| x.clone()).collect())
                .collect(),
        }
    }

    fn transpose(&self) -> Self {
        let mut tile = vec![];
        for i in 0..self.tile.len() {
            tile.push(self.tile.iter().map(|x| x[i].clone()).collect())
        }

        Tile { id: self.id, tile }
    }

    fn rotate_ninety(&self) -> Self {
        self.transpose().flip_x()
    }

    fn compatible_with(&self, other: &Tile, direction: Direction) -> bool {
        match direction {
            Direction::Up => self.tile[0] == other.tile[other.tile.len() - 1],
            Direction::Left => self
                .tile
                .iter()
                .map(|x| x[0])
                .eq(other.tile.iter().map(|x| x[other.tile.len() - 1])),
            Direction::Down => other.compatible_with(self, Direction::Up),
            Direction::Right => other.compatible_with(self, Direction::Left),
        }
    }

    fn contains_monster(&self) -> usize {
        let mut num_monsters = 0;

        for y in 0..self.tile.len() {
            for x in 0..self.tile.len() {
                if y < self.tile.len() - 3 {
                    if x < self.tile.len() - 20 {
                        if contains_monster(vec![
                            self.tile[y][x..(x + 20)].to_vec(),
                            self.tile[y + 1][x..(x + 20)].to_vec(),
                            self.tile[y + 2][x..(x + 20)].to_vec(),
                        ]) {
                            num_monsters += 1;
                        }
                    }
                }
            }
        }

        num_monsters
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn contains_monster(input: Vec<Vec<char>>) -> bool {
    if input[0][18] == '#'
        && input[1][0] == '#'
        && input[1][5] == '#'
        && input[1][6] == '#'
        && input[1][11] == '#'
        && input[1][12] == '#'
        && input[1][17] == '#'
        && input[1][18] == '#'
        && input[1][19] == '#'
        && input[2][1] == '#'
        && input[2][4] == '#'
        && input[2][7] == '#'
        && input[2][10] == '#'
        && input[2][13] == '#'
        && input[2][16] == '#'
    {
        return true;
    }

    false
}

#[test]
fn test_variants() {
    let tile = vec![vec!['a', 'b'], vec!['c', 'd']];

    let tile = Tile { id: 10, tile };

    let variants = tile.all_variants();

    assert_eq!(variants.len(), 8);
}
