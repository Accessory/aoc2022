#![feature(map_many_mut)]

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;
use utils::get_input_path;

enum ProgressState {
    FirstLine,
    CrateLine,
    RownumberLine,
    MoveLine,
}

fn process_line(
    crane_map: &mut HashMap<usize, VecDeque<char>>,
    line: &str,
    cranes_count: usize,
) -> ProgressState {
    for r in 0..cranes_count {
        let line_part = &line[r * 4..r * 4 + 3];
        if !&line_part.trim().is_empty() {
            if line_part.chars().nth(0).unwrap() == '[' {
                let item = line_part.chars().nth(1).unwrap();
                crane_map.get_mut(&(r + 1)).unwrap().push_back(item);
            } else {
                return ProgressState::RownumberLine;
            }
        }
    }

    return ProgressState::CrateLine;
}

fn run(input_file: &str) {
    let mut state = ProgressState::FirstLine;

    let mut cranes_count = 0;
    let mut crane_map: HashMap<usize, VecDeque<char>> = HashMap::new();

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        match state {
            ProgressState::FirstLine => {
                cranes_count = (line.len() + 1) / 4;

                for i in 0..cranes_count {
                    crane_map.insert(i + 1, VecDeque::new());
                }

                state = process_line(&mut crane_map, &line, cranes_count);
            }
            ProgressState::CrateLine => {
                state = process_line(&mut crane_map, &line, cranes_count);
            }
            ProgressState::RownumberLine => {
                state = ProgressState::MoveLine;
            }
            ProgressState::MoveLine => {
                let rgx = Regex::new(r#"move (\d\d?) from (\d\d?) to (\d\d?)"#).unwrap();
                let captures = rgx.captures(&line).unwrap();
                let move_amount_str = &captures[1];
                let from_str = &captures[2];
                let to_str = &captures[3];
                let move_amount: usize = move_amount_str.parse().unwrap();
                let from: usize = from_str.parse().unwrap();
                let to: usize = to_str.parse().unwrap();
                let [to_crane, from_crane] = crane_map.get_many_mut([&to, &from]).unwrap();

                for _t in 0..move_amount {
                    to_crane.push_front(from_crane.pop_front().unwrap())
                }
            }
        }
    }

    let mut log_line: String = String::new();
    for r in 0..cranes_count {
        log_line.push(*crane_map.get(&(r + 1)).unwrap().front().unwrap());
    }

    println!("The final Line is: {:?}", log_line);
}

fn run2(input_file: &str) {
    let mut state = ProgressState::FirstLine;

    let mut cranes_count = 0;
    let mut crane_map: HashMap<usize, VecDeque<char>> = HashMap::new();

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        match state {
            ProgressState::FirstLine => {
                cranes_count = (line.len() + 1) / 4;

                for i in 0..cranes_count {
                    crane_map.insert(i + 1, VecDeque::new());
                }

                state = process_line(&mut crane_map, &line, cranes_count);
            }
            ProgressState::CrateLine => {
                state = process_line(&mut crane_map, &line, cranes_count);
            }
            ProgressState::RownumberLine => {
                state = ProgressState::MoveLine;
            }
            ProgressState::MoveLine => {
                let rgx = Regex::new(r#"move (\d\d?) from (\d\d?) to (\d\d?)"#).unwrap();
                let captures = rgx.captures(&line).unwrap();
                let move_amount_str = &captures[1];
                let from_str = &captures[2];
                let to_str = &captures[3];
                let move_amount: usize = move_amount_str.parse().unwrap();
                let from: usize = from_str.parse().unwrap();
                let to: usize = to_str.parse().unwrap();
                let [to_crane, from_crane] = crane_map.get_many_mut([&to, &from]).unwrap();

                let mut tmp_stack: VecDeque<char> = VecDeque::new();
                for _t in 0..move_amount {
                    tmp_stack.push_back(from_crane.pop_front().unwrap());
                }
                for _t in 0..move_amount {
                    to_crane.push_front(tmp_stack.pop_back().unwrap());
                }
            }
        }
    }

    let mut log_line: String = String::new();
    for r in 0..cranes_count {
        log_line.push(*crane_map.get(&(r + 1)).unwrap().front().unwrap());
    }

    println!("The final Line is: {:?}", log_line);
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

    run(&input_file);
    run2(&input_file);
}

#[cfg(test)]
mod main_test {
    use utils::get_test_input_path;

    use crate::run;
    use crate::run2;

    #[test]
    fn test_input_part_1() {
        let input_path = get_test_input_path(file!());
        run(input_path.to_str().unwrap());
    }

    #[test]
    fn test_input_part_2() {
        let input_path = get_test_input_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
