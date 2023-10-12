#![feature(linked_list_cursors)]
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

use utils::get_input_path;

#[derive(PartialEq)]
struct NumberNode {
    pub number: i64,
    pub original_position: usize,
}

impl std::fmt::Display for NumberNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

impl std::fmt::Debug for NumberNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut numbers = Vec::new();
    let mut numbers_in_order = Vec::new();
    let mut null_node = Rc::new(NumberNode {
        number: 0,
        original_position: 0,
    });

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        let number = line.parse::<i64>().unwrap();
        let number_rc = Rc::new(NumberNode {
            number,
            original_position: index,
        });
        numbers.push(number_rc.clone());
        numbers_in_order.push(number_rc.clone());

        if number_rc.number == 0 {
            null_node = number_rc.clone()
        }
    }

    // Solve
    let max_length = numbers.len();
    // let mut break_point = 4;
    // println!("{:?}", numbers);
    for current_number in numbers_in_order {
        if current_number.number == 0 {
            continue;
        }

        let mut number_idx = numbers.iter().position(|i| *i == current_number).unwrap();
        if current_number.number > 0 {
            for _ in 0..current_number.number {
                let current_idx = number_idx;
                number_idx = (number_idx + 1) % max_length;
                numbers.swap(current_idx, number_idx);
            }
        } else {
            let abs_number = current_number.number.abs();
            for _ in 0..abs_number {
                let current_idx = number_idx;
                if number_idx == 0 {
                    number_idx = max_length - 1;
                } else {
                    number_idx -= 1;
                }
                numbers.swap(current_idx, number_idx);
            }
        }

        // println!("{:?}", numbers);
        // break_point -= 1;
        // if break_point == 0 {break;}
    }

    // Result
    let number_idx = numbers.iter().position(|i| *i == null_node).unwrap();
    let r1000_idx = (number_idx + 1000) % max_length;
    let r2000_idx = (number_idx + 2000) % max_length;
    let r3000_idx = (number_idx + 3000) % max_length;
    let r1000 = numbers.get(r1000_idx).unwrap();
    let r2000 = numbers.get(r2000_idx).unwrap();
    let r3000 = numbers.get(r3000_idx).unwrap();

    let result = r1000.number + r2000.number + r3000.number;

    println!(
        "r1000 is {} r2000 is {} r3000 is {} the final result is {}",
        r1000, r2000, r3000, result
    );
}

fn run2(input_file: &str) {
    // Preamble
    let mut numbers = Vec::new();
    let mut numbers_in_order = Vec::new();
    let mut null_node = Rc::new(NumberNode {
        number: 0,
        original_position: 0,
    });
    const KEY: i64 = 811589153;
    const TIMES: usize = 10;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        let number = line.parse::<i64>().unwrap() * KEY;
        let number_rc = Rc::new(NumberNode {
            number,
            original_position: index,
        });
        numbers.push(number_rc.clone());
        numbers_in_order.push(number_rc.clone());

        if number_rc.number == 0 {
            null_node = number_rc.clone()
        }
    }

    // Solve
    let max_length = numbers.len();
    let modulo_length = max_length - 1;
    for _ in 0..TIMES {
        for current_number_rf in &numbers_in_order {
            let current_number = current_number_rf.clone();
            if current_number.number == 0 {
                continue;
            }

            let mut number_idx = numbers.iter().position(|i| *i == current_number).unwrap();
            if current_number.number > 0 {
                let numbers_to_move = current_number.number % modulo_length as i64;
                for _ in 0..numbers_to_move {
                    let current_idx = number_idx;
                    number_idx = (number_idx + 1) % max_length;
                    numbers.swap(current_idx, number_idx);
                }
            } else {
                let abs_number = current_number.number.abs();
                let numbers_to_move = abs_number % modulo_length as i64;
                for _ in 0..numbers_to_move {
                    let current_idx = number_idx;
                    if number_idx == 0 {
                        number_idx = max_length - 1;
                    } else {
                        number_idx -= 1;
                    }
                    numbers.swap(current_idx, number_idx);
                }
            }
        }
        
        // println!("{:?}", numbers);
    }
    // Result
    let number_idx = numbers.iter().position(|i| *i == null_node).unwrap();
    let r1000_idx = (number_idx + 1000) % max_length;
    let r2000_idx = (number_idx + 2000) % max_length;
    let r3000_idx = (number_idx + 3000) % max_length;
    let r1000 = numbers.get(r1000_idx).unwrap();
    let r2000 = numbers.get(r2000_idx).unwrap();
    let r3000 = numbers.get(r3000_idx).unwrap();

    let result = r1000.number + r2000.number + r3000.number;

    println!(
        "r1000 is {} r2000 is {} r3000 is {} the final result is {}",
        r1000, r2000, r3000, result
    );
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
