use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn run(input_file: &str) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut max_calories = 0;
    let mut current_count = 0;

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line.is_empty() {
            max_calories = max(max_calories, current_count);
            current_count = 0;
        } else {
            let number = line.parse::<usize>().unwrap();
            current_count = current_count + number;
        }
    }

    println!(
        "The Elf with the most Calories has {} Calories",
        max_calories
    );
}

fn run2(input_file: &str) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut first_calories:usize = 0;
    let mut second_calories:usize = 0;
    let mut thrid_calories:usize = 0;
    let mut current_count:usize = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line.is_empty() {
            if first_calories < current_count {
                thrid_calories = second_calories;
                second_calories = first_calories;
                first_calories = current_count;
                current_count = 0;
            } else if second_calories < current_count {
                thrid_calories = second_calories;
                second_calories = current_count;
                current_count = 0;
            } else if thrid_calories < current_count {
                thrid_calories = current_count;
                current_count = 0;
            } else {
                current_count = 0;
            }
        } else {
            current_count = current_count + line.parse::<usize>().unwrap();
        }
    }

    let result = first_calories + second_calories + thrid_calories;

    println!("The three Elfs have {} Calories", result);
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

    println!("{:?}", input_file);

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
