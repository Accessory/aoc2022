#![feature(map_many_mut)]

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn run(input_file: &str) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut results: Vec<usize> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let trimmed_line = line.unwrap().trim().to_string();

        if trimmed_line.is_empty() {
            continue;
        }

        for i in 0..trimmed_line.len() - 4 {
            let set: HashSet<char> = HashSet::from_iter(trimmed_line[i..i + 4].chars());
            if set.len() == 4 {
                results.push(i + 4);
                break;
            }
        }
    }

    for result in results {
        println!("Results {}", result);
    }
}

fn run2(input_file: &str) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut results: Vec<usize> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let trimmed_line = line.unwrap().trim().to_string();

        if trimmed_line.is_empty() {
            continue;
        }

        for i in 0..trimmed_line.len() - 14 {
            let set: HashSet<char> = HashSet::from_iter(trimmed_line[i..i + 14].chars());
            if set.len() == 14 {
                results.push(i + 14);
                break;
            }
        }
    }

    for result in results {
        println!("Results {}", result);
    }
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
