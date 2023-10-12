use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Coord {
    pub x: i16,
    pub y: i16,
}

fn wants_to_move(elv: &Coord, elves: &HashSet<Coord>) -> bool {
    let x = elv.x;
    let y = elv.y;

    let coords = Vec::from([
        Coord { x: x - 1, y: y - 1 },
        Coord { x: x - 1, y },
        Coord { x: x - 1, y: y + 1 },
        Coord { x, y: y - 1 },
        Coord { x, y: y + 1 },
        Coord { x: x + 1, y: y - 1 },
        Coord { x: x + 1, y },
        Coord { x: x + 1, y: y + 1 },
    ]);

    !are_coords_empty(&coords, elves)
}

fn are_coords_empty(coords: &[Coord], coords_set: &HashSet<Coord>) -> bool {
    for coord in coords {
        if coords_set.contains(coord) {
            return false;
        }
    }
    true
}

enum Direction {
    North,
    South,
    West,
    East,
}

fn get_direction_array() -> [Direction; 4] {
    [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]
}

fn get_directional_check_coords(elf: &Coord, current_direction: &Direction) -> [Coord; 3] {
    match current_direction {
        Direction::North => [
            Coord {
                x: elf.x - 1,
                y: elf.y - 1,
            },
            Coord {
                x: elf.x,
                y: elf.y - 1,
            },
            Coord {
                x: elf.x + 1,
                y: elf.y - 1,
            },
        ],
        Direction::South => [
            Coord {
                x: elf.x - 1,
                y: elf.y + 1,
            },
            Coord {
                x: elf.x,
                y: elf.y + 1,
            },
            Coord {
                x: elf.x + 1,
                y: elf.y + 1,
            },
        ],
        Direction::West => [
            Coord {
                x: elf.x - 1,
                y: elf.y - 1,
            },
            Coord {
                x: elf.x - 1,
                y: elf.y,
            },
            Coord {
                x: elf.x - 1,
                y: elf.y + 1,
            },
        ],
        Direction::East => [
            Coord {
                x: elf.x + 1,
                y: elf.y - 1,
            },
            Coord {
                x: elf.x + 1,
                y: elf.y,
            },
            Coord {
                x: elf.x + 1,
                y: elf.y + 1,
            },
        ],
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut elves: HashSet<Coord> = HashSet::new();
    let direction_list = get_direction_array();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(Coord {
                    x: x as i16,
                    y: y as i16,
                });
            }
        }
    }

    // Solve
    const ROUNDS: usize = 10;

    for round in 0..ROUNDS {
        //  println!("Starting round {}", round + 1);
        let mut moves: HashMap<Coord, Coord> = HashMap::new();
        let mut seen: HashSet<Coord> = HashSet::new();
        for elf in &elves {
            let wants_to_move = wants_to_move(elf, &elves);

            if wants_to_move {
                for dc in 0..direction_list.len() {
                    let direction = (round + dc) % direction_list.len();
                    let current_direction = direction_list.get(direction).unwrap();
                    let coords_to_check = get_directional_check_coords(elf, current_direction);

                    if are_coords_empty(&coords_to_check, &elves) {
                        let move_to = coords_to_check.get(1).unwrap();
                        if !seen.contains(move_to) && moves.insert(*move_to, *elf).is_some() {
                            moves.remove(move_to);
                            seen.insert(*move_to);
                        }
                        break;
                    }
                }
            }
        }

        for (key, value) in moves {
            elves.remove(&value);
            elves.insert(key);
        }
    }

    // Result
    let mut min_x = i16::MAX;
    let mut min_y = i16::MAX;
    let mut max_x = i16::MIN;
    let mut max_y = i16::MIN;

    for elv in &elves {
        min_x = min_x.min(elv.x);
        min_y = min_y.min(elv.y);
        max_x = max_x.max(elv.x);
        max_y = max_y.max(elv.y);
    }

    let result = (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i16;

    println!("The Result for part 1 is {result}");
}

fn run2(input_file: &str) {
    // Preamble
    let mut elves: HashSet<Coord> = HashSet::new();
    let direction_list = get_direction_array();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(Coord {
                    x: x as i16,
                    y: y as i16,
                });
            }
        }
    }

    // Solve
    const ROUNDS: usize = usize::MAX;
    let mut end_at = 0;

    for round in 0..ROUNDS {
        // println!("Starting round {}", round + 1);
        let mut moves: HashMap<Coord, Coord> = HashMap::new();
        let mut seen: HashSet<Coord> = HashSet::new();
        for elf in &elves {
            let wants_to_move = wants_to_move(elf, &elves);

            if wants_to_move {
                for dc in 0..direction_list.len() {
                    let direction = (round + dc) % direction_list.len();
                    let current_direction = direction_list.get(direction).unwrap();
                    let coords_to_check = get_directional_check_coords(elf, current_direction);

                    if are_coords_empty(&coords_to_check, &elves) {
                        let move_to = coords_to_check.get(1).unwrap();
                        if !seen.contains(move_to) && moves.insert(*move_to, *elf).is_some() {
                            moves.remove(move_to);
                            seen.insert(*move_to);
                        }
                        break;
                    }
                }
            }
        }

        if moves.is_empty() {
            end_at = round + 1;
            break;
        }

        for (key, value) in moves {
            elves.remove(&value);
            elves.insert(key);
        }
    }

    // Result
    println!("It took {end_at} rounds");
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

    println!("{input_file:?}");

    run(input_file);
    run2(input_file);
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
