use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn parse_input(file_name: &str) -> (Vec<u8>, u64, u64, u64) {
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

    let mut operations: Vec<u8> = Vec::new();

    let lines: Vec<&str> = s.split("\n").collect();

    let reg_a: u64 = lines[0][12..].parse().unwrap();
    let reg_b: u64 = lines[1][12..].parse().unwrap();
    let reg_c: u64 = lines[2][12..].parse().unwrap();

    for char in lines[4][9..].split(",") {
        operations.push(char.parse().unwrap());
    }

    (operations, reg_a, reg_b, reg_c)
}

fn check_instr(
    input: &Vec<u8>,
    reg_a: &mut u64,
    reg_b: &mut u64,
    reg_c: &mut u64,
    instr_counter: &mut usize,
) -> Option<u8> {
    let opcode = input[*instr_counter];
    let literal_operand = input[*instr_counter + 1];

    let combo_operand: u64 = match literal_operand {
        0 | 1 | 2 | 3 => literal_operand as u64,
        4 => *reg_a,
        5 => *reg_b,
        6 => *reg_c,
        _ => 0,
    };

    let mut should_increase = true;

    let mut return_value = None;

    match opcode {
        0 => {
            let denom = u64::pow(2, combo_operand as u32);
            *reg_a = *reg_a / denom;
        }
        1 => {
            *reg_b = *reg_b ^ (literal_operand as u64);
        }
        2 => *reg_b = combo_operand % 8,
        3 => {
            if *reg_a != 0 {
                *instr_counter = literal_operand as usize;
                should_increase = false;
            }
        }
        4 => {
            *reg_b = *reg_b ^ *reg_c;
        }
        5 => {
            return_value = Some((combo_operand % 8) as u8);
        }
        6 => {
            let denom = u64::pow(2, combo_operand as u32);
            *reg_b = *reg_a / denom;
        }
        7 => {
            let denom = u64::pow(2, combo_operand as u32);
            *reg_c = *reg_a / denom;
        }
        _ => panic!("Unknown opcode"),
    }

    if should_increase {
        *instr_counter = *instr_counter + 2;
    }

    return_value
}

fn part1(input: &Vec<u8>, reg_a: u64, reg_b: u64, reg_c: u64) {
    let mut has_printed = false;
    let mut instr_counter = 0;

    let mut reg_a = reg_a;
    let mut reg_b = reg_b;
    let mut reg_c = reg_c;

    while instr_counter < input.len() {
        if let Some(x) = check_instr(
            &input,
            &mut reg_a,
            &mut reg_b,
            &mut reg_c,
            &mut instr_counter,
        ) {
            if has_printed {
                print!(",");
            }
            print!("{}", x);
            has_printed = true;
        }
    }
}

// Only works for true input
fn part2(input: &Vec<u8>, reg_b: u64, reg_c: u64) -> u64 {
    for initial_a in 0.. {
        let patterns: Vec<u64> = vec![
            0b101000011110110000001110011011,
            0b101000011110110000001110011101,
            0b101000011110110010001110011011,
            0b101000011110110010001110011101,
            0b101001011110110000001110011011,
            0b101001011110110000001110011101,
            0b101001011110110010001110011011,
            0b101001011110110010001110011101,
        ];
        for p in patterns {
            let starting_value = p + (initial_a << 30);
            let mut instr_counter = 0;
            let mut current_check = 0;
            let mut reg_a = starting_value;
            let mut reg_b = reg_b;
            let mut reg_c = reg_c;

            let mut valid = true;

            while instr_counter < input.len() {
                if let Some(x) = check_instr(
                    &input,
                    &mut reg_a,
                    &mut reg_b,
                    &mut reg_c,
                    &mut instr_counter,
                ) {
                    if current_check >= input.len() {
                        println!("Exceeded: {starting_value}");
                        valid = false;
                    }

                    if valid && x != input[current_check] {
                        break;
                    } else if valid {
                        current_check += 1;
                    }
                }
            }

            if current_check == input.len() && valid {
                return starting_value;
            }
            if current_check >= 14 && valid {
                let valid_digits = &input[0..current_check];
                println!(
                    "Checked: {starting_value} | Correct: {}/{} | {:?}",
                    current_check,
                    input.len(),
                    valid_digits
                );
            }
        }
    }

    0
}

fn main() {
    // let (operations, reg_a, reg_b, reg_c) = parse_input("Example.txt");
    // let (operations, reg_a, reg_b, reg_c) = parse_input("Example2.txt");
    // let (operations, reg_a, reg_b, reg_c) = parse_input("TestInput.txt");
    let (operations, reg_a, reg_b, reg_c) = parse_input("Input.txt");

    print!("Part 1: ");
    part1(&operations, reg_a, reg_b, reg_c);
    println!("");

    let p2 = part2(&operations, reg_b, reg_c);
    println!("Part 2: {p2}");
}
