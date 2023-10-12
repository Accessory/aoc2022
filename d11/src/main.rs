use eval::eval;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[allow(dead_code)]
#[derive(Debug)]
struct Monkey {
    pub id: usize,
    pub items: VecDeque<u64>,
    pub operation: String,
    pub test_divisible: u64,
    pub test_false_to: usize,
    pub test_true_to: usize,
    pub inspects: u64,
}

fn run(input_file: &str) {
    // Preamble
    const ROUNDS: usize = 20;
    let mut monkeys: Vec<Monkey> = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line_result) = lines.next() {
        let mut line = line_result.unwrap().trim().to_string();
        let id: usize = line[7..8].parse().unwrap();
        line = lines.next().unwrap().unwrap().trim().to_string();
        let splits = line[16..].split(", ");
        let mut items: VecDeque<u64> = VecDeque::new();
        for item in splits {
            items.push_back(item.parse().unwrap());
        }
        let operation = lines.next().unwrap().unwrap().trim().to_string()[17..].to_string();
        let test_divisible: u64 = lines.next().unwrap().unwrap().trim().to_string()[19..]
            .parse()
            .unwrap();
        let test_true_to: usize = lines.next().unwrap().unwrap().trim().to_string()[25..]
            .parse()
            .unwrap();
        let test_false_to: usize = lines.next().unwrap().unwrap().trim().to_string()[26..]
            .parse()
            .unwrap();

        let monkey = Monkey {
            id,
            items,
            operation,
            test_divisible,
            test_false_to,
            test_true_to,
            inspects: 0,
        };

        monkeys.push(monkey);
        lines.next();
    }

    // Solve
    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            // for monkey in monkeys.iter_mut() {
            while let Some(item) = monkeys.get_mut(i).unwrap().items.pop_front() {
                monkeys.get_mut(i).unwrap().inspects += 1;
                let division_test = monkeys
                    .get(i)
                    .unwrap()
                    .operation
                    .replace("old", &item.to_string());
                let test_result = eval(&division_test).unwrap().as_u64().unwrap() / 3;
                if test_result % monkeys.get(i).unwrap().test_divisible == 0 {
                    let test_true_to = monkeys.get(i).unwrap().test_true_to;
                    monkeys
                        .get_mut(test_true_to)
                        .unwrap()
                        .items
                        .push_back(test_result);
                } else {
                    let test_false_to = monkeys.get(i).unwrap().test_false_to;
                    monkeys
                        .get_mut(test_false_to)
                        .unwrap()
                        .items
                        .push_back(test_result);
                }
            }
        }
    }

    // Result
    monkeys.sort_by_key(|k| k.inspects);

    let result = monkeys.get(monkeys.len() - 1).unwrap().inspects
        * monkeys.get(monkeys.len() - 2).unwrap().inspects;

    println!("{:?}", &monkeys);

    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    const ROUNDS: usize = 10000;
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut comman_divisor = 1;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line_result) = lines.next() {
        let mut line = line_result.unwrap().trim().to_string();
        let id: usize = line[7..8].parse().unwrap();
        line = lines.next().unwrap().unwrap().trim().to_string();
        let splits = line[16..].split(", ");
        let mut items: VecDeque<u64> = VecDeque::new();
        for item in splits {
            items.push_back(item.parse().unwrap());
        }
        let operation = lines.next().unwrap().unwrap().trim().to_string()[17..].to_string();
        let test_divisible: u64 = lines.next().unwrap().unwrap().trim().to_string()[19..]
            .parse()
            .unwrap();
        let test_true_to: usize = lines.next().unwrap().unwrap().trim().to_string()[25..]
            .parse()
            .unwrap();
        let test_false_to: usize = lines.next().unwrap().unwrap().trim().to_string()[26..]
            .parse()
            .unwrap();

        comman_divisor *= test_divisible;
        let monkey = Monkey {
            id,
            items,
            operation,
            test_divisible,
            test_false_to,
            test_true_to,
            inspects: 0,
        };

        monkeys.push(monkey);
        lines.next();
    }

    // Solve
    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            // for monkey in monkeys.iter_mut() {
            while let Some(item) = monkeys.get_mut(i).unwrap().items.pop_front() {
                monkeys.get_mut(i).unwrap().inspects += 1;
                let division_test = monkeys
                    .get(i)
                    .unwrap()
                    .operation
                    .replace("old", &item.to_string());
                let test_result = eval(&division_test).unwrap().as_u64().unwrap() % comman_divisor;
                if test_result % monkeys.get(i).unwrap().test_divisible == 0 {
                    let test_true_to = monkeys.get(i).unwrap().test_true_to;
                    monkeys
                        .get_mut(test_true_to)
                        .unwrap()
                        .items
                        .push_back(test_result);
                } else {
                    let test_false_to = monkeys.get(i).unwrap().test_false_to;
                    monkeys
                        .get_mut(test_false_to)
                        .unwrap()
                        .items
                        .push_back(test_result);
                }
            }
        }
    }

    // Result
    monkeys.sort_by_key(|k| k.inspects);

    let result = monkeys.get(monkeys.len() - 1).unwrap().inspects
        * monkeys.get(monkeys.len() - 2).unwrap().inspects;

    println!("{:?}", &monkeys);

    println!("Result is {}", result);
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
