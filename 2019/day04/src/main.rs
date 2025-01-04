use std::collections::HashMap;
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

fn is_valid_num(num: i32, allow_repeated: bool) -> bool {
    assert!(num >= 100000);
    assert!(num <= 1000000);

    let mut double = false;

    let mut values: HashMap<i32, i32> = HashMap::new();

    let mut previous_digit = 0;
    for i in 0..6 {
        let digit = i32::pow(10, 5 - i);
        let v = num / digit % 10;
        let diff = v - previous_digit;
        previous_digit = v;

        if let Some(x) = values.get(&v) {
            values.insert(v, x + 1);
        } else {
            values.insert(v, 1);
        }

        if diff < 0 {
            return false;
        } else if diff == 0 {
            double = true;
        }
    }

    if !allow_repeated {
        for (_, v) in values.iter() {
            if *v == 2 {
                return true;
            }
        }
        return false;
    }

    double
}

fn part1() -> (i32, i32) {
    (165432..=707912)
        .map(|v| (is_valid_num(v, true) as i32, is_valid_num(v, false) as i32))
        .fold((0, 0), |(sum_a, sum_b), (a, b)| (sum_a + a, sum_b + b))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (p1, p2) = part1();
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
