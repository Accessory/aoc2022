use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn run(input_file: &str) {
    let mut result: u32 = 0;

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut split = line.split(",");
        let left = split.next().unwrap();
        let right = split.next().unwrap();
        let mut ranges_left = left.split("-");
        let mut ranges_right = right.split("-");
        let start0 = ranges_left.next().unwrap().parse::<usize>().unwrap();
        let end0 = ranges_left.next().unwrap().parse::<usize>().unwrap();
        let start1 = ranges_right.next().unwrap().parse::<usize>().unwrap();
        let end1 = ranges_right.next().unwrap().parse::<usize>().unwrap();

        // println!("Line {}-{},{}-{}", &start0, &end0, &start1, &end1);

        if (start0 <= start1 && end0 >= end1) || (start1 <= start0 && end1 >= end0) {
            result = result + 1;
        }
    }

    println!("The final Result is: {:?}", result);
}

fn run2(input_file: &str) {
    let mut result: u32 = 0;

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut split = line.split(",");
        let left = split.next().unwrap();
        let right = split.next().unwrap();
        let mut ranges_left = left.split("-");
        let mut ranges_right = right.split("-");
        let start0 = ranges_left.next().unwrap().parse::<usize>().unwrap();
        let end0 = ranges_left.next().unwrap().parse::<usize>().unwrap();
        let start1 = ranges_right.next().unwrap().parse::<usize>().unwrap();
        let end1 = ranges_right.next().unwrap().parse::<usize>().unwrap();

        // println!("Line {}-{},{}-{}", &start0, &end0, &start1, &end1);

        if (start0 >= start1 && start0 <= end1)
            || (end0 >= start1 && end0 <= end1)
            || (start1 >= start0 && start1 <= end0)
        {
            result = result + 1;
        }
    }

    println!("The final Result is: {:?}", result);
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
