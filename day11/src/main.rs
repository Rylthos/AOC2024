use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::collections::HashMap;

fn parse_input(file_name: &str) -> Vec<u64> {
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

    let mut values: Vec<u64> = Vec::new();

    for line in s.split("\n") {
        if line.is_empty() {
            continue;
        }

        let mut line_values: Vec<u64> = line
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        values.append(&mut line_values);
    }

    values
}

fn digit_count(value: u64) -> u64 {
    let mut copy = value;
    let mut digits = 0;
    while copy > 0 {
        digits += 1;
        copy /= 10;
    }

    digits
}

fn split_digit(value: u64) -> (u64, u64) {
    let count = digit_count(value);
    assert!(count % 2 == 0);

    let mask = u64::pow(10, (count / 2) as u32);

    let left = value % mask;
    let right = value / mask;

    (left, right)
}

fn calculate_value(
    value: u64,
    blinks: u8,
    memory: &mut HashMap<(u64, u8), u64>,
    max_blinks: u8,
) -> u64 {
    if blinks >= max_blinks {
        return 1;
    }

    if let Some(x) = memory.get(&(value, blinks)) {
        return *x;
    }

    if value == 0 {
        let score = calculate_value(1, blinks + 1, memory, max_blinks);
        memory.insert((value, blinks), score);
        return score;
    }

    if digit_count(value) % 2 == 0 {
        let (left, right) = split_digit(value);

        let score_left = calculate_value(left, blinks + 1, memory, max_blinks);
        memory.insert((left, blinks + 1), score_left);
        let score_right = calculate_value(right, blinks + 1, memory, max_blinks);
        memory.insert((right, blinks + 1), score_right);

        memory.insert((value, blinks), score_left + score_right);
        return score_left + score_right;
    }

    let score = calculate_value(value * 2024, blinks + 1, memory, max_blinks);
    memory.insert((value, blinks), score);

    score
}

fn puzzle(values: &Vec<u64>, part1: bool) -> u64 {
    let mut sum: u64 = 0;

    let mut memory: HashMap<(u64, u8), u64> = HashMap::new();

    for v in values {
        sum += calculate_value(*v, 0, &mut memory, if part1 { 25 } else { 75 });
    }

    sum
}

fn main() {
    // let parsed = parse_input("Example.txt");
    let parsed = parse_input("Input.txt");

    let p1 = puzzle(&parsed, true);
    let p2 = puzzle(&parsed, false);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
