use std::cmp::Ordering;
use std::collections::BinaryHeap;
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

impl Ord for Vec2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

#[derive(Clone, Eq, PartialEq)]
struct Node {
    cost: usize,
    position: Vec2,
    previous: HashSet<Vec2>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(file_name: &str) -> Vec<Vec2> {
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

    let mut values: Vec<Vec2> = Vec::new();

    for line in s.split("\n") {
        if line.is_empty() {
            continue;
        }
        let value: Vec<i32> = line.split(",").map(|v| v.parse().unwrap()).collect();
        values.push(Vec2 {
            x: value[0],
            y: value[1],
        })
    }

    values
}

fn check_path(
    width: &i32,
    height: &i32,
    input: &Vec<Vec2>,
    bytes: i32,
    first_path: bool,
) -> HashSet<Vec2> {
    let mut grid: HashSet<Vec2> = HashSet::new();

    for i in 0..bytes {
        grid.insert(input[i as usize]);
    }

    let mut to_check: BinaryHeap<Node> = BinaryHeap::new();
    let mut been_checked: HashMap<Vec2, HashSet<Vec2>> = HashMap::new();

    let valid_position = |x: &Vec2| x.x >= 0 && x.x <= *width && x.y >= 0 && x.y <= *height;
    let cost = |x: &Vec2| (i32::pow(x.x - width, 2) + i32::pow(x.y - height, 2)) as usize;

    let mut final_path: HashSet<Vec2> = HashSet::new();
    let mut shortest_path = (width * height) as usize;
    let start_pos = Vec2 { x: 0, y: 0 };
    to_check.push(Node {
        cost: cost(&start_pos),
        position: start_pos,
        previous: HashSet::from([start_pos]),
    });

    while let Some(Node {
        cost: _,
        position,
        previous,
    }) = to_check.pop()
    {
        if let Some(x) = been_checked.get(&position) {
            if x.len() > previous.len() {
                been_checked.insert(position, previous.clone());
            } else {
                continue;
            }
        } else {
            been_checked.insert(position.clone(), previous.clone());
        }

        let mut new_path = previous;
        new_path.insert(position);

        if position.x == *width && position.y == *height {
            if new_path.len() < shortest_path {
                final_path = new_path.clone();
                shortest_path = final_path.len();

                if first_path {
                    break;
                }
            } else {
                break;
            }
        }

        let left = Vec2 {
            x: position.x - 1,
            y: position.y,
        };
        let right = Vec2 {
            x: position.x + 1,
            y: position.y,
        };
        let up = Vec2 {
            x: position.x,
            y: position.y - 1,
        };
        let down = Vec2 {
            x: position.x,
            y: position.y + 1,
        };

        if valid_position(&left) && !grid.contains(&left) {
            to_check.push(Node {
                cost: cost(&left),
                position: left,
                previous: new_path.clone(),
            });
        }
        if valid_position(&right) && !grid.contains(&right) {
            to_check.push(Node {
                cost: cost(&right),
                position: right,
                previous: new_path.clone(),
            });
        }
        if valid_position(&up) && !grid.contains(&up) {
            to_check.push(Node {
                cost: cost(&up),
                position: up,
                previous: new_path.clone(),
            });
        }
        if valid_position(&down) && !grid.contains(&down) {
            to_check.push(Node {
                cost: cost(&down),
                position: down,
                previous: new_path.clone(),
            });
        }
    }

    // for y in 0..=*height {
    //     for x in 0..=*width {
    //         let pos = Vec2 { x, y };
    //         if grid.contains(&pos) {
    //             print!("#");
    //         } else if final_path.contains(&pos) {
    //             print!("O");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    final_path
}

fn part1(width: &i32, height: &i32, input: &Vec<Vec2>, bytes: i32) -> u64 {
    (check_path(width, height, input, bytes, false).len() - 1) as u64
}

fn part2(width: &i32, height: &i32, input: &Vec<Vec2>, bytes: i32) -> Vec2 {
    for i in (bytes as usize)..input.len() {
        if check_path(width, height, input, (i + 1) as i32, true).is_empty() {
            return input[i];
        }
    }

    return Vec2 { x: -1, y: -1 };
}

fn main() {
    let use_example = false;
    let (width, height, parsed, bytes) = if use_example {
        (6, 6, parse_input("Example.txt"), 12)
    } else {
        (70, 70, parse_input("Input.txt"), 1024)
    };

    let p1 = part1(&width, &height, &parsed, bytes);
    println!("Part 1: {p1}");
    let p2 = part2(&width, &height, &parsed, bytes);
    println!("Part 2: {},{}", p2.x, p2.y);
}
