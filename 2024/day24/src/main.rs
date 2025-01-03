use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
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

fn calc_value(
    target: &String,
    registers: &mut HashMap<String, bool>,
    targets: &HashMap<String, (String, Operation, String)>,
    added: &mut HashSet<(String, String)>,
    lookup: &mut HashMap<String, String>,
) -> bool {
    if let Some(v) = registers.get(target) {
        return *v;
    }

    let (reg1, op, reg2) = targets.get(&target.clone()).unwrap();

    let (reg1, reg2) = if reg1 < reg2 {
        (reg1, reg2)
    } else {
        (reg2, reg1)
    };

    let v1 = calc_value(reg1, registers, targets, added, lookup);
    let v2 = calc_value(reg2, registers, targets, added, lookup);

    let l = lookup.clone();

    let r1 = l.get(reg1).unwrap_or(reg1);
    let r2 = l.get(reg2).unwrap_or(reg2);

    // let (r1, r2) = if r1 < r2 { (r1, r2) } else { (r2, r1) };

    let mut new_target = String::new();

    let str = match op {
        Operation::AND => "AND",
        Operation::OR => "OR",
        Operation::XOR => "XOR",
    };

    if !l.contains_key(target) {
        if r1[1..] == r2[1..] {
            if r1[..1] == *"x" && r2[..1] == *"y" {
                let rv: u64 = r1[1..].parse().unwrap();

                new_target = match op {
                    Operation::AND => format!("A{:02}", rv),
                    Operation::OR => format!("O{:02}", rv),
                    Operation::XOR => format!("X{:02}", rv),
                };

                lookup.insert(target.to_string(), new_target.clone());
            }
        } else {
            new_target = lookup.get(target).unwrap_or(target).to_string();
        }
    } else {
        new_target = lookup.get(target).unwrap().to_string();
    }

    println!("{r1} {str} {r2} = {new_target} [{target}]");

    let final_value = match op {
        Operation::AND => v1 & v2,
        Operation::OR => v1 | v2,
        Operation::XOR => v1 ^ v2,
    };

    registers.insert(target.to_string(), final_value);

    return final_value;
}

fn calc_output(
    inputs: &HashMap<String, bool>,
    targets: &HashMap<String, (String, Operation, String)>,
) -> u64 {
    let mut inputs = inputs.clone();

    let mut lookup: HashMap<String, String> = HashMap::new();

    let mut outputs: BTreeSet<String> = BTreeSet::new();
    for (target, _) in targets {
        if target.chars().nth(0).unwrap() == 'z' {
            outputs.insert(target.clone());
        }
    }

    let mut added: HashSet<(String, String)> = HashSet::new();
    let mut output: u64 = 0;
    for (i, o) in (0..).zip(outputs.iter()) {
        let v = calc_value(o, &mut inputs, &targets, &mut added, &mut lookup);
        output |= ((v as u64) << i) as u64;

        println!();
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
    // let (inputs, targets) = parse_input("Changed.txt");

    let p1 = part1(&inputs, &targets);
    println!("Part 1: {p1}");
}
