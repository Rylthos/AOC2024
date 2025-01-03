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

fn parse_input(file_name: &str) -> ((i32, i32), HashMap<Vec2, char>) {
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
    let mut width = 0;
    let mut height = 0;

    for (y, line) in (0_i32..).zip(s.split("\n")) {
        if line.is_empty() {
            continue;
        }

        for (x, char) in (0_i32..).zip(line.chars()) {
            grid.insert(Vec2 { x, y }, char);

            width = cmp::max(width, x);
        }

        height = cmp::max(height, y);
    }

    ((width + 1, height + 1), grid)
}

fn parse_regions(width: i32, height: i32, grid: &HashMap<Vec2, char>) -> Vec<HashSet<Vec2>> {
    let mut all_checked: HashSet<Vec2> = HashSet::new();
    let mut regions: Vec<HashSet<Vec2>> = Vec::new();

    let valid_position = |p: &Vec2| -> bool { p.x >= 0 && p.x < width && p.y >= 0 && p.y < height };

    for (pos, value) in grid {
        if all_checked.contains(&pos) {
            continue;
        }

        let mut checked: HashSet<Vec2> = HashSet::new();

        let mut to_check: Vec<Vec2> = Vec::new();

        checked.insert(pos.clone());
        to_check.push(pos.clone());

        while !to_check.is_empty() {
            let pos = to_check.pop().unwrap();
            let left = Vec2 {
                x: pos.x - 1,
                y: pos.y,
            };
            let right = Vec2 {
                x: pos.x + 1,
                y: pos.y,
            };
            let up = Vec2 {
                x: pos.x,
                y: pos.y - 1,
            };
            let down = Vec2 {
                x: pos.x,
                y: pos.y + 1,
            };

            if valid_position(&left) {
                if *grid.get(&left).unwrap() == *value {
                    if !checked.contains(&left) {
                        checked.insert(left.clone());
                        to_check.push(left.clone());
                        all_checked.insert(left.clone());
                    }
                }
            }

            if valid_position(&right) {
                if *grid.get(&right).unwrap() == *value {
                    if !checked.contains(&right) {
                        checked.insert(right.clone());
                        to_check.push(right.clone());
                        all_checked.insert(right.clone());
                    }
                }
            }

            if valid_position(&up) {
                if *grid.get(&up).unwrap() == *value {
                    if !checked.contains(&up) {
                        checked.insert(up.clone());
                        to_check.push(up.clone());
                        all_checked.insert(up.clone());
                    }
                }
            }

            if valid_position(&down) {
                if *grid.get(&down).unwrap() == *value {
                    if !checked.contains(&down) {
                        checked.insert(down.clone());
                        to_check.push(down.clone());
                        all_checked.insert(down.clone());
                    }
                }
            }
        }

        regions.push(checked);
    }

    regions
}

fn part1(regions: &Vec<HashSet<Vec2>>) -> u64 {
    let mut sum: u64 = 0;
    for region in regions.iter() {
        let area = region.len();
        let mut perimeter: u64 = 0;
        for p in region {
            let left = Vec2 { x: p.x - 1, y: p.y };
            let right = Vec2 { x: p.x + 1, y: p.y };
            let up = Vec2 { x: p.x, y: p.y - 1 };
            let down = Vec2 { x: p.x, y: p.y + 1 };

            if !region.contains(&left) {
                perimeter += 1;
            }
            if !region.contains(&right) {
                perimeter += 1;
            }
            if !region.contains(&down) {
                perimeter += 1;
            }
            if !region.contains(&up) {
                perimeter += 1;
            }
        }

        sum += (area as u64) * perimeter;
    }

    sum
}

fn part2(regions: &Vec<HashSet<Vec2>>) -> u64 {
    let mut sum: u64 = 0;
    for region in regions.iter() {
        let area = region.len();
        let mut sides: u64 = 0;

        let mut directions: Vec<HashSet<Vec2>> = Vec::new();
        directions.resize(4, HashSet::new());

        // println!("Checking Region: {:?}", region);

        for p in region {
            let left = Vec2 { x: p.x - 1, y: p.y };
            let right = Vec2 { x: p.x + 1, y: p.y };
            let up = Vec2 { x: p.x, y: p.y - 1 };
            let down = Vec2 { x: p.x, y: p.y + 1 };

            if !region.contains(&left) {
                directions.get_mut(0).unwrap().insert(left.clone());
            }
            if !region.contains(&right) {
                directions.get_mut(2).unwrap().insert(right.clone());
            }
            if !region.contains(&down) {
                directions.get_mut(3).unwrap().insert(down.clone());
            }
            if !region.contains(&up) {
                directions.get_mut(1).unwrap().insert(up.clone());
            }
        }
        // println!("Directions: {:?}", directions);

        for dir in directions.iter() {
            if dir.is_empty() {
                continue;
            }
            // println!("\tDir: {:?}", dir);

            let mut all_ready_checked: HashSet<Vec2> = HashSet::new();
            let mut faces: Vec<HashSet<Vec2>> = Vec::new();
            for p in dir {
                let mut to_check: Vec<Vec2> = Vec::new();
                let mut face: HashSet<Vec2> = HashSet::new();

                if all_ready_checked.contains(p) {
                    continue;
                }

                face.insert(p.clone());
                to_check.push(p.clone());

                let mut add_element =
                    |pos: &Vec2,
                     all_ready_checked: &mut HashSet<Vec2>,
                     to_check: &mut Vec<Vec2>| {
                        if dir.contains(&pos) && !all_ready_checked.contains(&pos) {
                            all_ready_checked.insert(pos.clone());
                            face.insert(pos.clone());
                            to_check.push(pos.clone());
                        }
                    };

                while !to_check.is_empty() {
                    let p = to_check.pop().unwrap();

                    let left = Vec2 { x: p.x - 1, y: p.y };
                    let right = Vec2 { x: p.x + 1, y: p.y };
                    let up = Vec2 { x: p.x, y: p.y - 1 };
                    let down = Vec2 { x: p.x, y: p.y + 1 };

                    add_element(&left, &mut all_ready_checked, &mut to_check);
                    add_element(&right, &mut all_ready_checked, &mut to_check);
                    add_element(&up, &mut all_ready_checked, &mut to_check);
                    add_element(&down, &mut all_ready_checked, &mut to_check);
                }
                // println!("\t\tFace: {:?}", face);

                if !face.is_empty() {
                    faces.push(face);
                }
            }
            // println!("\tFaces: {:?}", faces);

            sides += faces.len() as u64;
        }
        // println!("Sides: {}", sides);

        sum += (area as u64) * sides;
    }

    sum
}

fn main() {
    // let ((width, height), grid) = parse_input("SmallExample.txt");
    // let ((width, height), grid) = parse_input("EShapeExample.txt");
    // let ((width, height), grid) = parse_input("EncompassExample.txt");
    // let ((width, height), grid) = parse_input("MultiRegionExample.txt");
    // let ((width, height), grid) = parse_input("Example.txt");
    let ((width, height), grid) = parse_input("Input.txt");
    let regions = parse_regions(width, height, &grid);

    let p1 = part1(&regions);
    println!("Part 1: {p1}");

    let p2 = part2(&regions);
    println!("Part 2: {p2}");
}
