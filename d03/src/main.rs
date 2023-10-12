use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

fn run(input_file: &str) {
    let mut result: u32 = 0;

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut items: Vec<char> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut item_set = HashSet::<char>::new();

        let split_idx = line.len() / 2;
        item_set.extend(line[0..split_idx].chars());

        for i in split_idx..line.len() {
            let c = line.chars().nth(i).unwrap();
            if item_set.contains(&c) {
                items.push(c);
                break;
            }
        }
    }

    for i in items {
        if i == i.to_ascii_uppercase() {
            result = result + ((i as u32) - 38);
        } else {
            result = result + ((i as u32) - 96);
        }
    }

    println!("The final Result is: {:?}", result);
}

fn run2(input_file: &str) {
    let mut result: u32 = 0;

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut items: Vec<char> = Vec::new();

    let mut lines = reader.lines();


    while let Some(line) = lines.next() {
        let line1 = line.unwrap();
        let line2 = lines.next().unwrap().unwrap();
        let line3 = lines.next().unwrap().unwrap();

        let set1: HashSet<char> = HashSet::from_iter(line1.chars());
        let set2: HashSet<char> = HashSet::from_iter(line2.chars());
        let set3: HashSet<char> = HashSet::from_iter(line3.chars());

        let mut char_map = HashMap::<char, usize>::new();

        for c in set1 {
            if char_map.contains_key(&c) {
                let old_v = char_map.get(&c).unwrap();
                char_map.insert(c, old_v + 1);
            } else {
                char_map.insert(c, 1);
            }
        }

        for c in set2 {
            if char_map.contains_key(&c) {
                let old_v = char_map.get(&c).unwrap();
                char_map.insert(c, old_v + 1);
            } else {
                char_map.insert(c, 1);
            }
        }

        for c in set3 {
            if char_map.contains_key(&c) {
                let old_v = char_map.get(&c).unwrap();
                char_map.insert(c, old_v + 1);
            } else {
                char_map.insert(c, 1);
            }
        }

        for (key, value) in char_map {
            if value == 3 {
                items.push(key);
                break;
            }
        }
    }

    for i in items {
        if i == i.to_ascii_uppercase() {
            result = result + ((i as u32) - 38);
        } else {
            result = result + ((i as u32) - 96);
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
