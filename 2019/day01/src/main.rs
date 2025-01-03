use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

fn parse_value(value: i64, full: bool) -> i64 {
    let new_value = value / 3 - 2;
    if new_value < 0 {
        return 0;
    }

    if full {
        new_value + parse_value(new_value, full)
    } else {
        new_value
    }
}

fn puzzle(input: &String, full: bool) -> i64 {
    input
        .split("\n")
        .filter(|x| x.len() != 0)
        .map(|s| s.parse::<i64>().unwrap())
        .map(|x| parse_value(x, full))
        .fold(0, |sum, v| sum + v)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Incorrect Usage");
    }

    let input = parse_file(args.get(1).unwrap());

    let p1 = puzzle(&input, false);
    println!("Part 1: {p1}");

    let p2 = puzzle(&input, true);
    println!("Part 2: {p2}");
}
