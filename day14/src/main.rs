use regex::Regex;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::io::stdin;

use image::ExtendedColorType::*;
use image::*;

fn parse_input(file_name: &str) -> Vec<((i32, i32), (i32, i32))> {
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

    let mut values: Vec<((i32, i32), (i32, i32))> = Vec::new();

    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    for line in s.split("\n") {
        if line.is_empty() {
            continue;
        }

        for (_, [px, py, vx, vy]) in re.captures_iter(&line).map(|s| s.extract()) {
            let pxi: i32 = px.parse().unwrap();
            let pyi: i32 = py.parse().unwrap();
            let vxi: i32 = vx.parse().unwrap();
            let vyi: i32 = vy.parse().unwrap();
            values.push(((pxi, pyi), (vxi, vyi)));
        }
    }

    values
}

fn get_quadrant(width: i32, height: i32, pos: (i32, i32)) -> (i32, i32, i32, i32) {
    let (px, py) = pos;

    let left = px < (width / 2);
    let right = px >= ((width + 1) / 2);
    let top = py < (height / 2);
    let bottom = py >= ((height + 1) / 2);

    if left && top {
        return (1, 0, 0, 0);
    } else if left && bottom {
        return (0, 0, 1, 0);
    } else if right && top {
        return (0, 1, 0, 0);
    } else if right && bottom {
        return (0, 0, 0, 1);
    } else {
        return (0, 0, 0, 0);
    }
}

fn part1(width: i32, height: i32, input: &Vec<((i32, i32), (i32, i32))>) -> u64 {
    let mut quadrants = (0, 0, 0, 0);

    for robot in input {
        let ((px, py), (vx, vy)) = robot;
        let mut npx = (px + (vx * 100)) % width;
        let mut npy = (py + (vy * 100)) % height;

        if npx < 0 {
            npx = width + npx;
        }

        if npy < 0 {
            npy = height + npy;
        }

        let (tl, tr, bl, br) = get_quadrant(width, height, (npx, npy));
        let (stl, str, sbl, sbr) = quadrants;
        quadrants = (stl + tl, str + tr, sbl + bl, sbr + br);
    }

    let (tl, tr, bl, br) = quadrants;
    return (tl * tr * bl * br) as u64;
}

fn part2(
    width: i32,
    height: i32,
    input: &Vec<((i32, i32), (i32, i32))>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut total: i32 = 1;

    let mut output: Vec<u8> = Vec::new();
    loop {
        for _ in 0..1000 {
            output.clear();
            output.resize((width * height) as usize, 0);

            for robot in input.iter() {
                let ((px, py), (vx, vy)) = *robot;

                let mut npx = (px + (vx * total)) % width;
                let mut npy = (py + (vy * total)) % height;

                if npx < 0 {
                    npx = width + npx;
                }

                if npy < 0 {
                    npy = height + npy;
                }

                *output.get_mut((npy * width + npx) as usize).unwrap() = 255;
            }

            image::save_buffer(
                &Path::new(&format!("Output/image{}.png", total)),
                &output,
                width as u32,
                height as u32,
                L8,
            )?;

            println!("Current: {}", total);

            total += 1;
        }

        let mut user_input = String::new();
        stdin().read_line(&mut user_input).expect("Not a string");

        match user_input.chars().nth(0) {
            Some('\n') => continue,
            Some('q') => break,
            _ => break,
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let use_example = false;

    let (width, height, parsed) = if use_example {
        (11, 7, parse_input("Example.txt"))
    } else {
        (101, 103, parse_input("Input.txt"))
    };

    let p1 = part1(width, height, &parsed);
    println!("Part 1: {p1}");

    if !use_example {
        part2(width, height, &parsed)?
    }

    Ok(())
}
