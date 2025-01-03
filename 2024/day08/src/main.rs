use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use multimap::MultiMap;

use std::cmp;
// use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Eq, Debug, Clone)]
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

fn parse_input(file_name: &str) -> (Vec2, MultiMap<char, Vec2>) {
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::open(path) {
        Err(err) => panic!("Couldn't open {}: {}", display, err),
        Ok(file) => file,
    };

    let mut s = String::new();

    if !file.read_to_string(&mut s).is_ok() {
        panic!("Couldn't read {}", file_name);
    }

    let mut size: Vec2 = Vec2 { x: 0, y: 0 };
    let mut frequencies = MultiMap::new();
    // let mut rules = MultiMap::new();
    // let mut pages: Vec<Vec<u32>> = Vec::new();

    for (y, line) in (0_i32..).zip(s.split("\n")) {
        if line.is_empty() {
            continue;
        }
        for (x, char) in (0_i32..).zip(line.chars()) {
            if char != '.' {
                let pos: Vec2 = Vec2 { x, y };
                frequencies.insert(char, pos);
            }
            size.x = std::cmp::max(size.x, x + 1);
        }
        size.y = std::cmp::max(size.y, y + 1);
    }

    (size, frequencies)
}

fn puzzle(
    size: &Vec2,
    frequencies: &MultiMap<char, Vec2>,
    min_multiplier: i32,
    max_multiplier: i32,
) -> u64 {
    let mut antinodes: HashSet<Vec2> = HashSet::new();

    let valid_position =
        |v: &Vec2| -> bool { v.x >= 0 && v.x < size.x && v.y >= 0 && v.y < size.y };

    for freq in frequencies.keys() {
        let values = frequencies.get_vec(freq).unwrap();
        // println!("{}:", freq);
        for f1 in values.iter() {
            for f2 in values.iter() {
                if f1 == f2 {
                    continue;
                }

                for mul in min_multiplier..=max_multiplier {
                    let new_pos = Vec2 {
                        x: f1.x + mul * (f2.x - f1.x),
                        y: f1.y + mul * (f2.y - f1.y),
                    };

                    // println!("{:?} {:?} {:?}", f1, f2, new_pos);
                    // println!("{:?}", new_pos);

                    if !valid_position(&new_pos) {
                        continue;
                    }

                    antinodes.insert(new_pos);
                }
            }
        }
    }

    antinodes.len() as u64
}

fn main() {
    // let (size, frequencies) = parse_input("Example.txt");
    let (size, frequencies) = parse_input("Input.txt");

    let p1 = puzzle(&size, &frequencies, 2, 2);
    let p2 = puzzle(&size, &frequencies, 1, 100);
    println!("Part1: {}", p1);
    println!("Part2: {}", p2);
    // println!("{:?}", parsed);
    // println!("Hello, world!");
}
