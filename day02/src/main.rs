use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn parse_input(file_name: &str) -> Vec<Vec<i32>> {
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(err) => panic!("Couldn't open {}: {}", display, err),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(err) => panic!("Couldn't read {}: {}", display, err),
        Ok(_) => (),
    }

    let mut results: Vec<Vec<i32>> = Vec::new();

    for line in s.split("\n") {
        let numbers: Vec<i32> = line
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        if numbers.len() > 0 {
            results.push(numbers);
        }
    }

    return results;
}

fn parse_report(report: Vec<i32>, has_removed_value: bool) -> bool {
    let mut previous = report[0];
    let is_increasing = report[1] > previous;
    let mut is_valid: bool = true;

    let parse_others = |i: usize| -> bool {
        let mut copy = report.clone();
        copy.remove(i);
        return parse_report(copy, true);
    };

    for level_index in 1..report.len() {
        let value = report[level_index];
        let diff = value - previous;
        match is_increasing {
            true => {
                if diff <= 0 || diff > 3 {
                    if has_removed_value {
                        is_valid = false;
                        break;
                    } else {
                        for j in 0..level_index + 1 {
                            if parse_others(j) {
                                return true;
                            }
                        }
                        return false;
                    }
                }
            }
            false => {
                if diff >= 0 || diff < -3 {
                    if has_removed_value {
                        is_valid = false;
                        break;
                    } else {
                        for j in 0..level_index + 1 {
                            if parse_others(j) {
                                return true;
                            }
                        }
                        return false;
                    }
                }
            }
        }
        previous = value;
    }

    return is_valid;
}

fn part1(input: Vec<Vec<i32>>) -> u64 {
    let mut sum: u64 = 0;
    for report in input {
        if parse_report(report, true) {
            sum += 1;
        }
    }

    return sum;
}

fn part2(input: Vec<Vec<i32>>) -> u64 {
    let mut sum: u64 = 0;
    for report in input {
        if parse_report(report, false) {
            sum += 1;
        }
    }

    return sum;
}

fn main() {
    // let input = parse_input("Example.txt");
    let input = parse_input("Input.txt");

    // println!("{:?}", input);

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}
