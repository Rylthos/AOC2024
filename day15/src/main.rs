use std::collections::HashMap;
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

fn parse_input(file_name: &str, double_width: bool) -> (Vec2, HashMap<Vec2, char>, Vec<char>) {
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
    let mut movement: Vec<char> = Vec::new();
    let mut start_pos = Vec2 { x: 0, y: 0 };

    let mut parsing_grid = true;

    for (y, line) in (0_i32..).zip(s.split("\n")) {
        if line.is_empty() {
            parsing_grid = false;
            continue;
        }

        if parsing_grid {
            for (x, char) in (0_i32..).zip(line.chars()) {
                if !double_width {
                    if char == '@' {
                        start_pos = Vec2 { x, y };
                        grid.insert(Vec2 { x, y }, '.');
                    } else {
                        grid.insert(Vec2 { x, y }, char);
                    }
                } else {
                    let new_x = x * 2;
                    if char == '@' {
                        start_pos = Vec2 { x: new_x, y };
                        grid.insert(Vec2 { x: new_x, y }, '.');
                        grid.insert(Vec2 { x: new_x + 1, y }, '.');
                    } else {
                        let (left, right) = match char {
                            '.' => ('.', '.'),
                            '#' => ('#', '#'),
                            'O' => ('[', ']'),
                            _ => panic!("Unknown"),
                        };
                        grid.insert(Vec2 { x: new_x, y }, left);
                        grid.insert(Vec2 { x: new_x + 1, y }, right);
                    }
                }
            }
        } else {
            for c in line.chars() {
                movement.push(c);
            }
        }
    }

    (start_pos, grid, movement)
}

fn move_boxes(check_pos: &Vec2, dir: &Vec2, grid: &mut HashMap<Vec2, char>) -> bool {
    let movement_pos = Vec2 {
        x: check_pos.x + dir.x,
        y: check_pos.y + dir.y,
    };

    let contains_2 = grid.get(&movement_pos).unwrap();
    match contains_2 {
        '.' => {
            *grid.get_mut(&movement_pos).unwrap() = 'O';
            *grid.get_mut(&check_pos).unwrap() = '.';
            return true;
        }
        'O' => {
            if move_boxes(&movement_pos, dir, grid) {
                *grid.get_mut(&movement_pos).unwrap() = *grid.get(&check_pos).unwrap();
                *grid.get_mut(&check_pos).unwrap() = '.';
                return true;
            }
            return false;
        }
        _ => {
            return false;
        }
    }
}

fn part1(start_pos: Vec2, grid: &HashMap<Vec2, char>, moves: &Vec<char>) -> u64 {
    let mut current_pos = start_pos;
    let mut new_grid = grid.clone();

    for m in moves {
        let dir: Vec2 = match m {
            '^' => Vec2 { x: 0, y: -1 },
            'v' => Vec2 { x: 0, y: 1 },
            '<' => Vec2 { x: -1, y: 0 },
            '>' => Vec2 { x: 1, y: 0 },
            _ => panic!("Unexpected Character {}", m),
        };

        let mut new_pos = Vec2 {
            x: current_pos.x + dir.x,
            y: current_pos.y + dir.y,
        };

        let contains = new_grid.get(&new_pos).unwrap();
        match contains {
            '.' => {}
            '#' => new_pos = current_pos.clone(),
            'O' => {
                if move_boxes(&new_pos, &dir, &mut new_grid) {
                } else {
                    new_pos = current_pos.clone();
                }
            }
            _ => {}
        }

        current_pos = new_pos;
    }

    let mut sum: u64 = 0;
    for (pos, value) in new_grid {
        if value != 'O' {
            continue;
        }

        sum += (pos.y * 100 + pos.x) as u64;
    }

    sum
}

