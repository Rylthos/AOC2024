use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn parse_input(file_name: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
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

    let mut keys: Vec<Vec<i32>> = Vec::new();
    let mut locks: Vec<Vec<i32>> = Vec::new();

    let mut is_key = false;
    let mut started = false;
    let mut current: Vec<i32> = Vec::new();
    current.resize(5, 0);

    for line in s.split("\n") {
        if line.is_empty() {
            if is_key {
                current = current.iter().map(|v| v - 1).collect();

                keys.push(current.clone());
            } else {
                locks.push(current.clone());
            }
            started = false;
            current = current.iter().map(|_| 0).collect();
            // current.resize(5, 0);
            continue;
        }

        if !started {
            if line.chars().nth(0).unwrap() == '.' {
                is_key = true;
            } else {
                is_key = false;
            }

            started = true;
        } else {
            for (i, c) in (0..).zip(line.chars()) {
                if c == '#' {
                    current[i] = current.get(i).unwrap() + 1;
                }
            }
        }
    }

    println!("Keys: {keys:?}");
    println!("Locks: {locks:?}");

    (keys, locks)
}

fn part1(keys: &Vec<Vec<i32>>, locks: &Vec<Vec<i32>>) -> u64 {
    let mut sum: u64 = 0;
    for key in keys {
        for lock in locks {
            let mut valid = true;
            for i in 0..5 {
                if key[i] + lock[i] > 5 {
                    valid = false;
                    break;
                }
            }

            if valid {
                sum += 1;
            }
        }
    }

    sum
}

fn main() {
    // let (keys, locks) = parse_input("Example.txt");
    let (keys, locks) = parse_input("Input.txt");

    let p1 = part1(&keys, &locks);
    println!("Part 1: {p1}");
}
