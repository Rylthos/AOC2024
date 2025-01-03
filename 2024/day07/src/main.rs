use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

type Input = Vec<(i64, Vec<i64>)>;

fn parse_input(file_name: &str) -> Input {
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

    let mut input: Input = Vec::new();

    for line in s.split("\n") {
        if line.is_empty() {
            continue;
        }

        let split: Vec<&str> = line.split(":").collect();
        let target: i64 = split[0].parse().unwrap();

        let values: Vec<i64> = split[1]
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        input.push((target, values));
    }

    input
}

fn matching_digits(to_match: &i64, compare: &i64) -> Option<i64> {
    let mut current_match = *to_match;
    let mut current_compare = *compare;

    while current_compare > 0 && current_match > 0 {
        let last_digit_match = current_match % 10;
        let last_digit_compare = current_compare % 10;

        if last_digit_compare != last_digit_match {
            return None;
        }

        current_match /= 10;
        current_compare /= 10;
    }

    Some(current_match)
}

fn is_valid(target: &i64, values: &[i64], concatenation: bool) -> bool {
    if values.len() == 1 {
        return values[0] == *target;
    } else if values.is_empty() {
        return false;
    }

    // Addition
    {
        let new_target = target - values[0];
        if new_target >= 0 {
            let new_values: &[i64] = &values[1..];
            if is_valid(&new_target, new_values, concatenation) {
                return true;
            }
        }
    }

    // Multiplication
    if target % values[0] == 0 {
        let new_target = target / values[0];

        let new_values: &[i64] = &values[1..];
        if is_valid(&new_target, new_values, concatenation) {
            return true;
        }
    }

    if concatenation {
        if let Some(new_target) = matching_digits(target, &values[0]) {
            let new_values: &[i64] = &values[1..];
            if is_valid(&new_target, new_values, concatenation) {
                return true;
            }
        }
    }

    false
}

fn part1(input: &Input) -> i64 {
    let mut sum: i64 = 0;
    for (target, values) in input {
        let mut reversed = values.clone();
        reversed.reverse();
        if is_valid(target, &reversed, false) {
            sum += target;
        }
    }

    sum
}

fn part2(input: &Input) -> i64 {
    let mut sum: i64 = 0;
    for (target, values) in input {
        let mut reversed = values.clone();
        reversed.reverse();
        if is_valid(target, &reversed, true) {
            // println!("{:?} : {:?}", target, values);
            sum += target;
        }
    }

    sum
}

fn main() {
    // let input = parse_input("Example.txt");
    let input = parse_input("Input.txt");

    let p1 = part1(&input);
    let p2 = part2(&input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
