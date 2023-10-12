use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(PartialEq, Eq)]
enum PointState {
    Free,
    Locked,
}

fn generate_points_to_check(point: &Point) -> Vec<Point> {
    let x = point.x;
    let y = point.y;
    let z = point.z;
    Vec::from([
        Point { x, y, z: z + 1 },
        Point { x, y: y + 1, z },
        Point { x: x + 1, y, z },
        Point { x, y, z: z - 1 },
        Point { x, y: y - 1, z },
        Point { x: x - 1, y, z },
    ])
}

fn is_locked(
    air_gab: &Point,
    points: &HashSet<Point>,
    cache: &mut HashMap<Point, PointState>,
    visited: &mut HashSet<Point>,
    min_x: i64,
    min_y: i64,
    min_z: i64,
    max_x: i64,
    max_y: i64,
    max_z: i64,
) -> bool {
    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(air_gab.clone());

    while let Some(point) = queue.pop_back() {
        if visited.contains(&point) || points.contains(&point) {
            continue;
        }

        if cache.contains_key(&point) {
            let state = cache.get(&point).unwrap();
            return *state == PointState::Locked;
        }

        let x = point.x;
        let y = point.y;
        let z = point.z;

        if x < min_x || x > max_x || y < min_y || y > max_y || z < min_z || z > max_z {
            return false;
        }

        visited.insert(point.clone());

        let points_to_check = generate_points_to_check(&point);
        for ptc in points_to_check {
            queue.push_front(ptc);
        }
    }

    return true;
}

fn run(input_file: &str) {
    // Preamble
    let mut points: HashSet<Point> = HashSet::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        let mut split = line.split(",");
        let x: i64 = split.next().unwrap().parse().unwrap();
        let y: i64 = split.next().unwrap().parse().unwrap();
        let z: i64 = split.next().unwrap().parse().unwrap();

        points.insert(Point { x, y, z });
    }

    // Solve
    let mut surfaces = 0;
    for point in &points {
        let check_points = generate_points_to_check(&point);
        for check_point in &check_points {
            if !points.contains(check_point) {
                surfaces += 1;
            }
        }
    }

    // Result
    println!("There are {} visible surfaces.", surfaces);
}

fn run2(input_file: &str) {
    // Preamble
    let mut points: HashSet<Point> = HashSet::new();
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut min_z = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;
    let mut max_z = i64::MIN;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        let mut split = line.split(",");
        let x: i64 = split.next().unwrap().parse().unwrap();
        let y: i64 = split.next().unwrap().parse().unwrap();
        let z: i64 = split.next().unwrap().parse().unwrap();

        min_x = min_x.min(x);
        min_y = min_y.min(y);
        min_z = min_z.min(z);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        max_z = max_z.max(z);

        points.insert(Point { x, y, z });
    }

    // Solve
    let mut air_gaps: Vec<Point> = Vec::new();
    for point in &points {
        let check_points = generate_points_to_check(&point);
        for check_point in check_points {
            if !points.contains(&check_point) {
                air_gaps.push(check_point);
            }
        }
    }

    let mut cache: HashMap<Point, PointState> = HashMap::new();

    let mut surfaces = 0;

    for air_gab in air_gaps {
        let mut visited: HashSet<Point> = HashSet::new();
        if is_locked(
            &air_gab,
            &points,
            &mut cache,
            &mut visited,
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
        ) {
            for v in visited {
                cache.insert(v, PointState::Locked);
            }
        } else {
            for v in visited {
                cache.insert(v, PointState::Free);
            }
            surfaces += 1;
        }
    }

    // Result
    println!("There are {} not airlocked sufaces.", surfaces);
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
