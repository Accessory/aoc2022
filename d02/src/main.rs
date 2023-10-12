use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[allow(dead_code)]
#[derive(Debug)]
struct RPS {
    pub name: String,
    pub short_name: String,
    pub short_name_alt: String,
    pub points: usize,
    pub wins_to: String,
    pub wins_to_alt: String,
    pub lose_to_alt: String,
}
#[warn(dead_code)]
fn create_opponent_map() -> HashMap<String, RPS> {
    HashMap::from([
        (
            String::from("A"),
            RPS {
                name: String::from("Rock"),
                short_name: String::from("A"),
                short_name_alt: String::from("X"),
                points: 1,
                wins_to: String::from("C"),
                wins_to_alt: String::from("Z"),
                lose_to_alt: String::from("Y"),
            },
        ),
        (
            String::from("B"),
            RPS {
                name: String::from("Paper"),
                short_name: String::from("B"),
                short_name_alt: String::from("Y"),
                points: 2,
                wins_to: String::from("A"),
                wins_to_alt: String::from("X"),
                lose_to_alt: String::from("Z"),
            },
        ),
        (
            String::from("C"),
            RPS {
                name: String::from("Scissors"),
                short_name: String::from("C"),
                short_name_alt: String::from("Z"),
                points: 3,
                wins_to: String::from("B"),
                wins_to_alt: String::from("Y"),
                lose_to_alt: String::from("X"),
            },
        ),
    ])
}

fn create_you_map() -> HashMap<String, RPS> {
    HashMap::from([
        (
            String::from("X"),
            RPS {
                name: String::from("Rock"),
                short_name: String::from("A"),
                short_name_alt: String::from("X"),
                points: 1,
                wins_to: String::from("C"),
                wins_to_alt: String::from("Z"),
                lose_to_alt: String::from("Y"),
            },
        ),
        (
            String::from("Y"),
            RPS {
                name: String::from("Paper"),
                short_name: String::from("B"),
                short_name_alt: String::from("Y"),
                points: 2,
                wins_to: String::from("A"),
                wins_to_alt: String::from("X"),
                lose_to_alt: String::from("Z"),
            },
        ),
        (
            String::from("Z"),
            RPS {
                name: String::from("Scissors"),
                short_name: String::from("C"),
                short_name_alt: String::from("Z"),
                points: 3,
                wins_to: String::from("B"),
                wins_to_alt: String::from("Y"),
                lose_to_alt: String::from("X"),
            },
        ),
    ])
}

fn run(input_file: &str) {
    let mut final_result = 0;

    let opponent_map = create_opponent_map();
    let you_map = create_you_map();

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut split = line.split(" ");
        let opponent = split.next().unwrap();
        let you = split.next().unwrap();

        let opponent_item = opponent_map.get(opponent).unwrap();
        let you_item = you_map.get(you).unwrap();

        if opponent_item.short_name_alt == you {
            final_result += you_item.points;
            final_result += 3;
        } else if opponent_item.wins_to_alt == you {
            final_result += you_item.points;
        } else {
            final_result += you_item.points;
            final_result += 6;
        }
    }

    println!("The final Result is: {:?}", final_result);
}

fn get_your_item<'a>(
    opponent_item: &'a RPS,
    you: &'a str,
    you_map: &'a HashMap<String, RPS>,
) -> Option<&'a RPS> {
    match you {
        "X" => Some(you_map.get(&opponent_item.wins_to_alt).unwrap()),
        "Y" => Some(you_map.get(&opponent_item.short_name_alt).unwrap()),
        "Z" => Some(you_map.get(&opponent_item.lose_to_alt).unwrap()),
        _ => None,
    }
}

fn run2(input_file: &str) {
    let mut final_result = 0;

    let opponent_map = create_opponent_map();
    let you_map = create_you_map();

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut split = line.split(" ");
        let opponent = split.next().unwrap();
        let you = split.next().unwrap();

        let opponent_item = opponent_map.get(opponent).unwrap();
        let you_item = get_your_item(opponent_item, you, &you_map).unwrap();

        if opponent_item.short_name == you_item.short_name {
            final_result += you_item.points;
            final_result += 3;
        } else if opponent_item.wins_to_alt == you_item.short_name_alt {
            final_result += you_item.points;
        } else {
            final_result += you_item.points;
            final_result += 6;
        }
    }

    println!("The final Result is: {:?}", final_result);
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
