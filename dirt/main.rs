use std::cmp::max;
use std::fs::File;
use std::path::Path;
use std::{io::{BufRead, BufReader}};

fn run(input_file: &str) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut max_calories = 0;
    let mut current_count = 0;

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line.is_empty() {
            max_calories = max(max_calories, current_count)
        } else {
            let number = line.parse::<usize>().unwrap();
            current_count = current_count + number;
        }
    }

    println!("The Elf with the most Calories has  {} Calories", max_calories);
}

fn main() {
    Path::new(file!())
    let input_file = "input/input.txt";

    println!("{:?}", Path::new(".").canonicalize().unwrap());

    run(input_file);
}

#[cfg(test)]
mod main_test {
    use crate::run;

    #[test]
    fn test_input() {
        run("input/input_test.txt");
    }
}
