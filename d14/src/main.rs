#![feature(array_windows)]

use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

use utils::get_input_path;

struct Point {
    x: i64,
    y: i64,
}

#[allow(dead_code)]
fn print_grid_to_file(grid: &Vec<Vec<char>>, path: &str) {
    let mut file = fs::File::create(path).unwrap();
    for y in grid {
        for x in y {
            write!(file, "{}", x).expect("Failed writing to the grid");
        }
        writeln!(file).expect("Failed writing newline the grid");
    }
}

fn is_abyss(next_x: usize, next_y: usize, min_x: i64, min_y: i64, max_x: i64, max_y: i64) -> bool {
    next_y < min_y as usize
        || next_y >= max_y as usize
        || next_x < 0 as usize
        || next_x >= (max_x - min_x) as usize
}

fn run(input_file: &str) {
    // Preamble
    const SAND_START: i64 = 500;
    let mut min_x: i64 = i64::MAX;
    let mut min_y: i64 = 0;
    let mut max_x: i64 = i64::MIN;
    let mut max_y: i64 = i64::MIN;
    let mut grid = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut rock_formation: Vec<Vec<Point>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let points = line.split(" -> ");
        let mut rock_line = Vec::new();
        for point in points {
            let mut xy = point.split(",");
            let x = xy.next().unwrap().trim().parse().unwrap();
            let y = xy.next().unwrap().trim().parse().unwrap();
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            rock_line.push(Point { x, y });
        }
        rock_formation.push(rock_line);
    }

    max_x += 1;
    max_y += 2;

    // Create Grid
    for _ in min_y..max_y {
        let mut row = Vec::new();
        for _ in min_x..max_x {
            row.push('.'.to_owned());
        }
        grid.push(row);
    }

    // Add Rocks
    for rock_lines in &rock_formation {
        for [point1, point2] in rock_lines.array_windows() {
            let x1 = point1.x - min_x;
            let y1 = point1.y - min_y;
            let x2 = point2.x - min_x;
            let y2 = point2.y - min_y;

            if y1 == y2 {
                let x_min = x1.min(x2);
                let x_max = x1.max(x2) + 1;
                for d in x_min..x_max {
                    grid[y1 as usize][d as usize] = '#'.to_owned();
                }
            } else {
                let y_min = y1.min(y2);
                let y_max = y1.max(y2) + 1;
                for d in y_min..y_max {
                    grid[d as usize][x1 as usize] = '#'.to_owned();
                }
            }
        }
    }

    // Solve
    let sand_start_y: i64 = 0;
    let sand_start_x: i64 = SAND_START - min_x;
    let mut sand_drops: usize = 0;

    let mut sand_reached_abyss = false;
    let mut rounds = max_x * max_y;

    while !sand_reached_abyss && rounds > 0 {
        rounds -= 1;

        let mut can_continue = true;
        let mut sand_x = sand_start_x as usize;
        let mut sand_y = sand_start_y as usize;
        let mut velocity_check: i64 = 0;

        while can_continue {
            let next_y = sand_y + 1;
            let mut next_x = sand_x;

            if is_abyss(next_x, next_y, min_x, min_y, max_x, max_y) {
                can_continue = false;
                sand_reached_abyss = true;
                continue;
            }

            if grid[next_y][next_x] == '#' {
                if velocity_check != 0 {
                    next_x = (next_x as i64 + velocity_check) as usize;

                    if is_abyss(next_x, next_y, min_x, min_y, max_x, max_y) {
                        can_continue = false;
                        sand_reached_abyss = true;
                        continue;
                    }

                    if grid[next_y][next_x] == '.' {
                        sand_x = next_x;
                        sand_y = next_y;
                        continue;
                    }
                }

                grid[sand_y][sand_x] = 'o';
                sand_drops += 1;
                can_continue = false;
                continue;
            }

            if grid[next_y][next_x] == 'o' {
                let mut finished = false;
                next_x = next_x - 1;
                if is_abyss(next_x, next_y, min_x, min_y, max_x, max_y) {
                    can_continue = false;
                    sand_reached_abyss = true;
                    continue;
                }

                if grid[next_y][next_x] == '.' {
                    velocity_check = -1;
                } else {
                    next_x = next_x + 2;
                    if is_abyss(next_x, next_y, min_x, min_y, max_x, max_y) {
                        can_continue = false;
                        sand_reached_abyss = true;
                        continue;
                    }

                    if grid[next_y][next_x] == '.' {
                        velocity_check = 1;
                        finished = true;
                    }

                    if !finished {
                        grid[sand_y][sand_x] = 'o';
                        sand_drops += 1;
                        can_continue = false;
                        continue;
                    }
                }
            }

            sand_x = next_x;
            sand_y = next_y;
        }
    }

    // print_grid_to_file(&grid, "target/output2.txt");
    println!("There are {} sanddrops falling until the abyss is reached", sand_drops);
}

