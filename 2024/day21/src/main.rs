use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum State {
    State9,
    State8,
    State7,
    State6,
    State5,
    State4,
    State3,
    State2,
    State1,
    State0,
    StateUp,
    StateLeft,
    StateDown,
    StateRight,
    StateA,
}

#[derive(Debug, Clone, Copy, Eq)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl PartialEq<Vec2> for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Vec2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn char_to_state(c: char) -> State {
    match c {
        '0' => State::State0,
        '1' => State::State1,
        '2' => State::State2,
        '3' => State::State3,
        '4' => State::State4,
        '5' => State::State5,
        '6' => State::State6,
        '7' => State::State7,
        '8' => State::State8,
        '9' => State::State9,
        '^' => State::StateUp,
        'v' => State::StateDown,
        '<' => State::StateLeft,
        '>' => State::StateRight,
        'A' => State::StateA,
        _ => panic!("Unknown state"),
    }
}

fn parse_input(file_name: &str) -> Vec<(String, Vec<State>)> {
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

    let mut inputs: Vec<(String, Vec<State>)> = Vec::new();

    for line in s.split("\n") {
        if line.is_empty() {
            continue;
        }

        let mut input: Vec<State> = Vec::new();

        for c in line.chars() {
            input.push(char_to_state(c));
        }
        inputs.push((line.to_string(), input));
    }

    inputs
}

fn get_movement(
    current_state: State,
    target_state: State,
    mapping: &HashMap<State, Vec2>,
    invalid_pos: &Vec2,
) -> (bool, bool) {
    let mut move_left_first = true;
    let mut move_right_last = true;
    {
        let mut current_pos = *mapping.get(&current_state).unwrap();
        let target_pos = *mapping.get(&target_state).unwrap();

        while current_pos != target_pos {
            let left = i32::max(0, current_pos.x - target_pos.x);
            let right = i32::max(0, target_pos.x - current_pos.x);
            let up = i32::max(0, current_pos.y - target_pos.y);
            let down = i32::max(0, target_pos.y - current_pos.y);

            if left != 0 {
                current_pos = Vec2 {
                    x: current_pos.x - 1,
                    y: current_pos.y,
                }
            } else if up != 0 {
                current_pos = Vec2 {
                    x: current_pos.x,
                    y: current_pos.y - 1,
                }
            } else if down != 0 {
                current_pos = Vec2 {
                    x: current_pos.x,
                    y: current_pos.y + 1,
                }
            } else if right != 0 {
                current_pos = Vec2 {
                    x: current_pos.x + 1,
                    y: current_pos.y,
                }
            }

            if current_pos == *invalid_pos {
                if left != 0 {
                    move_left_first = false;
                } else {
                    if up != 0 || down != 0 {
                        move_right_last = false;
                    }
                }
                break;
            }
        }
    }
    return (move_left_first, move_right_last);
}

fn calculate_input(
    current_state: State,
    target_state: State,
    mapping: &HashMap<State, Vec2>,
    memory: &mut HashMap<(State, State, u64), u64>,
    depth: u64,
    max_depth: u64,
    invalid_bottom_left: bool,
) -> u64 {
    if depth >= max_depth {
        return 0;
    }

    if current_state == target_state {
        return 1;
    }

    let pair = (current_state, target_state, depth);
    if memory.contains_key(&pair) {
        return *memory.get(&pair).unwrap();
    }

    let invalid_pos = if invalid_bottom_left {
        Vec2 { x: 0, y: 3 }
    } else {
        Vec2 { x: 0, y: 0 }
    };

    let current_pos = *mapping.get(&current_state).unwrap();
    let target_pos = *mapping.get(&target_state).unwrap();

    let left = i32::max(0, current_pos.x - target_pos.x);
    let right = i32::max(0, target_pos.x - current_pos.x);
    let up = i32::max(0, current_pos.y - target_pos.y);
    let down = i32::max(0, target_pos.y - current_pos.y);

    let mut intermedary_state = State::StateA;

    let (move_left_first, move_right_last) =
        get_movement(current_state, target_state, mapping, &invalid_pos);

    let mut cost = 0;

    if move_left_first {
        for _ in 0..left {
            if depth + 1 == max_depth {
                cost += 1;
            }
            let values = calculate_input(
                intermedary_state,
                State::StateLeft,
                mapping,
                memory,
                depth + 1,
                max_depth,
                invalid_bottom_left,
            );
            cost += values;
            intermedary_state = State::StateLeft;
        }
    }
    if !move_right_last {
        for _ in 0..right {
            if depth + 1 == max_depth {
                cost += 1;
            }
            let values = calculate_input(
                intermedary_state,
                State::StateRight,
                mapping,
                memory,
                depth + 1,
                max_depth,
                invalid_bottom_left,
            );
            cost += values;
            intermedary_state = State::StateRight;
        }
    }
    for _ in 0..down {
        if depth + 1 == max_depth {
            cost += 1;
        }
        let values = calculate_input(
            intermedary_state,
            State::StateDown,
            mapping,
            memory,
            depth + 1,
            max_depth,
            invalid_bottom_left,
        );
        cost += values;
        intermedary_state = State::StateDown;
    }
    for _ in 0..up {
        if depth + 1 == max_depth {
            cost += 1;
        }
        let values = calculate_input(
            intermedary_state,
            State::StateUp,
            mapping,
            memory,
            depth + 1,
            max_depth,
            invalid_bottom_left,
        );
        cost += values;
        intermedary_state = State::StateUp;
    }
    if !move_left_first {
        for _ in 0..left {
            if depth + 1 == max_depth {
                cost += 1;
            }
            let values = calculate_input(
                intermedary_state,
                State::StateLeft,
                mapping,
                memory,
                depth + 1,
                max_depth,
                invalid_bottom_left,
            );
            cost += values;
            intermedary_state = State::StateLeft;
        }
    }
    if move_right_last {
        for _ in 0..right {
            if depth + 1 == max_depth {
                cost += 1;
            }
            let values = calculate_input(
                intermedary_state,
                State::StateRight,
                mapping,
                memory,
                depth + 1,
                max_depth,
                invalid_bottom_left,
            );
            cost += values;
            intermedary_state = State::StateRight;
        }
    }

    if depth + 1 == max_depth {
        cost += 1;
    } else {
        let values = calculate_input(
            intermedary_state,
            State::StateA,
            mapping,
            memory,
            depth + 1,
            max_depth,
            invalid_bottom_left,
        );
        cost += values;
    }

    memory.insert(pair, cost);

    return cost;
}