fn move_boxes_2(
    check_pos: &Vec2,
    dir: &Vec2,
    grid: &mut HashMap<Vec2, char>,
    can_move: bool,
) -> bool {
    let movement_pos = Vec2 {
        x: check_pos.x + dir.x,
        y: check_pos.y + dir.y,
    };

    let contains_2 = grid.get(&movement_pos).unwrap();
    match contains_2 {
        '.' => {
            if can_move {
                *grid.get_mut(&movement_pos).unwrap() = *grid.get(&check_pos).unwrap();
                *grid.get_mut(&check_pos).unwrap() = '.';
            }
            return true;
        }
        '[' | ']' => {
            let char = contains_2.clone();
            if dir.x == 0 {
                let (left_x, right_x) = match char {
                    '[' => (movement_pos.x, movement_pos.x + 1),
                    ']' => (movement_pos.x - 1, movement_pos.x),
                    _ => unreachable!(),
                };

                let left_pos = Vec2 {
                    x: left_x,
                    y: movement_pos.y,
                };

                let right_pos = Vec2 {
                    x: right_x,
                    y: movement_pos.y,
                };

                let left = move_boxes_2(&left_pos, &dir, grid, can_move);

                let right = move_boxes_2(&right_pos, &dir, grid, can_move);

                if left && right {
                    if can_move {
                        *grid.get_mut(&movement_pos).unwrap() = *grid.get(&check_pos).unwrap();

                        *grid.get_mut(&check_pos).unwrap() = '.';
                    }

                    return true;
                }
            } else {
                if move_boxes_2(&movement_pos, &dir, grid, can_move) {
                    if can_move {
                        *grid.get_mut(&movement_pos).unwrap() = *grid.get(&check_pos).unwrap();

                        *grid.get_mut(&check_pos).unwrap() = '.';
                    }

                    return true;
                }
            }
            return false;
        }
        _ => {
            return false;
        }
    }
}

fn part2(start_pos: Vec2, grid: &HashMap<Vec2, char>, moves: &Vec<char>) -> u64 {
    let mut current_pos = start_pos;
    let mut new_grid = grid.clone();

    for m in moves {
        let dir: Vec2 = match m {
            '^' => Vec2 { x: 0, y: -1 },
            'v' => Vec2 { x: 0, y: 1 },
            '<' => Vec2 { x: -1, y: 0 },
            '>' => Vec2 { x: 1, y: 0 },
            _ => panic!("Unexpected Character {}", m),
        };

        let mut new_pos = Vec2 {
            x: current_pos.x + dir.x,
            y: current_pos.y + dir.y,
        };

        let contains = new_grid.get(&new_pos).unwrap();
        match contains {
            '.' => {}
            '#' => new_pos = current_pos.clone(),
            '[' | ']' => {
                if move_boxes_2(&current_pos, &dir, &mut new_grid, false) {
                    move_boxes_2(&current_pos, &dir, &mut new_grid, true);
                } else {
                    new_pos = current_pos.clone();
                }
            }
            _ => {}
        }

        current_pos = new_pos;
    }

    let mut sum: u64 = 0;
    for (pos, value) in new_grid {
        if value != '[' {
            continue;
        }

        sum += (pos.y * 100 + pos.x) as u64;
    }

    sum
}

fn main() {
    {
        // let (start_pos, grid, moves) = parse_input("SmallExample.txt", false);
        // let (start_pos, grid, moves) = parse_input("Example.txt", false);
        let (start_pos, grid, moves) = parse_input("Input.txt", false);

        let p1 = part1(start_pos, &grid.clone(), &moves);
        println!("Part 1: {p1}");
    }

    {
        // let ((width, height), start_pos, grid, moves) = parse_input("BigBoxExample.txt", true);
        // let ((width, height), start_pos, grid, moves) = parse_input("Example.txt", true);
        let (start_pos, grid, moves) = parse_input("Input.txt", true);

        let p2 = part2(start_pos, &grid.clone(), &moves);
        println!("Part 2: {p2}");
    }
}