fn run2(input_file: &str) {
    // Preamble
    const SAND_START: i64 = 500;
    let mut min_x: i64 = i64::MAX;
    let mut min_y: i64 = 0;
    let mut max_x: i64 = i64::MIN;
    let mut max_y: i64 = i64::MIN;
    let mut grid = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut rock_formation: Vec<Vec<Point>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let points = line.split(" -> ");
        let mut rock_line = Vec::new();
        for point in points {
            let mut xy = point.split(",");
            let x = xy.next().unwrap().trim().parse().unwrap();
            let y = xy.next().unwrap().trim().parse().unwrap();
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            rock_line.push(Point { x, y });
        }
        rock_formation.push(rock_line);
    }

    max_x += 500;
    min_x -= 500;
    max_y += 3;

    // Create Grid
    for _ in min_y..(max_y - 1) {
        let mut row = Vec::new();
        for _ in min_x..max_x {
            row.push('.'.to_owned());
        }
        grid.push(row);
    }
    {
        let mut row = Vec::new();
        for _ in min_x..max_x {
            row.push('#'.to_owned());
        }
        grid.push(row);
    }

    // Add Rocks
    for rock_lines in &rock_formation {
        for [point1, point2] in rock_lines.array_windows() {
            let x1 = point1.x - min_x;
            let y1 = point1.y - min_y;
            let x2 = point2.x - min_x;
            let y2 = point2.y - min_y;

            if y1 == y2 {
                let x_min = x1.min(x2);
                let x_max = x1.max(x2) + 1;
                for d in x_min..x_max {
                    grid[y1 as usize][d as usize] = '#'.to_owned();
                }
            } else {
                let y_min = y1.min(y2);
                let y_max = y1.max(y2) + 1;
                for d in y_min..y_max {
                    grid[d as usize][x1 as usize] = '#'.to_owned();
                }
            }
        }
    }

    // Solve
    let sand_start_y: i64 = 0;
    let sand_start_x: i64 = SAND_START - min_x;
    let mut sand_drops: usize = 0;

    let mut sand_reached_abyss = false;
    let mut rounds = max_x * max_y;

    while !sand_reached_abyss && rounds > 0 {
        rounds -= 1;

        let mut can_continue = true;
        let mut sand_x = sand_start_x as usize;
        let mut sand_y = sand_start_y as usize;
        let mut velocity_check: i64 = 0;

        if grid[sand_y][sand_x] == 'o' {
            break;
        }

        while can_continue {
            let next_y = sand_y + 1;
            let mut next_x = sand_x;

            if is_abyss(next_x, next_y, min_x, min_y, max_x, max_y) {
                can_continue = false;
                sand_reached_abyss = true;
                continue;
            }

            if grid[next_y][next_x] == '#' {
                if velocity_check != 0 {
                    next_x = (next_x as i64 + velocity_check) as usize;

                    if is_abyss(next_x, next_y, min_x, min_y, max_x, max_y) {
                        can_continue = false;
                        sand_reached_abyss = true;
                        continue;
                    }

                    if grid[next_y][next_x] == '.' {
                        sand_x = next_x;
                        sand_y = next_y;
                        continue;
                    }
                }

                grid[sand_y][sand_x] = 'o';
                sand_drops += 1;
                can_continue = false;
                continue;
            }

            if grid[next_y][next_x] == 'o' {
                let mut finished = false;
                next_x = next_x - 1;
                if is_abyss(next_x, next_y, min_x, min_y, max_x, max_y) {
                    can_continue = false;
                    sand_reached_abyss = true;
                    continue;
                }

                if grid[next_y][next_x] == '.' {
                    velocity_check = -1;
                } else {
                    next_x = next_x + 2;
                    if is_abyss(next_x, next_y, min_x, min_y, max_x, max_y) {
                        can_continue = false;
                        sand_reached_abyss = true;
                        continue;
                    }

                    if grid[next_y][next_x] == '.' {
                        velocity_check = 1;
                        finished = true;
                    }

                    if !finished {
                        grid[sand_y][sand_x] = 'o';
                        sand_drops += 1;
                        can_continue = false;
                        continue;
                    }
                }
            }

            sand_x = next_x;
            sand_y = next_y;
        }
    }

    // print_grid_to_file(&grid, "target/output2.txt");
    println!("There are {} sanddrops falling until the top is reached", sand_drops);
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
