use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use utils::get_input_path;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Coord {
    pub x: i64,
    pub y: i64,
}

impl Coord {
    pub fn distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn in_bounds(&self, dims: &Dims) -> bool {
        self.x >= dims.x_min && self.x < dims.x_max && self.y >= dims.y_min && self.y < dims.y_max
    }
}

struct Dims {
    pub x_min: i64,
    pub y_min: i64,
    pub x_max: i64,
    pub y_max: i64,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}
impl Orientation {
    fn from_char(c: char) -> Orientation {
        match c {
            '>' => Orientation::Right,
            '<' => Orientation::Left,
            '^' => Orientation::Up,
            'v' => Orientation::Down,
            _ => panic!("Should not be here!"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Blizzard {
    pub coord: Coord,
    pub orientation: Orientation,
}
impl Blizzard {
    pub(crate) fn from_xyo(x: usize, y: usize, c: char) -> Self {
        Self {
            coord: Coord {
                x: x as i64,
                y: y as i64,
            },
            orientation: Orientation::from_char(c),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Expedition {
    pub coords: Coord,
    pub minutes: usize,
    pub distance: i64,
}

impl PartialOrd for Expedition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.coords.partial_cmp(&other.coords) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.minutes.partial_cmp(&other.minutes) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for Expedition {
    // fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    //     if self.minutes != other.minutes {
    //         other.minutes.cmp(&self.minutes)
    //     } else {
    //         other.distance.cmp(&self.distance)
    //     }
    // }
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.minutes != other.minutes {
            self.minutes.cmp(&other.minutes)
        } else {
            self.distance.cmp(&other.distance)
        }
    }
}

#[allow(dead_code)]
fn print_map(_expedition: &Expedition, next_blizzards: &HashSet<Coord>, dims: &Dims) {
    for y in 0..dims.y_max {
        for x in 0..dims.x_max {
            if y == 0 || x == 0 {
                print!("#");
            }
            if next_blizzards.contains(&Coord { x, y }) {
                print!("X");
            } else {
                print!(".");
            }
        }
        print!("#");
        println!();
    }
    for _ in 0..dims.y_max + 3 {
        print!("#");
    }
    println!();
}

fn get_next_positions(
    base: &Coord,
    dims: &Dims,
    blizzard_coords: &HashSet<Coord>,
    skip: &HashSet<Coord>,
) -> Vec<Coord> {
    let mut rtn = Vec::from([
        Coord {
            x: base.x + 1,
            y: base.y,
        },
        Coord {
            x: base.x - 1,
            y: base.y,
        },
        Coord {
            x: base.x,
            y: base.y + 1,
        },
        Coord {
            x: base.x,
            y: base.y - 1,
        },
        Coord {
            x: base.x,
            y: base.y,
        },
    ]);

    for idx in (0..rtn.len()).rev() {
        if !skip.contains(&rtn[idx])
            && (!rtn[idx].in_bounds(dims) || blizzard_coords.contains(&rtn[idx]))
        {
            rtn.remove(idx);
        }
    }

    rtn
}

fn get_blizzard_positions<'a>(
    blizzard_cache: &'a mut Vec<Vec<Blizzard>>,
    blizzard_coords_cache: &'a mut Vec<HashSet<Coord>>,
    minutes: usize,
    dims: &'a Dims,
) -> &'a HashSet<Coord> {
    let range_start = blizzard_cache.len();
    for _ in range_start..minutes + 1 {
        let mut next_blizzard = (*blizzard_cache.last().unwrap()).clone();
        for next in &mut next_blizzard {
            match next.orientation {
                Orientation::Up => {
                    next.coord.y -= 1;
                    if dims.y_min > next.coord.y {
                        next.coord.y = dims.y_max - 1;
                    }
                }
                Orientation::Down => {
                    next.coord.y += 1;
                    if dims.y_max <= next.coord.y {
                        next.coord.y = dims.y_min;
                    }
                }
                Orientation::Left => {
                    next.coord.x -= 1;
                    if dims.x_min > next.coord.x {
                        next.coord.x = dims.x_max - 1;
                    }
                }
                Orientation::Right => {
                    next.coord.x += 1;
                    if dims.x_max <= next.coord.x {
                        next.coord.x = dims.x_min
                    }
                }
            };
        }

        let coords_collection: Vec<Coord> = next_blizzard.iter().map(|nb| nb.coord).collect();
        let coords: HashSet<Coord> = HashSet::from_iter(coords_collection.into_iter());
        blizzard_coords_cache.push(coords);
        blizzard_cache.push(next_blizzard);
    }

    blizzard_coords_cache.get(minutes).unwrap()
}

fn run(input_file: &str) {
    // Preamble
    let mut start: Coord = Coord { x: 0, y: 0 };
    let mut blizzards: Vec<Blizzard> = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut end_y = 0;
    let mut max_x: i64 = 0;

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    for (y, line) in lines.iter().enumerate() {
        end_y += 1;

        let line = line.trim().to_string();
        if y == 0 {
            start.x = line.chars().position(|c| c == '.').unwrap() as i64;
            max_x = line.len() as i64 - 1;
        }

        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                continue;
            }
            if c != '.' {
                blizzards.push(Blizzard::from_xyo(x, y, c));
            }
        }
    }

    let end_x = lines
        .last()
        .unwrap()
        .trim()
        .chars()
        .position(|c| c == '.')
        .unwrap() as i64;
    let end = Coord {
        x: end_x,
        y: end_y - 1,
    };
    let skip = HashSet::from([start, end]);

    // Prepare
    let dims = Dims {
        x_min: 1,
        y_min: 1,
        x_max: max_x,
        y_max: lines.len() as i64 - 1,
    };
    let start_end_distance = start.distance(&end);
    let mut blizzard_cache: Vec<Vec<Blizzard>> = Vec::new();
    let mut blizzard_coords_cache: Vec<HashSet<Coord>> = Vec::new();

    let coords_collection: Vec<Coord> = blizzards.iter().map(|nb| nb.coord).collect();
    let coords: HashSet<Coord> = HashSet::from_iter(coords_collection.into_iter());
    let mut expedition_cache = HashSet::new();
    blizzard_coords_cache.push(coords);
    blizzard_cache.push(blizzards);

    let start_expedition = Expedition {
        coords: start,
        minutes: 0,
        distance: start_end_distance,
    };

    let mut queue: VecDeque<Expedition> = VecDeque::new();
    queue.push_back(start_expedition);
    // Solve
    let mut result: Option<Expedition> = None;

    let mut last_hightes_minute = 0;
    'outer: while !queue.is_empty() {
        let expedition = queue.pop_front().unwrap();
        let next_minute = expedition.minutes + 1;
        if last_hightes_minute < next_minute {
            last_hightes_minute = next_minute;
            // print!("\r{next_minute}");
        }

        let next_blizzards = get_blizzard_positions(
            &mut blizzard_cache,
            &mut blizzard_coords_cache,
            next_minute,
            &dims,
        );

        // print_map(&expedition, &next_blizzards, &dims);

        let next_positions = get_next_positions(&expedition.coords, &dims, next_blizzards, &skip);

        for next_position in next_positions {
            let distance = next_position.distance(&end);
            if distance == 0 {
                result = Some(expedition);
                break 'outer;
            }
            let next_expedition = Expedition {
                coords: next_position,
                minutes: next_minute,
                distance,
            };
            if expedition_cache.insert(next_expedition) {
                queue.push_back(next_expedition);
            }
        }
        queue.make_contiguous().sort_unstable_by(|a, b| a.cmp(b));
        // queue.make_contiguous().sort_unstable();
    }
    println!();
    // Result
    let minutes = result.unwrap().minutes + 1;
    println!("The expedition needs {minutes} minutes!");
}

fn run2(input_file: &str) {
    // Preamble
    let mut start: Coord = Coord { x: 0, y: 0 };
    let mut blizzards: Vec<Blizzard> = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut end_y = 0;
    let mut max_x: i64 = 0;

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    for (y, line) in lines.iter().enumerate() {
        end_y += 1;

        let line = line.trim().to_string();
        if y == 0 {
            start.x = line.chars().position(|c| c == '.').unwrap() as i64;
            max_x = line.len() as i64 - 1;
        }

        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                continue;
            }
            if c != '.' {
                blizzards.push(Blizzard::from_xyo(x, y, c));
            }
        }
    }

    let end_x = lines
        .last()
        .unwrap()
        .trim()
        .chars()
        .position(|c| c == '.')
        .unwrap() as i64;
    let end = Coord {
        x: end_x,
        y: end_y - 1,
    };
    let skip = HashSet::from([start, end]);

    // Prepare
    let dims = Dims {
        x_min: 1,
        y_min: 1,
        x_max: max_x,
        y_max: lines.len() as i64 - 1,
    };
    let start_end_distance = start.distance(&end);
    let mut blizzard_cache: Vec<Vec<Blizzard>> = Vec::new();
    let mut blizzard_coords_cache: Vec<HashSet<Coord>> = Vec::new();

    let coords_collection: Vec<Coord> = blizzards.iter().map(|nb| nb.coord).collect();
    let coords: HashSet<Coord> = HashSet::from_iter(coords_collection.into_iter());
    let mut expedition_cache = HashSet::new();
    blizzard_coords_cache.push(coords);
    blizzard_cache.push(blizzards);

    let mut queue: VecDeque<Expedition> = VecDeque::new();

    // Solve
    let mut result: Expedition = Expedition {
        coords: start,
        minutes: 0,
        distance: start_end_distance,
    };

    let mut last_hightes_minute = 0;

    let end_points = [end, start, end];

    for (idx, end_point) in end_points.iter().enumerate() {
        let start_expedition = Expedition {
            coords: result.coords,
            minutes: result.minutes,
            distance: result.coords.distance(end_point),
        };
        queue.push_back(start_expedition);

        'outer: while !queue.is_empty() {
            let expedition = queue.pop_front().unwrap();
            let next_minute = expedition.minutes + 1;
            if last_hightes_minute < next_minute {
                last_hightes_minute = next_minute;
                // print!("\r{next_minute}");
            }

            let next_blizzards = get_blizzard_positions(
                &mut blizzard_cache,
                &mut blizzard_coords_cache,
                next_minute,
                &dims,
            );

            // print_map(&expedition, &next_blizzards, &dims);

            let next_positions =
                get_next_positions(&expedition.coords, &dims, next_blizzards, &skip);

            for next_position in next_positions {
                let distance = next_position.distance(end_point);

                let next_expedition = Expedition {
                    coords: next_position,
                    minutes: next_minute,
                    distance,
                };
                if distance == 0 {
                    queue.clear();
                    expedition_cache.clear();
                    result = next_expedition;
                    break 'outer;
                }
                if expedition_cache.insert(next_expedition) {
                    queue.push_back(next_expedition);
                }
            }
            queue.make_contiguous().sort_unstable_by(|a, b| a.cmp(b));
            // queue.make_contiguous().sort_unstable();
        }

        let stand = result.minutes;
        println!(
            "The expedition needed {stand} minutes after round {}!",
            idx + 1
        );
    }
    println!();
    // Result
    println!("The expedition needs {} minutes!", result.minutes);
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
