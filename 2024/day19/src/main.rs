use std::cmp::Ordering;
// use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
// use std::collections::VecDeque;
use std::fs::File;
// use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    WHITE,
    BLUE,
    BLACK,
    RED,
    GREEN,
}

fn parse_input(file_name: &str) -> (Vec<Vec<State>>, Vec<Vec<State>>) {
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

    let lines: Vec<&str> = s.split("\n").filter(|s| !s.is_empty()).collect();

    let mut rules: Vec<Vec<State>> = Vec::new();
    let mut targets: Vec<Vec<State>> = Vec::new();

    for rule in lines[0].split(", ") {
        let mut r: Vec<State> = Vec::new();
        for c in rule.chars() {
            let new_state = match c {
                'w' => State::WHITE,
                'u' => State::BLUE,
                'b' => State::BLACK,
                'r' => State::RED,
                'g' => State::GREEN,
                _ => panic!(),
            };

            r.push(new_state);
        }
        rules.push(r);
    }

    for target in &lines[1..] {
        let mut r: Vec<State> = Vec::new();
        for c in target.chars() {
            let new_state = match c {
                'w' => State::WHITE,
                'u' => State::BLUE,
                'b' => State::BLACK,
                'r' => State::RED,
                'g' => State::GREEN,
                _ => panic!(),
            };

            r.push(new_state);
        }
        targets.push(r);
    }

    (rules, targets)
}

fn print_sequence(seq: &Vec<State>) {
    for s in seq {
        let c = match s {
            State::WHITE => 'w',
            State::BLUE => 'u',
            State::BLACK => 'b',
            State::RED => 'r',
            State::GREEN => 'g',
        };
        print!("{c}");
    }
    println!();
}

fn check_rule(rule: &Vec<State>, target: &Vec<State>, start_index: usize) -> Result<usize, ()> {
    for i in 0..rule.len() {
        let r = rule[i];
        if start_index + i >= target.len() {
            return Err(());
        }

        let t = target[start_index + i];

        if r != t {
            return Err(());
        }
    }

    Ok(start_index + rule.len())
}

fn check_target(
    rules: &Vec<Vec<State>>,
    target: &Vec<State>,
    start_index: usize,
    memory: &mut HashMap<Vec<State>, u64>,
) -> Result<u64, ()> {
    if start_index == target.len() {
        return Ok(1);
    }

    if memory.contains_key(&target[start_index..]) {
        return Ok(*memory.get(&target[start_index..]).unwrap());
    }

    let mut count = 0;

    for rule in rules {
        if let Ok(new_index) = check_rule(rule, target, start_index) {
            if let Ok(x) = check_target(rules, target, new_index, memory) {
                count += x;
            }
        }
    }

    memory.insert(target[start_index..].to_vec(), count);

    if count != 0 {
        return Ok(count);
    } else {
        Err(())
    }
}

fn puzzle(rules: &Vec<Vec<State>>, targets: &Vec<Vec<State>>) -> (u64, u64) {
    let mut count = 0;
    let mut total = 0;

    let mut memory: HashMap<Vec<State>, u64> = HashMap::new();

    for target in targets {
        if let Ok(x) = check_target(rules, target, 0, &mut memory) {
            count += 1;
            total += x;
            print!("Seq: ");
            print_sequence(target);
            println!("{}", x);
        }
    }

    (count, total)
}

fn main() {
    // let (rules, inputs) = parse_input("Example.txt");
    let (rules, inputs) = parse_input("Input.txt");

    let (p1, p2) = puzzle(&rules, &inputs);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    // println!("{inputs:?}");
}
