use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Debug, Clone)]
struct Coords {
    x: u64,
    y: u64,
}

struct State {
    steps: u64,
    id: u64,
    step_list: Vec<Coords>,
}

fn create_node_id(x: u64, y: u64, x_length: u64) -> u64 {
    return y * x_length + x;
}

fn create_coords_from_id(id: u64, x_length: u64) -> Coords {
    let rtn = Coords {
        x: id % x_length,
        y: id / x_length,
    };
    return rtn;
}

fn create_next_node_list(id: u64, x_length: u64, y_length: u64, grid: &Vec<Vec<u64>>) -> Vec<u64> {
    let mut rtn: Vec<u64> = Vec::new();

    let coords = create_coords_from_id(id, x_length);
    let x = coords.x as i64;
    let y = coords.y as i64;

    let to_check = [[y - 1, x], [y + 1, x], [y, x - 1], [y, x + 1]];

    let height = grid
        .get(y as usize)
        .unwrap()
        .get(x as usize)
        .unwrap()
        .clone();

    for check in to_check {
        let iy = check.get(0).unwrap().clone();
        let ix = check.get(1).unwrap().clone();

        if ix < 0 || ix >= x_length as i64 || iy < 0 || iy >= y_length as i64 {
            continue;
        }

        let neighbour_height = grid
            .get(iy as usize)
            .unwrap()
            .get(ix as usize)
            .unwrap()
            .clone();

        if (height + 1) >= neighbour_height {
            let node_id = create_node_id(ix as u64, iy as u64, x_length);
            // println!("x: {} y: {} id: {}", ix, iy, node_id);
            rtn.push(node_id);
        }
    }
    return rtn;
}

fn create_next_node_list_r2(
    id: u64,
    x_length: u64,
    y_length: u64,
    grid: &Vec<Vec<u64>>,
) -> Vec<u64> {
    let mut rtn: Vec<u64> = Vec::new();

    let coords = create_coords_from_id(id, x_length);
    let x = coords.x as i64;
    let y = coords.y as i64;

    let to_check = [[y - 1, x], [y + 1, x], [y, x - 1], [y, x + 1]];

    let height = grid
        .get(y as usize)
        .unwrap()
        .get(x as usize)
        .unwrap()
        .clone();

    for check in to_check {
        let iy = check.get(0).unwrap().clone();
        let ix = check.get(1).unwrap().clone();

        if ix < 0 || ix >= x_length as i64 || iy < 0 || iy >= y_length as i64 {
            continue;
        }

        let neighbour_height = grid
            .get(iy as usize)
            .unwrap()
            .get(ix as usize)
            .unwrap()
            .clone();

        if (neighbour_height as i64 + 1) >= height as i64 {
            let node_id = create_node_id(ix as u64, iy as u64, x_length);
            // println!("x: {} y: {} id: {}", ix, iy, node_id);
            rtn.push(node_id);
        }
    }

    // println!("{:?}", &rtn);

    return rtn;
}

fn run(input_file: &str) {
    // Preamble
    let mut grid: Vec<Vec<u64>> = Vec::new();
    let mut start_id = 0;
    let mut end_id = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut current_id = 0;
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut row: Vec<u64> = Vec::new();

        for c in line.chars() {
            if c == 'E' {
                end_id = current_id;
                row.push('z'.into());
            } else if c == 'S' {
                start_id = current_id;
                row.push('a'.into());
            } else {
                row.push(c.into());
            }

            current_id += 1;
        }

        grid.push(row);
    }

    // Prepare
    let y_length: u64 = grid.len() as u64;
    let x_length: u64 = grid.get(0).unwrap().len() as u64;

    // Solve
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut visited: HashSet<u64> = HashSet::new();

    queue.push_back(State {
        steps: 0,
        id: start_id,
        step_list: Vec::from([Coords { x: 0, y: 0 }]),
    });

    let mut result = 0;

    'outer: while let Some(state) = queue.pop_front() {
        let next_nodes: Vec<u64> = create_next_node_list(state.id, x_length, y_length, &grid);

        for next_node in next_nodes {
            if visited.contains(&next_node) {
                continue;
            }

            if end_id == next_node {
                result = state.steps + 1;
                break 'outer;
            }

            visited.insert(next_node);
            let mut step_list = state.step_list.clone();
            step_list.push(create_coords_from_id(next_node, x_length));
            queue.push_back(State {
                steps: state.steps + 1,
                id: next_node,
                step_list,
            })
        }
    }

    // Result

    println!("Result for part 1 {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut grid: Vec<Vec<u64>> = Vec::new();
    let mut start_id = 0;
    let final_char: u64 = 'a'.into();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut current_id = 0;
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let mut row: Vec<u64> = Vec::new();

        for c in line.chars() {
            if c == 'E' {
                start_id = current_id;
                row.push('z'.into());
            } else if c == 'S' {
                row.push('a'.into());
            } else {
                row.push(c.into());
            }

            current_id += 1;
        }

        grid.push(row);
    }

    // Prepare
    let y_length: u64 = grid.len() as u64;
    let x_length: u64 = grid.get(0).unwrap().len() as u64;

    // Solve
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut visited: HashSet<u64> = HashSet::new();

    queue.push_back(State {
        steps: 0,
        id: start_id,
        step_list: Vec::from([Coords { x: 0, y: 0 }]),
    });

    let mut result = 0;

    'outer: while let Some(state) = queue.pop_front() {
        let next_nodes: Vec<u64> = create_next_node_list_r2(state.id, x_length, y_length, &grid);

        for next_node in next_nodes {
            if visited.contains(&next_node) {
                continue;
            }

            let coords = create_coords_from_id(next_node, x_length);
            let x = coords.x as i64;
            let y = coords.y as i64;

            let height = grid
                .get(y as usize)
                .unwrap()
                .get(x as usize)
                .unwrap()
                .clone();

            if height == final_char {
                result = state.steps + 1;
                break 'outer;
            }

            visited.insert(next_node);
            let mut step_list = state.step_list.clone();
            step_list.push(create_coords_from_id(next_node, x_length));
            queue.push_back(State {
                steps: state.steps + 1,
                id: next_node,
                step_list,
            })
        }
        // println!("Queue length {}", queue.len());
    }

    // Result

    println!("Result for part 2 is {}", result);
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
