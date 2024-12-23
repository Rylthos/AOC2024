use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use circular_buffer::CircularBuffer;

fn parse_input(file_name: &str) -> Vec<u64> {
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

    let mut input: Vec<u64> = Vec::new();
    for line in s.split("\n") {
        if line.is_empty() {
            continue;
        }

        input.push(line.parse().unwrap());
    }

    input
}

fn evolve_secret(secret: u64, iterations: i32) -> u64 {
    if iterations == 0 {
        return secret;
    }
    let mul = secret << 6;
    let s2 = (secret ^ mul) % 16777216;
    let div = s2 >> 5;
    let s3 = (s2 ^ div) % 16777216;
    let mul2 = s3 << 11;
    let s4 = (s3 ^ mul2) % 16777216;

    return evolve_secret(s4, iterations - 1);
}

fn part1(inputs: &Vec<u64>) -> u64 {
    let mut total = 0;
    for i in inputs {
        let value = evolve_secret(*i, 2000);
        println!("{i} | {value}");
        total += value;
    }
    total
    // println!("{inputs:?}");
    // 0
}

fn track_changes(
    secret: u64,
    iterations: i32,
    memory: &mut HashSet<(i8, i8, i8, i8)>,
) -> HashMap<(i8, i8, i8, i8), u8> {
    let mut current = secret;
    let mut buf = CircularBuffer::<4, i8>::new();

    // let equal_elem = |i: usize, buf: &CircularBuffer<4, i8>| {
    //     buf.get(i).unwrap() == target_sequence.get(i).unwrap()
    // };

    let mut new_memory: HashSet<(i8, i8, i8, i8)> = HashSet::new();
    let mut cache: HashMap<(i8, i8, i8, i8), u8> = HashMap::new();

    for _ in 0..iterations {
        let mul = current << 6;
        let s2 = (current ^ mul) % 16777216;
        let div = s2 >> 5;
        let s3 = (s2 ^ div) % 16777216;
        let mul2 = s3 << 11;
        let s4 = (s3 ^ mul2) % 16777216;

        let diff = (((s4 % 10) as i8) - ((current % 10) as i8)) as i8;
        current = s4;

        buf.push_back(diff);
        if buf.len() == 4 {
            let pair = (
                *buf.get(0).unwrap(),
                *buf.get(1).unwrap(),
                *buf.get(2).unwrap(),
                *buf.get(3).unwrap(),
            );
            new_memory.insert(pair);

            if !cache.contains_key(&pair) {
                cache.insert(pair, (current % 10) as u8);
            }
        }
    }
    memory.extend(&new_memory);
    cache
}

fn part2(inputs: &Vec<u64>) -> i64 {
    // let mut current_sequence: Vec<i8> = vec![-9, -9, -9, -9];
    // let mut current_sequence: Vec<i8> = vec![-1, -1, 0, 2];
    let mut max = 0;

    let mut memory: HashSet<(i8, i8, i8, i8)> = HashSet::new();
    let mut all_maps: Vec<HashMap<(i8, i8, i8, i8), u8>> = Vec::new();
    for input in inputs {
        all_maps.push(track_changes(*input, 2000, &mut memory))
    }
    // println!("{}", memory.len());
    // println!("{all_maps:?}");

    let mut count = 0;
    let length = memory.len();
    for pair in memory {
        println!("Checking: {pair:?} | {} | {}/{}", max, count, length);

        let mut total = 0;
        for m in all_maps.iter() {
            if m.contains_key(&pair) {
                total += (*m.get(&pair).unwrap()) as i64;
            }
        }
        // for (i, m) in all_maps.iter() {
        //     if !m.contains(&(a, b, c, d)) {
        //         continue;
        //     }
        //
        //     for w in i.windows(4) {
        //         let (a1, _) = w[0];
        //         let (b1, _) = w[1];
        //         let (c1, _) = w[2];
        //         let (d1, v) = w[3];
        //         if a1 == a && b1 == b && c1 == c && d1 == d {
        //             total += v as i64;
        //             break;
        //         }
        //     }
        // }

        max = i64::max(total, max);
        count += 1;
    }

    max
}

fn main() {
    // let inputs = parse_input("Example.txt");
    // let inputs = parse_input("SecondExample.txt");
    let inputs = parse_input("Input.txt");
    // let inputs = parse_input("ThirdExample.txt");

    // let p1 = part1(&inputs);
    // println!("Part 1: {p1}");
    //
    let p2 = part2(&inputs);
    println!("Part 2: {p2}");
}
