use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

struct Coord {
    pub x: i16,
    pub y: i16,
}

struct Elv {
    pub x: i16,
    pub y: i16,
    pub nx: i16,
    pub ny: i16,
    pub wants_to_move: bool,
}

fn wants_to_move(elv: &Elv, elves: &[Elv]) -> bool {
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

fn are_coords_empty(coords: &[Coord], elves: &[Elv]) -> bool {
    for coord in coords {
        for elv in elves {
            if coord.x == elv.x && elv.y == coord.y {
                return false;
            }
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

fn get_directional_check_coords(elv: &Elv, current_direction: &Direction) -> [Coord; 3] {
    match current_direction {
        Direction::North => [
            Coord {
                x: elv.x - 1,
                y: elv.y - 1,
            },
            Coord {
                x: elv.x,
                y: elv.y - 1,
            },
            Coord {
                x: elv.x + 1,
                y: elv.y - 1,
            },
        ],
        Direction::South => [
            Coord {
                x: elv.x - 1,
                y: elv.y + 1,
            },
            Coord {
                x: elv.x,
                y: elv.y + 1,
            },
            Coord {
                x: elv.x + 1,
                y: elv.y + 1,
            },
        ],
        Direction::West => [
            Coord {
                x: elv.x - 1,
                y: elv.y - 1,
            },
            Coord {
                x: elv.x - 1,
                y: elv.y,
            },
            Coord {
                x: elv.x - 1,
                y: elv.y + 1,
            },
        ],
        Direction::East => [
            Coord {
                x: elv.x + 1,
                y: elv.y - 1,
            },
            Coord {
                x: elv.x + 1,
                y: elv.y,
            },
            Coord {
                x: elv.x + 1,
                y: elv.y + 1,
            },
        ],
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut elves: Vec<Elv> = Vec::new();
    let direction_list = get_direction_array();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.push(Elv {
                    x: x as i16,
                    y: y as i16,
                    nx: x as i16,
                    ny: y as i16,
                    wants_to_move: false,
                });
            }
        }
    }

    // Solve
    const ROUNDS: usize = 10;

    for round in 0..ROUNDS {
        // for mut elv in &mut elves {
        for idx in 0..elves.len() {
            let wants_to_move = wants_to_move(elves.get(idx).unwrap(), &elves);
            elves[idx].nx = elves[idx].x;
            elves[idx].ny = elves[idx].y;
            elves[idx].wants_to_move = wants_to_move;

            if elves[idx].wants_to_move {
                for dc in 0..direction_list.len() {
                    let direction = (round + dc) % direction_list.len();
                    let current_direction = direction_list.get(direction).unwrap();
                    let coords_to_check =
                        get_directional_check_coords(&elves[idx], current_direction);

                    if are_coords_empty(&coords_to_check, &elves) {
                        elves[idx].nx = coords_to_check.get(1).unwrap().x;
                        elves[idx].ny = coords_to_check.get(1).unwrap().y;
                        break;
                    }
                }
            }
        }

        for idx in 0..elves.len() {
            if !elves[idx].wants_to_move {
                continue;
            }

            for idx2 in 0..elves.len() {
                if idx == idx2 {
                    continue;
                }
                if elves[idx].nx == elves[idx2].nx && elves[idx].ny == elves[idx2].ny {
                    elves[idx].wants_to_move = false;
                    elves[idx2].wants_to_move = false;
                }
            }
        }

        for elv in &mut elves {
            if elv.wants_to_move {
                elv.x = elv.nx;
                elv.y = elv.ny;
            }
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

    println!("Result is {result}");
}

fn run2(input_file: &str) {
    // Preamble
    let mut elves: Vec<Elv> = Vec::new();
    let direction_list = get_direction_array();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.push(Elv {
                    x: x as i16,
                    y: y as i16,
                    nx: x as i16,
                    ny: y as i16,
                    wants_to_move: false,
                });
            }
        }
    }

    // Solve
    const ROUNDS: usize = usize::MAX;
    let mut end_at = 0;
    
    for round in 0..ROUNDS {
        // for mut elv in &mut elves {
        for idx in 0..elves.len() {
            let wants_to_move = wants_to_move(elves.get(idx).unwrap(), &elves);
            elves[idx].nx = elves[idx].x;
            elves[idx].ny = elves[idx].y;
            elves[idx].wants_to_move = wants_to_move;

            if elves[idx].wants_to_move {
                for dc in 0..direction_list.len() {
                    let direction = (round + dc) % direction_list.len();
                    let current_direction = direction_list.get(direction).unwrap();
                    let coords_to_check =
                        get_directional_check_coords(&elves[idx], current_direction);

                    if are_coords_empty(&coords_to_check, &elves) {
                        elves[idx].nx = coords_to_check.get(1).unwrap().x;
                        elves[idx].ny = coords_to_check.get(1).unwrap().y;
                        break;
                    }
                }
            }
        }

        for idx in 0..elves.len() {
            if !elves[idx].wants_to_move {
                continue;
            }

            for idx2 in 0..elves.len() {
                if idx == idx2 {
                    continue;
                }
                if elves[idx].nx == elves[idx2].nx && elves[idx].ny == elves[idx2].ny {
                    elves[idx].wants_to_move = false;
                    elves[idx2].wants_to_move = false;
                }
            }
        }

        let mut did_move = false;
        for elv in &mut elves {
            if elv.wants_to_move {
                elv.x = elv.nx;
                elv.y = elv.ny;
                did_move = true;
            }
        }

        if !did_move {
            end_at = round + 1;
            break;
        }
        // println!("End of round {}", round + 1);
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
