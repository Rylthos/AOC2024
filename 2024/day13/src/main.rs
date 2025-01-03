use regex::Regex;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

type PuzzleInput = ((i64, i64), (i64, i64), (i64, i64));

fn parse_input(file_name: &str) -> Vec<PuzzleInput> {
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

    let mut values: Vec<PuzzleInput> = Vec::new();

    let re = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();

    let mut values_raw: Vec<(i64, i64)> = Vec::new();

    for line in s.split("\n") {
        if line.is_empty() {
            continue;
        }

        for (_, [v1, v2]) in re.captures_iter(&line).map(|s| s.extract()) {
            let v1i: i64 = v1.parse().unwrap();
            let v2i: i64 = v2.parse().unwrap();
            values_raw.push((v1i, v2i));
        }
    }

    let mut temp: Vec<(i64, i64)> = Vec::new();
    for i in 0..values_raw.len() {
        if i % 3 == 0 && i != 0 {
            let v1 = *temp.get(0).unwrap();
            let v2 = *temp.get(1).unwrap();
            let v3 = *temp.get(2).unwrap();
            values.push((v1, v2, v3));
            temp.clear();
        }
        temp.push(*values_raw.get(i).unwrap());
    }
    {
        let v1 = *temp.get(0).unwrap();
        let v2 = *temp.get(1).unwrap();
        let v3 = *temp.get(2).unwrap();
        values.push((v1, v2, v3));
    }

    values
}

fn puzzle(input: &Vec<PuzzleInput>, offset: bool) -> u64 {
    let mut sum: u64 = 0;
    for ((ax, ay), (bx, by), (tx, ty)) in input {
        let offset_amount: i64 = 10000000000000;

        let (tx, ty) = if offset {
            (*tx + offset_amount, *ty + offset_amount)
        } else {
            (*tx, *ty)
        };

        let inv_scale: f64 = (ax * by - bx * ay) as f64;

        // let (tx, ty): (u128, u128) = ((*tx).into(), (*ty).into());

        // scale * (tx * by - ty * bx) as f64,
        // scale * (ty * ax - tx * ay) as f64,

        let scale: f64 = 1. / inv_scale;
        let (a, b) = (
            scale * (tx * by - ty * bx) as f64,
            scale * (ty * ax - tx * ay) as f64,
            // scale * calc_disc(*tx, *ty, *bx, *by),
            // scale * calc_disc(*ty, *tx, *ay, *ax),
        );

        if a < 0. || b < 0. {
            continue;
        }

        let eps = 0.01;
        let ar = a.round();
        let br = b.round();

        if (ar - a).abs() >= eps || (br - b).abs() >= eps {
            continue;
        }

        let ai: u64 = ar as u64;
        let bi: u64 = br as u64;

        sum += ai * 3 + bi;
    }

    sum
}

fn main() {
    // let parsed = parse_input("Example.txt");
    let parsed = parse_input("Input.txt");

    let p1 = puzzle(&parsed, false);
    println!("Part 1: {p1}");

    let p2 = puzzle(&parsed, true);
    println!("Part 2: {p2}");
}
