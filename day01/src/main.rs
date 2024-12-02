use std::cmp::Reverse;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::collections::BinaryHeap;
use std::collections::HashMap;

fn parse_input(file_name: &str) -> (BinaryHeap<Reverse<i32>>, BinaryHeap<Reverse<i32>>) {
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(err) => panic!("Couldn't open {}: {}", display, err),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(err) => panic!("Couldn't read {}: {}", display, err),
        Ok(_) => (),
    }

    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();

    for line in s.split("\n") {
        let numbers: Vec<i32> = line
            .split("   ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let left_num = numbers.get(0);
        let right_num = numbers.get(1);
        match left_num {
            Some(x) => left_heap.push(Reverse(*x)),
            None => (),
        };

        match right_num {
            Some(x) => right_heap.push(Reverse(*x)),
            None => (),
        };
    }

    return (left_heap, right_heap);
}

fn part1(mut left: BinaryHeap<Reverse<i32>>, mut right: BinaryHeap<Reverse<i32>>) -> i64 {
    let mut sum: i64 = 0;

    while left.len() > 0 {
        let left_min = left.pop().unwrap_or(Reverse(0));
        let right_min = right.pop().unwrap_or(Reverse(0));

        sum += i64::from((left_min.0 - right_min.0).abs());
    }

    sum
}

fn part2(mut left: BinaryHeap<Reverse<i32>>, mut right: BinaryHeap<Reverse<i32>>) -> i64 {
    let mut sum: i64 = 0;

    let mut right_count: HashMap<i32, i32> = HashMap::new();
    while right.len() > 0 {
        let right_min = right.pop().unwrap_or(Reverse(0));
        let current = right_count.get(&right_min.0);

        match current {
            Some(x) => right_count.insert(right_min.0, x + 1),
            None => right_count.insert(right_min.0, 1),
        };
    }

    while left.len() > 0 {
        let left_min = left.pop().unwrap_or(Reverse(0));
        let count = right_count.get(&left_min.0);
        let multiplier = match count {
            Some(x) => *x,
            None => 0,
        };

        sum += i64::from(left_min.0 * multiplier);
    }

    sum
}

fn main() {
    let (left_heap, right_heap) = parse_input("Example.txt");
    // let (left_heap, right_heap) = parse_input("Input.txt");

    println!("Part 1: {}", part1(left_heap.clone(), right_heap.clone()));
    println!("Part 2: {}", part2(left_heap.clone(), right_heap.clone()));
}
