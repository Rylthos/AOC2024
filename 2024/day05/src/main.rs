use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use multimap::MultiMap;

fn parse_input(file_name: &str) -> (MultiMap<u32, u32>, Vec<Vec<u32>>) {
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::open(path) {
        Err(err) => panic!("Couldn't open {}: {}", display, err),
        Ok(file) => file,
    };

    let mut s = String::new();

    if !file.read_to_string(&mut s).is_ok() {
        panic!("Couldn't read {}", file_name);
    }

    let mut rules = MultiMap::new();
    let mut pages: Vec<Vec<u32>> = Vec::new();

    for line in s.split("\n") {
        if line.len() == 5 {
            // Rule
            let values: Vec<u32> = line.split("|").map(|s| s.parse().unwrap()).collect();
            rules.insert(values[0], values[1]);
        } else {
            let numbers: Vec<u32> = line
                .split(",")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();

            if !numbers.is_empty() {
                pages.push(numbers);
            }
        }
    }

    (rules, pages)
}

fn is_valid(
    input: &Vec<u32>,
    rules: &MultiMap<u32, u32>,
    fix: &mut Option<Vec<(usize, usize)>>,
) -> bool {
    let mut valid = true;
    for i in 1..input.len() {
        let current_value = input[i];
        let bound_rules: &Vec<u32>;
        if let Some(a) = rules.get_vec(&current_value) {
            bound_rules = a;
        } else {
            continue;
        }

        for j in 0..i {
            let check_value = input[j];

            for v in bound_rules {
                if check_value == *v {
                    if let Some(values) = fix {
                        valid = false;
                        values.push((i, j));
                    } else {
                        return false;
                    }
                }
            }
        }
    }

    return valid;
}

fn part1(rules: &MultiMap<u32, u32>, pages: &Vec<Vec<u32>>) -> u32 {
    let mut sum: u32 = 0;
    for page in pages {
        let mut none_option = None;
        if is_valid(&page, rules, &mut none_option) {
            let mid = (page.len() - 1) / 2;
            sum += page[mid];
        }
    }

    sum
}

fn part2(rules: &MultiMap<u32, u32>, pages: &Vec<Vec<u32>>) -> u32 {
    let mut sum: u32 = 0;
    for page in pages {
        let mut fix: Option<Vec<(usize, usize)>> = Some(Vec::new());
        if !is_valid(&page, rules, &mut fix) {
            let mut page_entry = page.clone();

            let first_fix = fix.as_ref().unwrap()[0];
            let (i1, i2) = first_fix;

            let t = page_entry[i1];
            page_entry[i1] = page_entry[i2];
            page_entry[i2] = t;

            fix = Some(Vec::new());

            while !is_valid(&page_entry, rules, &mut fix) {
                let first_fix = fix.as_ref().unwrap()[0];
                let (i1, i2) = first_fix;

                let t = page_entry[i1];
                page_entry[i1] = page_entry[i2];
                page_entry[i2] = t;

                fix = Some(Vec::new());
            }

            let mid = (page_entry.len() - 1) / 2;
            sum += page_entry[mid];
        }
    }

    sum
}

fn main() {
    // let (rules, pages) = parse_input("Example.txt");
    let (rules, pages) = parse_input("Input.txt");

    // println!("{:?}", input);
    // let input = parse_input("Input.txt");

    println!("Part 1: {}", part1(&rules, &pages));
    println!("Part 2: {}", part2(&rules, &pages));
}
