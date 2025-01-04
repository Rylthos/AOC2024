use std::collections::HashMap;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
enum Operation {
    Right(i32),
    Left(i32),
    Up(i32),
    Down(i32),
}

fn parse_file(filename: &str) -> String {
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

    s
}

fn parse_operation(input: &str) -> Operation {
    let value: i32 = input[1..].parse().unwrap();

    match input.chars().nth(0).unwrap() {
        'U' => Operation::Up(value),
        'D' => Operation::Down(value),
        'R' => Operation::Right(value),
        'L' => Operation::Left(value),
        _ => panic!("Unexpected Operation"),
    }
}

fn puzzle(input: &String) -> (i32, i32) {
    let inputs: Vec<Vec<Operation>> = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.split(",").map(|v| parse_operation(v)).collect())
        .collect();

    let mut stored: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut shortest_distance = i32::MAX;
    let mut shortest_signal = i32::MAX;

    let contained = |stored: &HashMap<(i32, i32), (i32, i32)>,
                     shortest_distance: &mut i32,
                     shortest_signal: &mut i32,
                     id,
                     x,
                     y,
                     signal| {
        if let Some((i, s)) = stored.get(&(x, y)) {
            if *i != id {
                *shortest_distance = i32::min(*shortest_distance, i32::abs(x) + i32::abs(y));
                *shortest_signal = i32::min(*shortest_signal, s + signal)
            }
        }
    };

    for id in 0..2 {
        let mut x = 0;
        let mut y = 0;
        let mut signal = 0;
        for o in inputs[id as usize].iter() {
            match o {
                Operation::Up(v) => {
                    for _ in 0..*v {
                        y += 1;
                        signal += 1;

                        contained(
                            &stored,
                            &mut shortest_distance,
                            &mut shortest_signal,
                            id,
                            x,
                            y,
                            signal,
                        );
                        stored.insert((x, y), (id, signal));
                    }
                }
                Operation::Down(v) => {
                    for _ in 0..*v {
                        y -= 1;
                        signal += 1;

                        contained(
                            &stored,
                            &mut shortest_distance,
                            &mut shortest_signal,
                            id,
                            x,
                            y,
                            signal,
                        );
                        stored.insert((x, y), (id, signal));
                    }
                }
                Operation::Right(v) => {
                    for _ in 0..*v {
                        x += 1;
                        signal += 1;

                        contained(
                            &stored,
                            &mut shortest_distance,
                            &mut shortest_signal,
                            id,
                            x,
                            y,
                            signal,
                        );
                        stored.insert((x, y), (id, signal));
                    }
                }
                Operation::Left(v) => {
                    for _ in 0..*v {
                        x -= 1;
                        signal += 1;

                        contained(
                            &stored,
                            &mut shortest_distance,
                            &mut shortest_signal,
                            id,
                            x,
                            y,
                            signal,
                        );
                        stored.insert((x, y), (id, signal));
                    }
                }
            }
        }
    }

    (shortest_distance, shortest_signal)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Incorrect Usage");
    }

    let input = parse_file(args.get(1).unwrap());

    let (p1, p2) = puzzle(&input);
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
