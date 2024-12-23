use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use multimap::MultiMap;

fn parse_input(file_name: &str) -> (MultiMap<i32, i32>, HashMap<i32, String>) {
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

    let mut int_to_string: HashMap<i32, String> = HashMap::new();
    let mut string_to_int: HashMap<String, i32> = HashMap::new();
    let mut current_index = 0;
    let mut input: MultiMap<i32, i32> = MultiMap::new();
    for line in s.split("\n") {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split("-").collect();
        let v1 = parts[0];
        let v2 = parts[1];

        if !string_to_int.contains_key(v1) {
            string_to_int.insert(v1.to_string(), current_index);
            int_to_string.insert(current_index, v1.to_string());
            current_index += 1;
        }
        if !string_to_int.contains_key(v2) {
            string_to_int.insert(v2.to_string(), current_index);
            int_to_string.insert(current_index, v2.to_string());
            current_index += 1;
        }

        let i1 = string_to_int.get(v1).unwrap();
        let i2 = string_to_int.get(v2).unwrap();

        input.insert(*i1, *i2);
        input.insert(*i2, *i1);
    }

    (input, int_to_string)
}

fn part1(input: &MultiMap<i32, i32>, int_to_string: &HashMap<i32, String>) -> u64 {
    let mut count = 0;
    for (i, v1) in input.iter_all() {
        for (j, _) in input.iter() {
            if i == j {
                continue;
            }

            for v in v1 {
                let current_set: BTreeSet<i32> = BTreeSet::from([*i, *j, *v]);

                if !is_clique(&current_set, &input) {
                    continue;
                }

                if i < j && j < v {
                    let a = int_to_string.get(i).unwrap();
                    let b = int_to_string.get(j).unwrap();
                    let c = int_to_string.get(v).unwrap();
                    if a.chars().nth(0).unwrap() == 't'
                        || b.chars().nth(0).unwrap() == 't'
                        || c.chars().nth(0).unwrap() == 't'
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn is_clique(current_set: &BTreeSet<i32>, nodes: &MultiMap<i32, i32>) -> bool {
    for e1 in current_set.iter() {
        for e2 in current_set.iter() {
            if e1 >= e2 {
                continue;
            }

            if let Some(v) = nodes.get_vec(e1) {
                if !v.contains(e2) {
                    return false;
                }
            }
        }
    }

    return true;
}

fn part2(input: &MultiMap<i32, i32>, int_to_string: &HashMap<i32, String>) {
    let mut sizes: Vec<Vec<BTreeSet<i32>>> = Vec::new();
    let nodes: BTreeSet<i32> = BTreeSet::from_iter(input.iter().map(|(k, _)| k.clone()));
    sizes.push(Vec::from_iter(
        nodes.iter().map(|v| BTreeSet::from([v.clone()])),
    ));

    let mut current_index = 0;

    println!("size of clique 1 | {}", nodes.len());
    loop {
        let mut new_sets: Vec<BTreeSet<i32>> = Vec::new();
        let current_set = &sizes[current_index];
        let current_size = current_index + 2;
        for set in current_set.iter() {
            let first = set.iter().next().unwrap();
            let last = set.iter().last().unwrap();
            let neighbours = input.get_vec(&first).unwrap();

            for n in neighbours.iter() {
                if n <= last {
                    continue;
                }

                if !set.contains(n) {
                    let mut new_set = set.clone();
                    new_set.insert(*n);

                    if is_clique(&new_set, input) {
                        new_sets.push(new_set);
                    }
                }
            }
        }
        current_index += 1;
        if new_sets.len() == 0 {
            break;
        }
        println!("size of clique {current_size} | {}", new_sets.len());
        sizes.push(new_sets);
    }

    let last_set = sizes.get(current_index - 1).unwrap();
    let set = last_set.get(0).unwrap();
    let new_set = BTreeSet::from_iter(set.iter().map(|s| int_to_string.get(s).unwrap()));
    for s in new_set {
        print!("{s},");
    }
    println!();
}

fn main() {
    // let (inputs, int_to_string) = parse_input("Example.txt");
    let (inputs, int_to_string) = parse_input("Input.txt");

    let p1 = part1(&inputs, &int_to_string);
    println!("Part 1: {p1}");

    part2(&inputs, &int_to_string);
}
