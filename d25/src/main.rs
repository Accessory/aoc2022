use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn run(input_file: &str) {
    let mut total: i64 = 0;

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        let mut coef = 1;
        for i in (0..line.len()).rev() {
            let x = line.chars().nth(i).unwrap();
            let n: i64 = "=-012".find(x).unwrap() as i64;
            total += (n - 2) * coef;
            coef = coef * 5;
        }
    }

    let mut output = String::new();

    while total != 0 {
        let rem = total % 5;
        total = total / 5;
        if rem <= 2 {
            output = rem.to_string() + &output;
        } else {
            let nc = "   =-".chars().nth(rem as usize).unwrap();
            output.insert(0, nc);
            total += 1;
        }
    }

    println!("The final Result is: {:?}", output);
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

    run(&input_file);
}

#[cfg(test)]
mod main_test {
    use utils::get_test_input_path;

    use crate::run;

    #[test]
    fn test_input_part_1() {
        let input_path = get_test_input_path(file!());
        run(input_path.to_str().unwrap());
    }
}
