use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone, Eq)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl PartialEq<Vec2> for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Vec2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn parse_input(file_name: &str) -> ((i32, i32), HashMap<Vec2, u8>) {
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::open(path) {
        Err(err) => panic!("Couldn't open {}: {}", display, err),
        Ok(file) => file,
    };

    let mut s = String::new();

    if file.read_to_string(&mut s).is_err() {
        panic!("Couldn't read {}", file_name);
    }

    let mut grid: HashMap<Vec2, u8> = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in (0_i32..).zip(s.split("\n")) {
        if line.is_empty() {
            continue;
        }

        for (x, char) in (0_i32..).zip(line.chars()) {
            grid.insert(Vec2 { x, y }, char.to_digit(10).unwrap() as u8);

            width = cmp::max(width, x);
        }

        height = cmp::max(height, y);
    }

    ((width + 1, height + 1), grid)
}

fn part1(width: i32, height: i32, grid: &HashMap<Vec2, u8>) -> u64 {
    let mut positions: Vec<HashMap<Vec2, HashSet<Vec2>>> = Vec::new();
    positions.resize(10, HashMap::new());

    for (pos, value) in grid {
        if *value == 9_u8 {
            positions
                .get_mut(9)
                .unwrap()
                .insert(pos.clone(), HashSet::from([pos.clone()]));
        }
    }

    let is_valid_position =
        |x: &Vec2| -> bool { x.x >= 0 && x.x < width && x.y >= 0 && x.y < height };

    for current_level in (1..=9).rev() {
        let clone = positions.clone();
        let current_position = clone.get(current_level).unwrap().clone();

        for (pos, score) in current_position {
            for y in -1..=1 {
                for x in -1..=1 {
                    if x == y || x == -y {
                        continue;
                    }

                    let new_pos = Vec2 {
                        x: pos.x + x,
                        y: pos.y + y,
                    };

                    if !is_valid_position(&new_pos) {
                        continue;
                    }

                    let value = grid.get(&new_pos).unwrap();
                    if *value == (current_level - 1) as u8 {
                        let next_level = positions.get_mut(current_level - 1).unwrap();

                        if next_level.get(&new_pos).is_some() {
                            let mut new_union = HashSet::new();
                            for u in next_level.get(&new_pos).unwrap().union(&score) {
                                new_union.insert(u.clone());
                            }

                            next_level.insert(new_pos.clone(), new_union);
                        } else {
                            next_level.insert(new_pos.clone(), score.clone());
                        }
                    }
                }
            }
        }
    }

    let mut sum: u64 = 0;
    for score in positions[0].values() {
        sum += score.len() as u64;
    }

    sum
}

fn part2(width: i32, height: i32, grid: &HashMap<Vec2, u8>) -> u64 {
    let mut positions: Vec<HashMap<Vec2, u64>> = Vec::new();
    positions.resize(10, HashMap::new());

    for (pos, value) in grid {
        if *value == 9_u8 {
            positions.get_mut(9).unwrap().insert(pos.clone(), 1);
        }
    }

    let is_valid_position =
        |x: &Vec2| -> bool { x.x >= 0 && x.x < width && x.y >= 0 && x.y < height };

    for current_level in (1..=9).rev() {
        let clone = positions.clone();
        let current_position = clone.get(current_level).unwrap().clone();

        for (pos, score) in current_position {
            for y in -1..=1 {
                for x in -1..=1 {
                    if x == y || x == -y {
                        continue;
                    }

                    let new_pos = Vec2 {
                        x: pos.x + x,
                        y: pos.y + y,
                    };

                    if !is_valid_position(&new_pos) {
                        continue;
                    }

                    let value = grid.get(&new_pos).unwrap();
                    if *value == (current_level - 1) as u8 {
                        let next_level = positions.get_mut(current_level - 1).unwrap();

                        if next_level.get(&new_pos).is_some() {
                            let previous_score = next_level.get(&new_pos).unwrap();

                            next_level.insert(new_pos.clone(), previous_score + score);
                        } else {
                            next_level.insert(new_pos.clone(), score);
                        }
                    }
                }
            }
        }
    }

    let mut sum: u64 = 0;
    for score in positions[0].values() {
        sum += *score;
    }

    sum
}

fn main() {
    // let ((width, height), grid) = parse_input("SmallExample.txt");
    // let ((width, height), grid) = parse_input("Example.txt");
    let ((width, height), grid) = parse_input("Input.txt");

    // println!("{:?}", parsed);
    let p1 = part1(width, height, &grid);
    let p2 = part2(width, height, &grid);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    // println!("Hello, world!");
}
