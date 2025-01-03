use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone, Copy, Eq)]
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

fn parse_input(file_name: &str) -> (Vec2, Vec2, HashMap<Vec2, char>) {
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

    let mut grid: HashMap<Vec2, char> = HashMap::new();
    let mut start_pos = Vec2 { x: 0, y: 0 };
    let mut end_pos = Vec2 { x: 0, y: 0 };

    for (y, line) in (0_i32..).zip(s.split("\n")) {
        if line.is_empty() {
            continue;
        }
        for (x, char) in (0_i32..).zip(line.chars()) {
            if char == 'S' {
                start_pos = Vec2 { x, y };
                grid.insert(Vec2 { x, y }, '.');
            } else if char == 'E' {
                end_pos = Vec2 { x, y };
                grid.insert(Vec2 { x, y }, '.');
            } else {
                grid.insert(Vec2 { x, y }, char);
            }
        }
    }

    (start_pos, end_pos, grid)
}

fn dir_to_vec(dir: i32) -> Vec2 {
    match dir {
        0 => Vec2 { x: 0, y: -1 },
        1 => Vec2 { x: 1, y: 0 },
        2 => Vec2 { x: 0, y: 1 },
        3 => Vec2 { x: -1, y: 0 },
        _ => unreachable!(),
    }
}

fn bfs(
    target: &Vec2,
    grid: &HashMap<Vec2, char>,
    to_be_checked: &mut VecDeque<(Vec2, u64, HashSet<(Vec2, u64)>)>,
    searched: &mut HashMap<Vec2, u64>,
) -> HashSet<(Vec2, u64)> {
    while !to_be_checked.is_empty() {
        let (pos, score, previous): (Vec2, u64, HashSet<(Vec2, u64)>) =
            to_be_checked.pop_front().unwrap();

        if pos == *target {
            let mut extend = previous.clone();
            extend.insert((pos, score));

            return extend;
        } else {
            if let Some(x) = searched.get(&pos) {
                if *x < score {
                    continue;
                } else {
                    searched.insert(pos, score);
                }
            } else {
                searched.insert(pos, score);
            }
        }

        for i in 0..=3 {
            let offset = dir_to_vec(i);
            let new_pos = Vec2 {
                x: pos.x + offset.x,
                y: pos.y + offset.y,
            };

            if *grid.get(&new_pos).unwrap() == '#' {
                continue;
            }

            let mut new_node = previous.clone();
            new_node.insert((pos, score + 1));

            to_be_checked.push_back((new_pos, score + 1, new_node));
        }
    }

    HashSet::new()
}

fn part1(start_pos: Vec2, end_pos: Vec2, grid: &HashMap<Vec2, char>, diff: i32, save: i32) -> u64 {
    let mut searched: HashMap<Vec2, u64> = HashMap::new();
    let mut to_be_checked: VecDeque<(Vec2, u64, HashSet<(Vec2, u64)>)> = VecDeque::new();
    to_be_checked.push_back((start_pos, 0, HashSet::<(Vec2, u64)>::new()));
    let path = bfs(&end_pos, grid, &mut to_be_checked, &mut searched);
    println!("Finished First pass");

    let valid_pair = |(p1, p2, s1, s2): &(Vec2, Vec2, u64, u64)| {
        if s1 > s2 {
            return false;
        }
        if i32::abs((*s1 as i32) - (*s2 as i32)) <= save {
            return false;
        }

        let manhatten_distance = i32::abs(p1.x - p2.x) + i32::abs(p1.y - p2.y);

        if manhatten_distance > diff || manhatten_distance < 2 {
            return false;
        }

        if ((*s2 as i32) - manhatten_distance - ((*s1 - 1) as i32)) < save {
            return false;
        }

        // if i32::abs(((*s1 as i32) + manhatten_distance) - (*s2 as i32)) <= save {
        //     return false;
        // }

        return true;
    };

    let pairs: Vec<(Vec2, Vec2, u64, u64)> = path
        .iter()
        .map(|(p, s)| {
            path.iter()
                .map(move |(p2, s2)| (*p, p2.to_owned(), *s, *s2))
        })
        .flatten()
        .collect::<Vec<(Vec2, Vec2, u64, u64)>>()
        .iter()
        .filter(|p| valid_pair(p))
        .cloned()
        .collect();

    pairs.len() as u64
}

fn main() {
    // let (start_pos, end_pos, grid) = parse_input("Example.txt");
    let (start_pos, end_pos, grid) = parse_input("Input.txt");

    let p1 = part1(start_pos, end_pos, &grid, 2, 100);
    println!("Part 1: {p1}");

    let p2 = part1(start_pos, end_pos, &grid, 20, 100);
    println!("Part 2: {p2}");
}
