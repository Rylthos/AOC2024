use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Eq, Debug, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
    dir: i32,
}

impl PartialEq<Vec2> for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        self.x == other.x && self.y == other.y && self.dir == other.dir
    }
}

impl Hash for Vec2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.dir.hash(state);
    }
}

#[derive(Clone)]
struct Grid {
    width: i32,
    height: i32,
    data: HashMap<Vec2, char>,
}

fn parse_input(file_name: &str) -> (Vec2, Grid) {
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

    let mut start: Vec2 = Vec2 {
        x: -1,
        y: -1,
        dir: 0,
    };
    let mut grid = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in (0_i32..).zip(s.split("\n")) {
        if line.is_empty() {
            continue;
        }
        for (x, char) in (0_i32..).zip(line.chars()) {
            let value = Vec2 { x, y, dir: 0 };

            grid.insert(value, char);

            if char == '^' {
                start = Vec2 { x, y, dir: 0 };
            }
            width = cmp::max(x, width);
        }
        height = cmp::max(y, height);
    }

    (
        start,
        Grid {
            width: width + 1,
            height: height + 1,
            data: grid,
        },
    )
}

fn get_direction(direction: i32) -> Vec2 {
    let mut dir = Vec2 { x: 0, y: 0, dir: 0 };
    dir.x = if direction == 1 {
        1
    } else if direction == 3 {
        -1
    } else {
        0
    };
    dir.y = if direction == 0 {
        -1
    } else if direction == 2 {
        1
    } else {
        0
    };

    dir
}

fn trace_path(start: &Vec2, grid: &Grid, trace_path: bool) -> (bool, HashSet<Vec2>) {
    let mut visited: HashSet<Vec2> = HashSet::new();

    let mut current_pos = start.clone();

    if !trace_path {
        visited.insert(start.clone());
    }

    loop {
        let offset = get_direction(current_pos.dir);

        let new_pos = Vec2 {
            x: current_pos.x + offset.x,
            y: current_pos.y + offset.y,
            dir: 0,
        };

        if new_pos.x < 0 || new_pos.x >= grid.width || new_pos.y < 0 || new_pos.y >= grid.height {
            break;
        }

        let next_char = grid.data.get(&new_pos).unwrap();
        let (write_value, next_pos) = match next_char {
            '.' | '^' => (
                true,
                Vec2 {
                    x: new_pos.x,
                    y: new_pos.y,
                    dir: current_pos.dir,
                },
            ),
            '#' => (
                false,
                Vec2 {
                    x: current_pos.x,
                    y: current_pos.y,
                    dir: (current_pos.dir + 1) % 4,
                },
            ),
            _ => panic!("Unexepected Character"),
        };

        if trace_path && write_value {
            let write_pos = Vec2 {
                x: current_pos.x,
                y: current_pos.y,
                dir: next_pos.dir,
            };
            // println!("{:?}", write_pos);
            if visited.contains(&write_pos) {
                // println!("Contained");
                return (false, visited);
            } else {
                visited.insert(write_pos);
            }
        } else if !trace_path {
            visited.insert(Vec2 {
                x: next_pos.x,
                y: next_pos.y,
                dir: 0,
            });
        }

        current_pos = next_pos;
    }

    (true, visited)
}

fn part1(start: &Vec2, grid: &Grid) -> u64 {
    let (_, path) = trace_path(start, grid, false);
    path.len() as u64
}

fn part2(start: &Vec2, grid: &Grid) -> u64 {
    let (_, path) = trace_path(start, grid, true);

    let mut mut_grid = grid.clone();

    let mut valid_nodes: HashSet<Vec2> = HashSet::new();

    for pos in path {
        let offset = get_direction(pos.dir);

        let new_pos = Vec2 {
            x: pos.x + offset.x,
            y: pos.y + offset.y,
            dir: 0,
        };

        if new_pos.x < 0 || new_pos.x >= grid.width || new_pos.y < 0 || new_pos.y >= grid.height {
            continue;
        }

        if *grid.data.get(&new_pos).unwrap() != '.' {
            continue;
        }

        mut_grid.data.insert(new_pos.clone(), '#');
        let (valid_path, _) = trace_path(start, &mut_grid, true);
        if !valid_path {
            valid_nodes.insert(new_pos.clone());
        }
        mut_grid.data.insert(new_pos.clone(), '.');
    }

    valid_nodes.len() as u64
}

fn main() {
    // let (start, grid) = parse_input("Example.txt");
    let (start, grid) = parse_input("Input.txt");
    let p1 = part1(&start, &grid);
    let p2 = part2(&start, &grid);

    println!("Part1: {}", p1);
    println!("Part2: {}", p2);
    // println!("Part1: {}", p1);
}
