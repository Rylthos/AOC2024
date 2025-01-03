use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const PRINT_DEBUG: bool = false;

fn parse_file(filename: &str) -> Vec<u64> {
    let path = Path::new(filename);

    let mut file = match File::open(&path) {
        Err(_) => panic!("Couldn't open file"),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_) => panic!("Couldn't read file"),
        Ok(_) => (),
    };

    s.split(",")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn get_value(index: usize, values: &Vec<u64>) -> u64 {
    *values.get(index).unwrap()
}

fn parse_value(program_counter: &mut usize, values: &mut Vec<u64>) -> bool {
    let current_value = values.get(*program_counter).unwrap();
    match current_value {
        1 => {
            let a1 = get_value(*program_counter + 1, values);
            let v1 = values.get(a1 as usize).unwrap().clone();
            let a2 = get_value(*program_counter + 2, values);
            let v2 = values.get(a2 as usize).unwrap().clone();
            let target = get_value(*program_counter + 3, values);

            let value = v1 + v2;
            values[target as usize] = value;

            if PRINT_DEBUG {
                println!("{program_counter}: {target} [{value}] = {a1} [{v1}] + {a2} [{v2}]");
            }
            *program_counter += 4;
        }

        2 => {
            let a1 = get_value(*program_counter + 1, values);
            let v1 = values.get(a1 as usize).unwrap().clone();
            let a2 = get_value(*program_counter + 2, values);
            let v2 = values.get(a2 as usize).unwrap().clone();
            let target = get_value(*program_counter + 3, values);

            let value = v1 * v2;
            values[target as usize] = value;

            if PRINT_DEBUG {
                println!("{program_counter}: {target} [{value}] = {a1} [{v1}] * {a2} [{v2}]");
            }
            *program_counter += 4;
        }

        99 => return true,

        _ => (),
    }

    false
}

fn intcode(input: &Vec<u64>) -> Vec<u64> {
    let mut values = input.clone();

    let mut program_counter = 0;
    loop {
        if program_counter > values.len() {
            panic!("Unexpected end");
        }

        if parse_value(&mut program_counter, &mut values) {
            break;
        }
    }

    values
}

fn part1(input: &Vec<u64>) -> u64 {
    let mut input = input.clone();
    input[1] = 12;
    input[2] = 2;

    intcode(&input)[0]
}

fn part2(input: &Vec<u64>) -> u64 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut temp = input.clone();
            temp[1] = noun;
            temp[2] = verb;

            let output = intcode(&temp);

            println!("{} {} = {}", temp[1], temp[2], output[0]);

            if output[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Incorrect Usage");
    }

    let input = parse_file(&args[1]);

    let p1 = part1(&input);
    println!("Part 1: {p1}");

    let p2 = part2(&input);
    println!("Part 2: {p2}");
}
