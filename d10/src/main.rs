use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use utils::get_input_path;

enum ExecutionState {
    NeedInstruction,
    ExecuteInstruction,
}

fn run(input_file: &str) {
    // Init
    let check_results_at = HashSet::from([20, 60, 100, 140, 180, 220]);
    let max_cycles = 220;

    let file = File::open(input_file).unwrap();
    let mut reader = BufReader::new(file);

    let mut x = 1;
    let mut final_result = 0;
    let mut instruction = String::with_capacity(64);
    let mut state = ExecutionState::NeedInstruction;

    for c in 1..max_cycles + 1 {
        if check_results_at.contains(&c) {
            let result = c * x;
            println!(
                "Current Value at {} is {} what results to {}",
                &c, &x, &result
            );
            final_result += result;
        }
        match state {
            ExecutionState::NeedInstruction => {
                instruction.clear();
                reader.read_line(&mut instruction).unwrap();

                if !instruction.starts_with("noop") {
                    state = ExecutionState::ExecuteInstruction;
                }
            }
            ExecutionState::ExecuteInstruction => {
                let split: Vec<&str> = instruction.trim().split(" ").collect();
                let number_string = split.get(1).unwrap();
                let number: i64 = number_string.parse().unwrap();
                x += number;
                state = ExecutionState::NeedInstruction;
            }
        };
    }
    println!("The final result is {}", final_result);
}

fn run2(input_file: &str) {
    // Init
    const COLUMNS: usize = 40;
    const ROWS: usize = 6;
    const MAX_CYCLES: usize = COLUMNS * ROWS;
    let mut result_matrix = [0 as u8; MAX_CYCLES];

    let file = File::open(input_file).unwrap();
    let mut reader = BufReader::new(file);

    let mut instruction = String::with_capacity(64);
    let mut state = ExecutionState::NeedInstruction;
    let mut sprite_position: i64 = 0;

    // Execute
    for c in 0..MAX_CYCLES {
        let column = c % COLUMNS;

        if sprite_position == column as i64
            || (sprite_position + 1) == column as i64
            || (sprite_position + 2) == column as i64
        {
            result_matrix[c] = 1;
        } else {
            result_matrix[c] = 0;
        }

        match state {
            ExecutionState::NeedInstruction => {
                instruction.clear();
                reader.read_line(&mut instruction).unwrap();

                if !instruction.starts_with("noop") {
                    state = ExecutionState::ExecuteInstruction;
                }
            }
            ExecutionState::ExecuteInstruction => {
                let split: Vec<&str> = instruction.trim().split(" ").collect();
                let number_string = split.get(1).unwrap();
                let number: i64 = number_string.parse().unwrap();
                sprite_position += number;
                state = ExecutionState::NeedInstruction;
            }
        };
    }

    // Display
    for p in 0..MAX_CYCLES {
        let symbol = if result_matrix[p] == 0 { '.' } else { '#' };
        print!("{}", symbol);
        if (p + 1) % COLUMNS == 0 {
            println!();
        }
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
