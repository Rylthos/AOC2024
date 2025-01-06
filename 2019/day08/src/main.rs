use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::prelude::*;

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

fn part1(input: &str, width: i32, height: i32) -> u64 {
    let mut layers: Vec<String> = Vec::new();
    let mut current_string: String = String::new();
    let mut current_index = 0;
    for i in input.chars() {
        current_string.push(i);
        current_index += 1;
        if current_index == width * height {
            layers.push(current_string);
            current_index = 0;
            current_string = String::new();
        }
    }

    let (a, b) = layers
        .iter()
        .map(|l| {
            let v = l.chars().fold(0, |c, v| if v == '0' { c + 1 } else { c });
            (v, l)
        })
        .fold((i32::MAX, String::new()), |(m, p), (c, l)| {
            if c < m {
                (c, l.to_string())
            } else {
                (m, p)
            }
        })
        .1
        .chars()
        .fold((0, 0), |(a, b), c| {
            if c == '1' {
                (a + 1, b)
            } else if c == '2' {
                (a, b + 1)
            } else {
                (a, b)
            }
        });

    let value = a * b;

    value
}

fn part2(input: &str, width: i32, height: i32) {
    let mut layers: Vec<String> = Vec::new();
    let mut current_string: String = String::new();
    let mut current_index = 0;
    for i in input.chars() {
        current_string.push(i);
        current_index += 1;
        if current_index == width * height {
            layers.push(current_string);
            current_index = 0;
            current_string = String::new();
        }
    }

    let mut transposed: Vec<String> = Vec::new();
    transposed.resize(layers[0].len(), String::new());
    for l in layers {
        for (i, c) in (0..).zip(l.chars()) {
            transposed[i].push(c);
        }
    }

    let image: Vec<char> = transposed
        .iter()
        .map(|s| s.chars().fold('2', |c, v| if c == '2' { v } else { c }))
        .collect();

    for y in 0..height {
        for x in 0..width {
            print!("{}", image[(y * width + x) as usize])
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        panic!("Incorrect Usage");
    }

    let width: i32 = args.get(1).unwrap().parse().unwrap();
    let height: i32 = args.get(2).unwrap().parse().unwrap();

    let input = parse_file(args.get(3).unwrap());

    let p1 = part1(&input, width, height);
    println!("Part 1: {p1}");

    part2(&input, width, height);
}
