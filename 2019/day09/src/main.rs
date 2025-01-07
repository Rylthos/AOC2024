use std::collections::BTreeMap;
use std::collections::VecDeque;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

const DEBUG_PRINT: bool = false;
const PRINT_MSG: bool = false;
const FAKE_INPUT: bool = false;

const BREAK_ON_INPUT: bool = false;

struct ProgramSettings {
    pub inputs: VecDeque<i64>,
    pub outputs: VecDeque<i64>,

    pub relative_base: i64,
}

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

fn get_parameter(
    index: i64,
    values: &BTreeMap<i64, i64>,
    mode: i64,
    write: bool,
    settings: &ProgramSettings,
) -> i64 {
    let value: i64 = match mode {
        0 => {
            let offset = values.get(&index).unwrap_or(&0);
            if write {
                *offset
            } else {
                *values.get(offset).unwrap_or(&0)
            }
        }
        1 => *values.get(&index).unwrap_or(&0),
        2 => {
            let offset = settings.relative_base + *values.get(&index).unwrap_or(&0);
            if write {
                offset
            } else {
                *values.get(&offset).unwrap_or(&0)
            }
        }
        _ => panic!("Unexpected mode"),
    };

    if DEBUG_PRINT {
        println!("{index}: {} -> {value}", values.get(&index).unwrap_or(&0));
    }

    return value;
}

fn get_1_parameter(
    program_counter: i64,
    values: &BTreeMap<i64, i64>,
    mode: (i64, i64, i64),
    write: (bool, bool, bool),
    settings: &ProgramSettings,
) -> i64 {
    let (m, _, _) = mode;
    let (m1, _, _) = write;
    return get_parameter(program_counter + 1, values, m, m1, settings);
}

fn get_2_parameter(
    program_counter: i64,
    values: &BTreeMap<i64, i64>,
    mode: (i64, i64, i64),
    write: (bool, bool, bool),
    settings: &ProgramSettings,
) -> (i64, i64) {
    let (a, b, _) = mode;
    let (a1, b1, _) = write;
    (
        get_parameter(program_counter + 1, values, a, a1, settings),
        get_parameter(program_counter + 2, values, b, b1, settings),
    )
}

fn get_3_parameter(
    program_counter: i64,
    values: &BTreeMap<i64, i64>,
    mode: (i64, i64, i64),
    write: (bool, bool, bool),
    settings: &ProgramSettings,
) -> (i64, i64, i64) {
    let (a, b, c) = mode;
    let (a1, b1, c1) = write;
    (
        get_parameter(program_counter + 1, values, a, a1, settings),
        get_parameter(program_counter + 2, values, b, b1, settings),
        get_parameter(program_counter + 3, values, c, c1, settings),
    )
}

fn get_mode(opcode: i64) -> ((i64, i64, i64), i64) {
    let a = (opcode / 10000) % 10;
    let b = (opcode / 1000) % 10;
    let c = (opcode / 100) % 10;
    let op = opcode % 100;

    return ((c, b, a), op);
}

fn parse_value(
    program_counter: &mut i64,
    values: &mut BTreeMap<i64, i64>,
    settings: &mut ProgramSettings,
) -> bool {
    let current_value = values.get(program_counter).unwrap_or(&0);
    let (mode, opcode) = get_mode(*current_value);
    let (a, b, c) = mode;
    if PRINT_MSG {
        print!("{program_counter:4} | {current_value:5} ({opcode:2}: {mode:?}) | ");
        if DEBUG_PRINT {
            println!();
        }
    }
    match opcode {
        1 => {
            let (v1, v2, target) = get_3_parameter(
                *program_counter,
                values,
                (a, b, c),
                (false, false, true),
                settings,
            );

            let value = v1 + v2;
            if PRINT_MSG {
                println!("Add: {value} [{target}] = {v1} + {v2}");
            }
            values.insert(target, value);

            *program_counter += 4;
        }

        2 => {
            let (v1, v2, target) = get_3_parameter(
                *program_counter,
                values,
                (a, b, c),
                (false, false, true),
                settings,
            );
            let value = v1 * v2;
            if PRINT_MSG {
                println!("Multiply: {value} [{target}] = {v1} * {v2}");
            }
            values.insert(target, value);

            *program_counter += 4;
        }

        3 => {
            let target = get_1_parameter(
                *program_counter,
                values,
                (a, 0, 0),
                (true, false, false),
                settings,
            );
            if PRINT_MSG {
                println!("Input: {target}");
            }

            let v: i64 = if FAKE_INPUT {
                if BREAK_ON_INPUT && settings.inputs.len() == 0 {
                    return true;
                }

                settings.inputs.pop_front().expect("No input")
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
            let value = get_1_parameter(
                *program_counter,
                values,
                (a, 0, 0),
                (false, false, false),
                settings,
            );
            if PRINT_MSG {
                println!("Output: {value}");
            }

            if FAKE_INPUT {
                settings.outputs.push_back(value);
            } else {
                println!(">> {}", value);
            }

            *program_counter += 2;
        }

        5 => {
            let (condition, target) = get_2_parameter(
                *program_counter,
                values,
                (a, b, 0),
                (false, false, false),
                settings,
            );
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
            let (condition, target) = get_2_parameter(
                *program_counter,
                values,
                (a, b, 0),
                (false, false, false),
                settings,
            );
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
            let (v1, v2, target) = get_3_parameter(
                *program_counter,
                values,
                (a, b, c),
                (false, false, true),
                settings,
            );
            let v = (v1 < v2) as i64;
            if PRINT_MSG {
                println!("less than: {v1} < {v2} -> {v} [{target}]");
            }

            values.insert(target, v);

            *program_counter += 4;
        }

        8 => {
            let (v1, v2, target) = get_3_parameter(
                *program_counter,
                values,
                (a, b, c),
                (false, false, true),
                settings,
            );
            let v = (v1 == v2) as i64;
            if PRINT_MSG {
                println!("equals: {v1} == {v2} -> {v} [{target}]");
            }

            values.insert(target, v);

            *program_counter += 4;
        }

        9 => {
            let v = get_1_parameter(
                *program_counter,
                values,
                (a, 0, c),
                (false, false, false),
                settings,
            );
            settings.relative_base += v;

            if PRINT_MSG {
                println!("modify_base: {} [{v}]", settings.relative_base);
            }

            *program_counter += 2;
        }

        99 => return true,

        _ => return true,
    }

    false
}

fn intcode(input: &BTreeMap<i64, i64>) -> BTreeMap<i64, i64> {
    let mut values = input.clone();

    let mut settings = ProgramSettings {
        inputs: VecDeque::new(),
        outputs: VecDeque::new(),
        relative_base: 0,
    };

    let mut program_counter = 0;
    loop {
        if (program_counter as usize) > values.len() {
            panic!("Unexpected end");
        }

        if parse_value(&mut program_counter, &mut values, &mut settings) {
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

    intcode(&input);
}
