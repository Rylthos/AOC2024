use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn parse_input(file_name: &str) -> Vec<String> {
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

    let mut results: Vec<String> = Vec::new();

    for line in s.split("\n") {
        if line.len() > 0 {
            results.push(line.to_string());
        }
    }

    return results;
}

fn check_input(
    input: &Vec<String>,
    x: i32,
    y: i32,
    check_char: char,
    dir_x: i32,
    dir_y: i32,
) -> u64 {
    if input
        .get(y as usize)
        .unwrap()
        .chars()
        .nth(x as usize)
        .unwrap()
        != check_char
    {
        return 0;
    }

    if check_char == 'S' {
        return 1;
    }

    let next_char = match check_char {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => ' ',
    };

    if dir_x < -1 && dir_y < -1 {
        let mut sum = 0;
        for xd in -1..2 {
            for yd in -1..2 {
                if xd == yd && xd == 0 {
                    continue;
                }
                let new_x = x + xd;
                let new_y = y + yd;

                if new_x < 0 || new_y < 0 {
                    continue;
                }

                if (new_y as usize) < input.len()
                    && (new_x as usize) < input.get(new_y as usize).unwrap().len()
                {
                    sum += check_input(input, new_x, new_y, next_char, xd, yd);
                }
            }
        }

        return sum;
    } else {
        let new_x = x + dir_x;
        let new_y = y + dir_y;

        if new_x < 0 || new_y < 0 {
            return 0;
        }

        if (new_y as usize) < input.len()
            && (new_x as usize) < input.get(new_y as usize).unwrap().len()
        {
            return check_input(input, new_x, new_y, next_char, dir_x, dir_y);
        } else {
            return 0;
        }
    }
}

fn part1(input: &Vec<String>) -> u64 {
    let mut sum: u64 = 0;

    for y in 0..input.len() {
        for x in 0..input.get(y as usize).unwrap().len() {
            sum += check_input(&input, x as i32, y as i32, 'X', -2, -2);
        }
    }

    return sum;
}

fn part2(input: Vec<String>) -> u64 {
    let mut sum: u64 = 0;

    let get_index = |x: usize, y: usize| -> char { input.get(y).unwrap().chars().nth(x).unwrap() };

    for y in 1..(input.len() - 1) {
        for x in 1..(input.get(y as usize).unwrap().len() - 1) {
            if get_index(x, y) != 'A' {
                continue;
            }

            let top_left = get_index(x - 1, y - 1);
            let top_right = get_index(x + 1, y - 1);
            let bottom_left = get_index(x - 1, y + 1);
            let bottom_right = get_index(x + 1, y + 1);

            if (top_left == 'M' && bottom_right == 'S') || (top_left == 'S' && bottom_right == 'M')
            {
                if (top_right == 'M' && bottom_left == 'S')
                    || (top_right == 'S' && bottom_left == 'M')
                {
                    sum += 1;
                }
            }
        }
    }

    return sum;
}

fn main() {
    // let input = parse_input("Example.txt");
    let input = parse_input("Input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(input.clone()));
}
