use std::collections::BTreeMap;
use std::collections::VecDeque;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

const DEBUG_PRINT: bool = false;
const PRINT_MSG: bool = false;
const FAKE_INPUT: bool = true;

const BREAK_ON_INPUT: bool = true;

static mut INPUTS: VecDeque<i64> = VecDeque::new();
static mut OUTPUTS: VecDeque<i64> = VecDeque::new();

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
    if PRINT_MSG {
        print!("{program_counter:4} | {current_value:5} ({opcode:2}: {mode:?}) | ");
    }
    match opcode {
        1 => {
            let (v1, v2, target) = get_3_parameter(*program_counter, values, (a, b, 1));

            let value = v1 + v2;
            if PRINT_MSG {
                println!("Add: {value} [{target}] = {v1} + {v2}");
            }
            values.insert(target, value);

            *program_counter += 4;
        }

        2 => {
            let (v1, v2, target) = get_3_parameter(*program_counter, values, (a, b, 1));
            let value = v1 * v2;
            if PRINT_MSG {
                println!("Multiply: {value} [{target}] = {v1} * {v2}");
            }
            values.insert(target, value);

            *program_counter += 4;
        }

        3 => {
            let target = get_1_parameter(*program_counter, values, (1, 0, 0));
            if PRINT_MSG {
                println!("Input: {target}");
            }

            let v: i64 = if FAKE_INPUT {
                unsafe {
                    if BREAK_ON_INPUT && INPUTS.len() == 0 {
                        return true;
                    }
                }

                unsafe { INPUTS.pop_front().expect("No input") }
            } else {
                print!("> ");
                let _ = std::io::stdout().flush();
                let mut input_line = String::new();
                io::stdin()
                    .read_line(&mut input_line)
                    .expect("Failed to read line");
                input_line.trim().parse().expect("Failed to parse line")
            };

            values.insert(target, v);

            *program_counter += 2;
        }

        4 => {
            let value = get_1_parameter(*program_counter, values, mode);
            if PRINT_MSG {
                println!("Output: {value}");
            }

            if FAKE_INPUT {
                unsafe {
                    OUTPUTS.push_back(value);
                }
            } else {
                println!(">> {}", value);
            }

            *program_counter += 2;
        }

        5 => {
            let (condition, target) = get_2_parameter(*program_counter, values, (a, b, 0));
            if PRINT_MSG {
                println!("jump-if-true: {condition} -> {target}");
            }
            if condition != 0 {
                *program_counter = target;
            } else {
                *program_counter += 3;
            }
        }

        6 => {
            let (condition, target) = get_2_parameter(*program_counter, values, (a, b, 0));
            if PRINT_MSG {
                println!("jump-if-false: {condition} -> {target}");
            }
            if condition == 0 {
                *program_counter = target;
            } else {
                *program_counter += 3;
            }
        }

        7 => {
            let (v1, v2, target) = get_3_parameter(*program_counter, values, (a, b, 1));
            let v = (v1 < v2) as i64;
            if PRINT_MSG {
                println!("less than: {v1} < {v2} -> {v} [{target}]");
            }

            values.insert(target, v);

            *program_counter += 4;
        }

        8 => {
            let (v1, v2, target) = get_3_parameter(*program_counter, values, (a, b, 1));
            let v = (v1 == v2) as i64;
            if PRINT_MSG {
                println!("equals: {v1} == {v2} -> {v} [{target}]");
            }

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

fn part1(input: &BTreeMap<i64, i64>) {
    let mut max = 0;
    let mut max_value = 0;
    for i in 10000..100000 {
        let inputs = [
            i / 10000 % 10,
            i / 1000 % 10,
            i / 100 % 10,
            i / 10 % 10,
            i / 1 % 10,
        ];

        let mut count: BTreeMap<i64, i32> = BTreeMap::new();
        for j in inputs {
            if let Some(x) = count.get(&j) {
                count.insert(j, x + 1);
            } else {
                count.insert(j, 1);
            }
        }

        let mut skip_value = false;
        for (k, v) in count {
            if k > 4 {
                skip_value = true;
                break;
            } else if v > 1 {
                skip_value = true;
            }
        }

        if skip_value {
            continue;
        }

        let mut previous = 0;

        for j in inputs {
            unsafe {
                INPUTS.clear();
                INPUTS.push_back(j);
                INPUTS.push_back(previous);

                intcode(input);

                let output = OUTPUTS.pop_front().unwrap();
                previous = output;
            }
        }

        if previous > max {
            max = previous;
            max_value = i;
        }
    }
    println!("Part 1: {max_value} -> {max}");
}

fn part2(input: &BTreeMap<i64, i64>) {
    let mut max_value = 0;
    let mut max = 0;
    for i in 10000..100000 {
        let current_input = [
            i / 10000 % 10,
            i / 1000 % 10,
            i / 100 % 10,
            i / 10 % 10,
            i / 1 % 10,
        ];

        let mut count: BTreeMap<i64, i32> = BTreeMap::new();
        for j in current_input {
            if let Some(x) = count.get(&j) {
                count.insert(j, x + 1);
            } else {
                count.insert(j, 1);
            }
        }

        let mut skip_value = false;
        for (k, v) in count {
            if k < 5 {
                skip_value = true;
                break;
            } else if v > 1 {
                skip_value = true;
            }
        }

        if skip_value {
            continue;
        }

        let mut amps = [
            (0, input.clone()),
            (0, input.clone()),
            (0, input.clone()),
            (0, input.clone()),
            (0, input.clone()),
        ];
        let mut current_amp = 0;

        let mut previous = 0;
        for ((pc, mem), i) in amps.iter_mut().zip(current_input) {
            unsafe {
                INPUTS.clear();
                INPUTS.push_back(i);
                INPUTS.push_back(previous);

                let mut pc2 = pc.clone();
                let mut mem2 = mem.clone();
                loop {
                    if parse_value(&mut pc2, &mut mem2) {
                        break;
                    }
                }

                if *mem.get(&pc2).unwrap() == 99 {
                    break;
                }

                previous = OUTPUTS.pop_front().unwrap();

                *pc = pc2.clone();
                *mem = mem2.clone();
            }
        }

        loop {
            let (pc, mem) = &amps[current_amp];

            if *mem.get(&pc).unwrap() == 99 {
                break;
            }

            unsafe {
                INPUTS.clear();
                INPUTS.push_back(previous);

                let mut pc2 = pc.clone();
                let mut mem2 = mem.clone();
                loop {
                    if parse_value(&mut pc2, &mut mem2) {
                        break;
                    }
                }

                previous = OUTPUTS.pop_front().unwrap();

                amps[current_amp] = (pc2, mem2);
            }

            current_amp = (current_amp + 1) % 5;

            if previous > max {
                max = previous;
                max_value = i;
            }
        }
    }

    println!("Part2: {max_value} -> {max}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Incorrect Usage");
    }

    let input = parse_file(&args[1]);

    unsafe {
        INPUTS.push_back(4);
        INPUTS.push_back(0);
    }

    part1(&input);
    part2(&input);
}
