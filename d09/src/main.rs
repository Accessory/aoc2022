#![feature(get_many_mut)]

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Coord {
    pub x: i64,
    pub y: i64,
}

#[allow(dead_code)]
struct Knot {
    pub id: usize,
    pub x: i64,
    pub y: i64,
    pub positions: Vec<Coord>,
}

fn aligning_chain(knots: &mut Vec<Knot>) {
    for i in 0..(knots.len()-1) {
        let [knot1, knot2] = knots.get_many_mut([i, i + 1]).unwrap();
        // let mut knot2 = knots.get_mut(i + 1).unwrap();

        if distance_greateer_2(knot1, knot2) {
            let mut xd = knot1.x - knot2.x;
            let mut yd = knot1.y - knot2.y;

            if xd.abs() > 1 {
                xd = xd / xd.abs();
            }

            if yd.abs() > 1 {
                yd = yd / yd.abs();
            }

            knot2.x += xd;
            knot2.y += yd;
        }
    }
}
fn save_positions(knots: &mut Vec<Knot>) {
    for knot in knots {
        knot.positions.push(Coord {
            x: knot.x,
            y: knot.y,
        });
    }
}

fn walk_up(steps: usize, knots: &mut Vec<Knot>) {
    for _ in 0..steps {
        knots.first_mut().unwrap().y += 1;
        aligning_chain(knots);
        save_positions(knots);
    }
}

fn walk_down(steps: usize, knots: &mut Vec<Knot>) {
    for _ in 0..steps {
        knots.first_mut().unwrap().y -= 1;
        aligning_chain(knots);
        save_positions(knots);
    }
}

fn walk_left(steps: usize, knots: &mut Vec<Knot>) {
    for _ in 0..steps {
        knots.first_mut().unwrap().x -= 1;
        aligning_chain(knots);
        save_positions(knots);
    }
}

fn walk_right(steps: usize, knots: &mut Vec<Knot>) {
    for _ in 0..steps {
        knots.first_mut().unwrap().x += 1;
        aligning_chain(knots);
        save_positions(knots);
    }
}

fn distance_greateer_2(knot1: &Knot, knot2: &Knot) -> bool {
    (knot1.x).abs_diff(knot2.x) >= 2 || (knot1.y).abs_diff(knot2.y) >= 2
}

fn run(input_file: &str) {
    // Init
    const KNOTS: usize = 2;
    let mut knots: Vec<Knot> = Vec::new();

    for i in 0..KNOTS {
        knots.push(Knot {
            id: i,
            x: 0,
            y: 0,
            positions: vec![Coord { x: 0, y: 0 }],
        });
    }

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    // Walk
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut split = line.split(" ");
        let direction = split.next().unwrap();
        let steps: usize = split.next().unwrap().parse().unwrap();

        match direction {
            "U" => walk_up(steps, &mut knots),
            "D" => walk_down(steps, &mut knots),
            "L" => walk_left(steps, &mut knots),
            "R" => walk_right(steps, &mut knots),
            _ => {
                panic!("Should not be here!");
            }
        }
    }

    // Result
    let mut tail_positions:HashSet<Coord> = HashSet::new();
    let tail = knots.last().unwrap();
    for p in &tail.positions {
        tail_positions.insert(p.clone());
    }

    println!("The final result is: {}", tail_positions.len());

}

fn run2(input_file: &str) {
    const KNOTS: usize = 10;
    let mut knots: Vec<Knot> = Vec::new();

    for i in 0..KNOTS {
        knots.push(Knot {
            id: i,
            x: 0,
            y: 0,
            positions: vec![Coord { x: 0, y: 0 }],
        });
    }

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    // Walk
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut split = line.split(" ");
        let direction = split.next().unwrap();
        let steps: usize = split.next().unwrap().parse().unwrap();

        match direction {
            "U" => walk_up(steps, &mut knots),
            "D" => walk_down(steps, &mut knots),
            "L" => walk_left(steps, &mut knots),
            "R" => walk_right(steps, &mut knots),
            _ => {
                panic!("Should not be here!");
            }
        }
    }

    // Result
    let mut tail_positions:HashSet<Coord> = HashSet::new();
    let tail = knots.last().unwrap();
    for p in &tail.positions {
        tail_positions.insert(p.clone());
    }

    println!("The final result is: {}", tail_positions.len());
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

    run(&input_file);
    run2(&input_file);
}

#[cfg(test)]
mod main_test {
    use utils::{get_test_input_2_path, get_test_input_path};
    
    use crate::run;
    use crate::run2;

    #[test]
    fn test_input_part_1() {
        let input_path = get_test_input_path(file!());
        run(input_path.to_str().unwrap());
    }

    #[test]
    fn test_input_part_2() {
        let input_path = get_test_input_2_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
