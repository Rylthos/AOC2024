use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use regex::Regex;

#[derive(Debug, PartialEq)]
enum Operation {
    AND,
    XOR,
    OR,
}

fn parse_input(
    file_name: &str,
) -> (
    HashMap<String, bool>,
    HashMap<String, (String, Operation, String)>,
) {
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

    let mut inputs: HashMap<String, bool> = HashMap::new();
    let mut targets: HashMap<String, (String, Operation, String)> = HashMap::new();

    let mut passing_inputs = true;

    let input_parse = Regex::new(r"([xy]\d\d): (\d)").unwrap();
    let target_parse = Regex::new(r"(\w+) (\w+) (\w+) -> (\w+)").unwrap();

    for line in s.split("\n") {
        if line.is_empty() {
            passing_inputs = false;
            continue;
        }

        if passing_inputs {
            for (_, [reg, v]) in input_parse.captures_iter(&line).map(|s| s.extract()) {
                inputs.insert(reg.to_string(), v.parse::<i32>().unwrap() != 0);
            }
        } else {
            for (_, [reg1, op, reg2, v]) in target_parse.captures_iter(&line).map(|s| s.extract()) {
                let op_v = match op {
                    "AND" => Operation::AND,
                    "OR" => Operation::OR,
                    "XOR" => Operation::XOR,
                    _ => panic!("Unexecpted"),
                };

                targets.insert(v.to_string(), (reg1.to_string(), op_v, reg2.to_string()));
            }
        }
    }

    (inputs, targets)
}

fn first_char(str: &String) -> char {
    str.chars().nth(0).unwrap()
}

fn calc_value(
    target: &String,
    registers: &mut HashMap<String, bool>,
    targets: &HashMap<String, (String, Operation, String)>,
) -> bool {
    if let Some(v) = registers.get(target) {
        return *v;
    }

    let (reg1, op, reg2) = targets.get(&target.clone()).unwrap();

    // let str = match op {
    //     Operation::AND => "AND",
    //     Operation::OR => "OR",
    //     Operation::XOR => "XOR",
    // };

    let v1 = calc_value(reg1, registers, targets);
    let v2 = calc_value(reg2, registers, targets);

    let final_value = match op {
        Operation::AND => v1 & v2,
        Operation::OR => v1 | v2,
        Operation::XOR => v1 ^ v2,
    };

    return final_value;
}

fn calc_output(
    inputs: &HashMap<String, bool>,
    targets: &HashMap<String, (String, Operation, String)>,
) -> u64 {
    let mut inputs = inputs.clone();

    let mut outputs: BTreeSet<String> = BTreeSet::new();
    for (target, _) in targets {
        if target.chars().nth(0).unwrap() == 'z' {
            outputs.insert(target.clone());
        }
    }

    let mut output = 0;
    for o in outputs.iter() {
        output = output << 1;

        let v = calc_value(o, &mut inputs, &targets);
        output |= v as u64;
    }

    output
}

fn part1(
    inputs: &HashMap<String, bool>,
    targets: &HashMap<String, (String, Operation, String)>,
) -> u64 {
    calc_output(inputs, targets)
}

fn main() {
    // let (inputs, targets) = parse_input("SmallExample.txt");
    // let (inputs, targets) = parse_input("Example.txt");
    // let (inputs, targets) = parse_input("SimpleExample.txt");
    let (inputs, targets) = parse_input("Input.txt");

    let p1 = part1(&inputs, &targets);
    println!("Part 1: {p1}");
}
