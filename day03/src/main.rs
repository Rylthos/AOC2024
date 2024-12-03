use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn parse_input(file_name: &str) -> Vec<String> {
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

    let mut input = String::new();
    let re = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();

    for line in s.split("\n") {
        input += line;
    }

    let mut inputs = vec![];

    for (_, [value]) in re.captures_iter(&input).map(|s| s.extract()) {
        inputs.push(String::from(value));
    }

    return inputs;
}

fn parse_inputs(inputs: &Vec<String>, should_disable: bool) -> i64 {
    let mut sum: i64 = 0;
    let mut enabled = true;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for input in inputs {
        if input.chars().nth(0).unwrap() == 'm' {
            for (_, [v1, v2]) in re.captures_iter(&input).map(|s| s.extract()) {
                // println!(
                //     "mul {}, {}",
                //     v1.parse::<i64>().unwrap(),
                //     v2.parse::<i64>().unwrap()
                // );
                let v1_i: i64 = v1.parse().unwrap();
                let v2_i: i64 = v2.parse().unwrap();
                if enabled {
                    sum += v1_i * v2_i;
                }
            }
        } else if should_disable {
            if input.len() == 4 {
                // do
                enabled = true;
            } else
            // dont
            {
                enabled = false;
            }
        }
    }
    return sum;
}

fn part1(inputs: &Vec<String>) -> i64 {
    parse_inputs(inputs, false)
}

fn part2(inputs: &Vec<String>) -> i64 {
    parse_inputs(inputs, true)
}

fn main() {
    // let inputs = parse_input("Example2.txt");
    let inputs = parse_input("Input.txt");
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
    //
    // println!("Hello, world!");
}
