use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
struct Object {
    name: String,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

#[derive(Debug)]
struct Orbit {
    pub objects: Vec<Object>,
    pub lookup: BTreeMap<String, usize>,
}

fn parse_file(filename: &str) -> Orbit {
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

    let items: Vec<Vec<&str>> = s
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.split(")").collect())
        .collect();

    let mut orbit: Orbit = Orbit {
        objects: Vec::new(),
        lookup: BTreeMap::new(),
    };

    for item in items {
        let parent = item[0];
        let child = item[1];

        if !orbit.lookup.contains_key(parent) {
            let parent_obj = Object {
                name: parent.to_string(),
                parent: None,
                children: Vec::new(),
            };

            let index = orbit.objects.len();
            println!("Adding Parent: {parent}: {index}");
            orbit.objects.push(parent_obj);
            orbit.lookup.insert(parent.to_string(), index);
        }

        let parent_index = orbit.lookup.get(parent).unwrap();

        let child_obj = Object {
            name: child.to_string(),
            parent: Some(*parent_index),
            children: Vec::new(),
        };

        let mut index = orbit.objects.len();
        if let Some(x) = orbit.lookup.get(child) {
            index = *x;
        } else {
            println!("Adding Child: {child}: {index}");
            orbit.objects.push(child_obj);
            orbit.lookup.insert(child.to_string(), index);
        }

        if let Some(id) = orbit.lookup.get(parent) {
            {
                let p = orbit.objects.get_mut(*id).unwrap();
                p.children.push(index);
                println!("Adding Child -> parent | {}: {}", p.name, child);
            }

            {
                let child = orbit.objects.get_mut(index).unwrap();
                child.parent = Some(*id);
            }
        }
    }

    orbit
}

fn count_orbits(obj: &Object, orbit: &Orbit, depth: u64) -> u64 {
    let mut count = depth;

    for child in obj.children.iter() {
        let child_obj = &orbit.objects[*child];
        count += count_orbits(child_obj, orbit, depth + 1);
    }

    count
}

fn part1(orbit: &Orbit) -> u64 {
    let parent = &orbit.objects[*orbit.lookup.get("COM").unwrap()];
    return count_orbits(&parent, orbit, 0);
}

fn get_parent_id(name: &str, orbit: &Orbit) -> usize {
    let obj = orbit.objects.get(*orbit.lookup.get(name).unwrap()).unwrap();
    if let Some(x) = obj.parent {
        x
    } else {
        panic!("No parent");
    }
}

fn can_reach(current_id: usize, target_id: usize, orbit: &Orbit) -> Result<usize, ()> {
    let obj = orbit.objects.get(current_id).unwrap();
    if current_id == target_id {
        return Ok(0);
    }

    for child in obj.children.iter() {
        let result = can_reach(*child, target_id, orbit);
        if let Ok(x) = result {
            return Ok(x + 1);
        }
    }

    Err(())
}

fn part2(orbit: &Orbit) -> usize {
    let parent_you = get_parent_id("YOU", orbit);
    let parent_san = get_parent_id("SAN", orbit);

    let mut current_parent = parent_you;
    let mut distance = 0;
    loop {
        let reachable = can_reach(current_parent, parent_san, orbit);
        if let Ok(x) = reachable {
            return distance + x;
        } else {
            distance += 1;
            current_parent = get_parent_id(&orbit.objects.get(current_parent).unwrap().name, orbit);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Incorrect Usage");
    }

    let input = parse_file(args.get(1).unwrap());

    let p1 = part1(&input);
    println!("Part 1: {p1}");

    let p2 = part2(&input);
    println!("Part 2: {p2}");
}
