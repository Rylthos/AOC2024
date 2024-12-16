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
    to_be_checked: &mut VecDeque<(Vec2, i32, u64, HashSet<Vec2>)>,
    searched: &mut HashMap<(Vec2, i32), u64>,
) -> (u64, HashSet<Vec2>) {
    let mut lowest_score: u64 = 1000000000;
    let mut path: HashSet<Vec2> = HashSet::new();

    while !to_be_checked.is_empty() {
        let (pos, dir, score, previous): (Vec2, i32, u64, HashSet<Vec2>) =
            to_be_checked.pop_front().unwrap();

        if pos == *target {
            if score != lowest_score {
                path = HashSet::new();
            }

            let mut extend = previous.clone();
            extend.insert(pos);

            path.extend(&extend);
            lowest_score = u64::min(lowest_score, score);

            continue;
        } else {
            if let Some(x) = searched.get(&(pos, dir)) {
                if *x < score {
                    continue;
                } else {
                    searched.insert((pos, dir), score);
                }
            } else {
                searched.insert((pos, dir), score);
            }
        }

        for i in -1..=1 {
            let mut new_dir = (dir + i) % 4;
            if new_dir < 0 {
                new_dir = 4 + new_dir;
            }
            let mut current_score = 1;

            if new_dir != dir {
                current_score += 1000;
            }

            let offset = dir_to_vec(new_dir);
            let new_pos = Vec2 {
                x: pos.x + offset.x,
                y: pos.y + offset.y,
            };

            if *grid.get(&new_pos).unwrap() == '#' {
                continue;
            }

            let mut new_node = previous.clone();
            new_node.insert(pos);

            to_be_checked.push_back((new_pos, new_dir, score + current_score, new_node));
        }
    }

    (lowest_score, path)
}

fn puzzle(start_pos: Vec2, end_pos: Vec2, grid: &HashMap<Vec2, char>) -> (u64, HashSet<Vec2>) {
    let mut searched: HashMap<(Vec2, i32), u64> = HashMap::new();
    let mut to_be_checked: VecDeque<(Vec2, i32, u64, HashSet<Vec2>)> = VecDeque::new();
    to_be_checked.push_back((start_pos, 1, 0, HashSet::<Vec2>::new()));
    bfs(&end_pos, grid, &mut to_be_checked, &mut searched)
}

fn main() {
    {
        let (start_pos, end_pos, grid) = parse_input("Input.txt");

        let (p1, path) = puzzle(start_pos, end_pos, &grid);
        println!("Part 1: {p1}");
        println!("Part 2: {}", path.len());
    }
}
