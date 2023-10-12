#![feature(array_windows)]
#![feature(array_chunks)]

use std::fs::File;
use std::io::{BufRead, BufReader};

use serde_json::{json, Value};
use utils::get_input_path;

fn check_packages(package1: &Value, package2: &Value) -> i8 {
    // println!("{} vs {}", package1, package2);
    let max_length = package1
        .as_array()
        .unwrap()
        .len()
        .min(package2.as_array().unwrap().len());

    for i in 0..max_length {
        let ipl = package1.get(i).unwrap();
        let ipr = package2.get(i).unwrap();

        if ipl.is_array() || ipr.is_array() {
            let to_use_ipl = if ipl.is_array() {
                ipl.clone()
            } else {
                json!([ipl.as_i64()])
            };

            let to_use_ipr = if ipr.is_array() {
                ipr.clone()
            } else {
                json!([ipr.as_i64()])
            };

            let zr = check_packages(&to_use_ipl, &to_use_ipr);

            if zr == -1 || zr == 1 {
                return zr;
            }
        }

        let iplv = if ipl.is_number() {
            ipl.as_i64().unwrap()
        } else {
            i64::MAX
        };

        let iprv = if ipr.is_number() {
            ipr.as_i64().unwrap()
        } else {
            i64::MAX
        };

        if iplv > iprv {
            return -1;
        }

        if iplv < iprv {
            return 1;
        }
    }

    if package1.as_array().unwrap().len() > package2.as_array().unwrap().len() {
        return -1;
    }

    if package1.as_array().unwrap().len() < package2.as_array().unwrap().len() {
        return 1;
    }

    return 0;
}

fn run(input_file: &str) {
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut values: Vec<Value> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        if line.is_empty() {
            continue;
        }

        let value = serde_json::from_str(&line).unwrap();
        values.push(value);
    }

    // Solve
    let mut in_order: usize = 0;
    for (idx, [package1, package2]) in values.array_chunks().enumerate() {
        print!("{} vs {}", package1, package2);
        let check_result = check_packages(package1, package2);
        if check_result == 1 {
            in_order += idx + 1;
            println!(" is in order");
        } else {
            println!(" is not in order");
        }
    }
    // Result
    println!("Result: {}", in_order);
}

fn run2(input_file: &str) {
    // Preamble
    let package_with_two = json!([[2]]);
    let package_with_six = json!([[6]]);

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut values: Vec<Value> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        if line.is_empty() {
            continue;
        }

        let value = serde_json::from_str(&line).unwrap();
        values.push(value);
    }

    values.push(package_with_two.clone());
    values.push(package_with_six.clone());

    // Solve
    values.sort_unstable_by(|p1, p2|{
        match check_packages(p1, p2) {
            1 => std::cmp::Ordering::Less,
            -1 => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        }
    });

    let mut package_with_two_idx = 0;
    let mut package_with_six_idx = 0;
    for (idx, value) in values.iter().enumerate() {
        if *value == package_with_two {
            package_with_two_idx = idx + 1;
        }

        if *value == package_with_six {
            package_with_six_idx = idx + 1;
        }

        if package_with_six_idx != 0 && package_with_two_idx != 0 {
            break;
        }
    }

    
    // Result
    let decoder_key = package_with_two_idx * package_with_six_idx;
    println!("Decoder key for the distress signal: {}", decoder_key);
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
