use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn print_forest(forest: &Vec<Vec<u8>>) {
    for tree_line in forest {
        for tree in tree_line {
            print!("{}", tree);
        }
        println!();
    }
}

fn look_right(x: usize, y: usize, forest: &Vec<Vec<u8>>) -> bool {
    let tree = forest.get(y).unwrap().get(x).unwrap();
    for ix in x + 1..forest.len() {
        let i_tree = forest.get(y).unwrap().get(ix).unwrap();
        if i_tree >= tree {
            return false;
        }
    }
    return true;
}

fn look_left(x: usize, y: usize, forest: &Vec<Vec<u8>>) -> bool {
    let tree = forest.get(y).unwrap().get(x).unwrap();
    for ix in (0..x).rev() {
        let i_tree = forest.get(y).unwrap().get(ix).unwrap();
        if i_tree >= tree {
            return false;
        }
    }
    return true;
}

fn look_up(x: usize, y: usize, forest: &Vec<Vec<u8>>) -> bool {
    let tree = forest.get(y).unwrap().get(x).unwrap();
    for iy in y + 1..forest.len() {
        let i_tree = forest.get(iy).unwrap().get(x).unwrap();
        if i_tree >= tree {
            return false;
        }
    }
    return true;
}

fn look_down(x: usize, y: usize, forest: &Vec<Vec<u8>>) -> bool {
    let tree = forest.get(y).unwrap().get(x).unwrap();
    for iy in (0..y).rev() {
        let i_tree = forest.get(iy).unwrap().get(x).unwrap();
        if i_tree >= tree {
            return false;
        }
    }
    return true;
}

fn look_right_count(x: usize, y: usize, forest: &Vec<Vec<u8>>) -> usize {
    let mut rtn = 0;
    let tree = forest.get(y).unwrap().get(x).unwrap();
    for ix in x + 1..forest.len() {
        let i_tree = forest.get(y).unwrap().get(ix).unwrap();
        rtn += 1;
        if i_tree >= tree {
            return rtn;
        }
    }
    return rtn;
}

fn look_left_count(x: usize, y: usize, forest: &Vec<Vec<u8>>) -> usize {
    let mut rtn = 0;
    let tree = forest.get(y).unwrap().get(x).unwrap();
    for ix in (0..x).rev() {
        let i_tree = forest.get(y).unwrap().get(ix).unwrap();
        rtn += 1;
        if i_tree >= tree {
            return rtn;
        }
    }
    return rtn;
}

fn look_up_count(x: usize, y: usize, forest: &Vec<Vec<u8>>) -> usize {
    let mut rtn = 0;
    let tree = forest.get(y).unwrap().get(x).unwrap();
    for iy in y + 1..forest.len() {
        let i_tree = forest.get(iy).unwrap().get(x).unwrap();
        rtn += 1;
        if i_tree >= tree {
            return rtn;
        }
    }
    return rtn;
}

fn look_down_count(x: usize, y: usize, forest: &Vec<Vec<u8>>) -> usize {
    let mut rtn = 0;
    let tree = forest.get(y).unwrap().get(x).unwrap();
    for iy in (0..y).rev() {
        let i_tree = forest.get(iy).unwrap().get(x).unwrap();
        rtn += 1;
        if i_tree >= tree {
            return rtn;
        }
    }
    return rtn;
}

fn run(input_file: &str) {
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut forest: Vec<Vec<u8>> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        let mut tree_line: Vec<u8> = Vec::new();

        for c in line.chars() {
            tree_line.push(c.to_digit(10).unwrap() as u8);
        }
        forest.push(tree_line)
    }

    print_forest(&forest);

    // Prepare
    let mut result_map: Vec<Vec<u8>> = Vec::new();
    let mut result_count: usize = 0;

    // Solve
    for tree_line in &forest {
        let mut line: Vec<u8> = Vec::new();
        for _tree in tree_line {
            line.push(0);
        }
        result_map.push(line);
    }

    for (y, tree_line) in forest.iter().enumerate() {
        for (x, _tree) in tree_line.iter().enumerate() {
            let result = look_right(x, y, &forest)
                || look_left(x, y, &forest)
                || look_up(x, y, &forest)
                || look_down(x, y, &forest);

            result_map.get_mut(y).unwrap()[x] = if result { 1 } else { 0 };
            result_count += if result { 1 } else { 0 }
        }
    }
    println!("Result Map:");
    print_forest(&result_map);
    println!();
    println!("Result: {}", result_count);
}

fn run2(input_file: &str) {
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut forest: Vec<Vec<u8>> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        let mut tree_line: Vec<u8> = Vec::new();

        for c in line.chars() {
            tree_line.push(c.to_digit(10).unwrap() as u8);
        }
        forest.push(tree_line)
    }

    print_forest(&forest);

    // Prepare
    let mut result_map: Vec<Vec<usize>> = Vec::new();
    let mut final_result: usize = 0;

    // Solve
    for tree_line in &forest {
        let mut line: Vec<usize> = Vec::new();
        for _tree in tree_line {
            line.push(0);
        }
        result_map.push(line);
    }

    for (y, tree_line) in forest.iter().enumerate() {
        for (x, _tree) in tree_line.iter().enumerate() {
            let result = look_right_count(x, y, &forest)
                * look_left_count(x, y, &forest)
                * look_up_count(x, y, &forest)
                * look_down_count(x, y, &forest);

            result_map.get_mut(y).unwrap()[x] = result;
            final_result = max(final_result, result);
        }
    }

    println!();
    println!("Result: {}", final_result);
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
