use std::collections::BTreeMap;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

const DEBUG_PRINT: bool = false;

fn parse_file(filename: &str) -> BTreeMap<i64, i64> {
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

    let v: Vec<i64> = s
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let mut memory: BTreeMap<i64, i64> = BTreeMap::new();
    for (i, v) in (0..).zip(v) {
        memory.insert(i, v);
    }

    memory
}

fn get_parameter(index: i64, values: &BTreeMap<i64, i64>, mode: i64) -> i64 {
    let value = match mode {
        0 => values.get(values.get(&index).unwrap()).unwrap(),
        1 => values.get(&index).unwrap(),
        _ => panic!("Unexpected mode"),
    };

    if DEBUG_PRINT {
        println!("{index}: {value}");
    }

    return *value;
}

fn get_1_parameter(
    program_counter: i64,
    values: &BTreeMap<i64, i64>,
    mode: (i64, i64, i64),
) -> i64 {
    let (m, _, _) = mode;
    return get_parameter(program_counter + 1, values, m);
}

fn get_2_parameter(
    program_counter: i64,
    values: &BTreeMap<i64, i64>,
    mode: (i64, i64, i64),
) -> (i64, i64) {
    let (a, b, _) = mode;
    (
        get_parameter(program_counter + 1, values, a),
        get_parameter(program_counter + 2, values, b),
    )
}

fn get_3_parameter(
    program_counter: i64,
    values: &BTreeMap<i64, i64>,
    mode: (i64, i64, i64),
) -> (i64, i64, i64) {
    let (a, b, c) = mode;
    (
        get_parameter(program_counter + 1, values, a),
        get_parameter(program_counter + 2, values, b),
        get_parameter(program_counter + 3, values, c),
    )
}

fn get_mode(opcode: i64) -> ((i64, i64, i64), i64) {
    let a = (opcode / 10000) % 10;
    let b = (opcode / 1000) % 10;
    let c = (opcode / 100) % 10;
    let op = opcode % 100;

    return ((c, b, a), op);
}

fn parse_value(program_counter: &mut i64, values: &mut BTreeMap<i64, i64>) -> bool {
    let current_value = values.get(program_counter).unwrap();
    let (mode, opcode) = get_mode(*current_value);
    let (a, b, c) = mode;
    if DEBUG_PRINT {
        println!("{current_value} -> {opcode} : {mode:?}");
    }
    print!("{program_counter:4} | {current_value:5} ({opcode:2}: {mode:?}) | ");
    match opcode {
        1 => {
            let (v1, v2, target) = get_3_parameter(*program_counter, values, (a, b, 1));

            let value = v1 + v2;
            println!("Add: {value} [{target}] = {v1} + {v2}");
            values.insert(target, value);

            *program_counter += 4;
        }

        2 => {
            let (v1, v2, target) = get_3_parameter(*program_counter, values, (a, b, 1));
            let value = v1 * v2;
            println!("Multiply: {value} [{target}] = {v1} * {v2}");
            values.insert(target, value);

            *program_counter += 4;
        }

        3 => {
            let target = get_1_parameter(*program_counter, values, (1, 0, 0));
            println!("Input: {target}");
            print!("> ");
            let _ = std::io::stdout().flush();
            let mut input_line = String::new();
            io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");

            let v: i64 = input_line.trim().parse().expect("Failed to parse line");
            values.insert(target, v);

            *program_counter += 2;
        }

        4 => {
            let value = get_1_parameter(*program_counter, values, mode);
            println!("Output: {value}");
            println!(">> {}", value);

            *program_counter += 2;
        }

        5 => {
            let (condition, target) = get_2_parameter(*program_counter, values, (a, b, 0));
            println!("jump-if-true: {condition} -> {target}");
            if condition != 0 {
                *program_counter = target;
            } else {
                *program_counter += 3;
            }
        }

        6 => {
            let (condition, target) = get_2_parameter(*program_counter, values, (a, b, 0));
            println!("jump-if-false: {condition} -> {target}");
            if condition == 0 {
                *program_counter = target;
            } else {
                *program_counter += 3;
            }
        }

        7 => {
            let (v1, v2, target) = get_3_parameter(*program_counter, values, (a, b, 1));
            let v = (v1 < v2) as i64;
            println!("less than: {v1} < {v2} -> {v} [{target}]");

            values.insert(target, v);

            *program_counter += 4;
        }

        8 => {
            let (v1, v2, target) = get_3_parameter(*program_counter, values, (a, b, 1));
            let v = (v1 == v2) as i64;
            println!("equals: {v1} == {v2} -> {v} [{target}]");

            values.insert(target, v);

            *program_counter += 4;
        }

        99 => return true,

        _ => return true,
    }

    false
}

fn intcode(input: &BTreeMap<i64, i64>) -> BTreeMap<i64, i64> {
    let mut values = input.clone();

    let mut program_counter = 0;
    loop {
        if (program_counter as usize) > values.len() {
            panic!("Unexpected end");
        }

        if parse_value(&mut program_counter, &mut values) {
            break;
        }
    }

    values
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Incorrect Usage");
    }

    let input = parse_file(&args[1]);

    let output = intcode(&input);
    if DEBUG_PRINT {
        println!("{:?}", output);
    }
}
