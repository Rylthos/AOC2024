use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::cmp;

#[derive(Debug, Clone)]
struct FileEntry {
    id: u32,
    size: u32,
    is_entry: bool,
    offset: u32,
}

fn parse_input(file_name: &str) -> Vec<FileEntry> {
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

    let mut file_entries: Vec<FileEntry> = Vec::new();

    let mut id: u32 = 0;
    let mut is_entry = true;
    for line in s.split("\n") {
        if line.is_empty() {
            continue;
        }

        for char in line.chars() {
            let value: u32 = char.to_digit(10).unwrap();
            let entry = if is_entry {
                id += 1;
                FileEntry {
                    id: id - 1,
                    size: value,
                    is_entry: true,
                    offset: 0,
                }
            } else {
                FileEntry {
                    id: 0,
                    size: value,
                    is_entry: false,
                    offset: 0,
                }
            };

            file_entries.push(entry);

            is_entry = !is_entry;
        }
    }

    file_entries
}

fn part1(file_entries: &mut Vec<FileEntry>) -> u64 {
    let mut first_ptr: usize = 0;
    let mut last_ptr: usize = file_entries.len() - 1;

    let mut final_positions: Vec<u32> = Vec::new();

    while first_ptr <= last_ptr {
        if file_entries.get(first_ptr).unwrap().is_entry
            || file_entries.get(first_ptr).unwrap().size == 0
        {
            for _ in 0..file_entries.get(first_ptr).unwrap().size {
                final_positions.push(file_entries.get(first_ptr).unwrap().id);
            }
            first_ptr += 1;
            continue;
        }

        if !file_entries.get(last_ptr).unwrap().is_entry
            || file_entries.get(last_ptr).unwrap().size == 0
        {
            last_ptr -= 1;
            continue;
        }

        let entries = cmp::min(
            file_entries.get(first_ptr).unwrap().size,
            file_entries.get(last_ptr).unwrap().size,
        );

        for _ in 0..entries {
            final_positions.push(file_entries.get(last_ptr).unwrap().id);
        }

        if file_entries.get(first_ptr).unwrap().size == entries {
            first_ptr += 1;
            file_entries.get_mut(last_ptr).unwrap().size -= entries;
        } else {
            last_ptr -= 1;
            file_entries.get_mut(first_ptr).unwrap().size -= entries;
        }
    }

    let mut sum: u64 = 0;
    for (id, v) in (0_u32..).zip(final_positions) {
        sum += (id * v) as u64;
    }

    sum
}

fn part2(file_entries: &mut Vec<FileEntry>) -> u64 {
    let mut current_offset = 0;
    for entry in file_entries.iter_mut() {
        entry.offset = current_offset;
        current_offset += entry.size;
    }

    let mut final_positions: Vec<u32> = vec![0; current_offset as usize];

    for last_ptr in (0..file_entries.len()).rev() {
        let last = file_entries.get(last_ptr).unwrap().clone();

        if last.size == 0 || !last.is_entry {
            continue;
        }

        for first_ptr in 0..last_ptr {
            let first = file_entries.get(first_ptr).unwrap().clone();

            if first.is_entry || first.size == 0 {
                continue;
            }
            if first.size < last.size {
                continue;
            }

            for i in first.offset..(first.offset + last.size) {
                final_positions[i as usize] = last.id;
            }

            file_entries.get_mut(first_ptr).unwrap().size -= last.size;
            file_entries.get_mut(first_ptr).unwrap().offset += last.size;
            file_entries.get_mut(last_ptr).unwrap().size = 0;
            break;
        }
    }

    for entry in file_entries {
        if entry.is_entry && entry.size > 0 {
            for i in entry.offset..(entry.offset + entry.size) {
                final_positions[i as usize] = entry.id;
            }
        }
    }

    let mut sum: u64 = 0;
    for (id, v) in (0_u32..).zip(final_positions) {
        sum += (id * v) as u64;
    }

    sum
}

fn main() {
    // let mut parsed = parse_input("SmallExample.txt");
    // let mut parsed = parse_input("Example.txt");
    let mut parsed = parse_input("Input.txt");

    let mut copy = parsed.clone();
    let p1 = part1(&mut copy);
    let p2 = part2(&mut parsed);
    // let parsed = parse_input("Example.txt");

    // println!("{:?}", parsed);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    // println!("Hello, world!");
}