fn calculate_inputs(
    input: &Vec<State>,
    mapping: &HashMap<State, Vec2>,
    memory: &mut HashMap<(State, State, u64), u64>,
    depth: u64,
    max_depth: u64,
    invalid_bottom_left: bool,
) -> u64 {
    let mut current_state = State::StateA;
    let mut cost = 0;
    for i in input {
        let v = calculate_input(
            current_state,
            *i,
            mapping,
            memory,
            depth,
            max_depth,
            invalid_bottom_left,
        );

        cost += v;

        current_state = *i;
    }

    cost
}

fn puzzle(inputs: &Vec<(String, Vec<State>)>, robot_controlled_direction: u64) -> u64 {
    let keypad_input: HashMap<State, Vec2> = HashMap::from([
        (State::State7, Vec2 { x: 0, y: 0 }),
        (State::State8, Vec2 { x: 1, y: 0 }),
        (State::State9, Vec2 { x: 2, y: 0 }),
        (State::State4, Vec2 { x: 0, y: 1 }),
        (State::State5, Vec2 { x: 1, y: 1 }),
        (State::State6, Vec2 { x: 2, y: 1 }),
        (State::State1, Vec2 { x: 0, y: 2 }),
        (State::State2, Vec2 { x: 1, y: 2 }),
        (State::State3, Vec2 { x: 2, y: 2 }),
        (State::State0, Vec2 { x: 1, y: 3 }),
        (State::StateA, Vec2 { x: 2, y: 3 }),
    ]);

    let control_input: HashMap<State, Vec2> = HashMap::from([
        (State::StateUp, Vec2 { x: 1, y: 0 }),
        (State::StateA, Vec2 { x: 2, y: 0 }),
        (State::StateLeft, Vec2 { x: 0, y: 1 }),
        (State::StateDown, Vec2 { x: 1, y: 1 }),
        (State::StateRight, Vec2 { x: 2, y: 1 }),
    ]);

    let mut total: u64 = 0;
    for (original, input) in inputs {
        let mut memory: HashMap<(State, State, u64), u64> = HashMap::new();
        let mut movement: Vec<State> = Vec::new();
        let mut current_state = State::StateA;

        let invalid_pos = Vec2 { x: 0, y: 3 };
        for i in input {
            let (move_left_first, move_right_last) =
                get_movement(current_state, *i, &keypad_input, &invalid_pos);

            let current_pos = *keypad_input.get(&current_state).unwrap();
            let target_pos = *keypad_input.get(&i).unwrap();

            let left = i32::max(0, current_pos.x - target_pos.x);
            let right = i32::max(0, target_pos.x - current_pos.x);
            let up = i32::max(0, current_pos.y - target_pos.y);
            let down = i32::max(0, target_pos.y - current_pos.y);

            if move_left_first {
                for _ in 0..left {
                    movement.push(State::StateLeft);
                }
            }
            if !move_right_last {
                for _ in 0..right {
                    movement.push(State::StateRight);
                }
            }
            for _ in 0..down {
                movement.push(State::StateDown);
            }
            for _ in 0..up {
                movement.push(State::StateUp);
            }
            if !move_left_first {
                for _ in 0..left {
                    movement.push(State::StateLeft);
                }
            }
            if move_right_last {
                for _ in 0..right {
                    movement.push(State::StateRight);
                }
            }
            movement.push(State::StateA);

            current_state = *i;
        }

        let cost = calculate_inputs(
            &movement,
            &control_input,
            &mut memory,
            0,
            robot_controlled_direction,
            false,
        );

        let value = (original[..(original.len() - 1)]).parse::<u64>().unwrap();
        total += cost * value;
    }
    total
}

fn main() {
    // let inputs = parse_input("Example.txt");
    let inputs = parse_input("Input.txt");

    let p1 = puzzle(&inputs, 2);
    println!("Part 1: {p1}");

    let p2 = puzzle(&inputs, 25);
    println!("Part 2: {p2}");
}
