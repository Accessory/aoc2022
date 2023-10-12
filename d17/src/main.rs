use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, Read};

use utils::get_input_path;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Point {
    pub x: u64,
    pub y: u64,
}
#[derive(Clone)]
struct Rock {
    pub blocks: Vec<Point>,
}

#[derive(Clone)]
struct Grid {
    pub rows: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new() -> Grid {
        let mut row = Vec::new();
        row.push(Vec::from([true, true, true, true, true, true, true]));
        Grid { rows: row }
    }
    pub fn fill_grid_upto(&mut self, y: u64) {
        for _ in self.rows.len()..(y as usize + 1) {
            self.rows
                .push(Vec::from([false, false, false, false, false, false, false]))
        }
    }

    pub(crate) fn hits(&self, rock: &Rock) -> bool {
        for point in &rock.blocks {
            if point.x == 0 || point.x == 8 {
                return true;
            }
            if self.rows[point.y as usize][point.x as usize - 1] {
                return true;
            }
        }
        return false;
    }

    pub(crate) fn insert_rock(&mut self, rock: &Rock) {
        for point in &rock.blocks {
            self.rows[point.y as usize][point.x as usize - 1] = true;
        }
    }

    pub(crate) fn remove_rock(&mut self, rock: &Rock) {
        for point in &rock.blocks {
            self.rows[point.y as usize][point.x as usize - 1] = false;
        }
    }

    #[allow(dead_code)]
    pub(crate) fn print(&self) {
        for (idx, row) in self.rows.iter().rev().enumerate() {
            print!("{:<6}|", self.rows.len() - idx - 1);
            for coulmn in row {
                let c = if *coulmn { '#' } else { '.' };
                print!("{}", c);
            }
            println!("|");
        }
    }

    pub(crate) fn get_highest_row(&self) -> u64 {
        for (idx, row) in self.rows.iter().rev().enumerate() {
            for coulmn in row {
                if *coulmn {
                    return (self.rows.len() - idx - 1) as u64;
                }
            }
        }
        return 0;
    }
}

impl Rock {
    pub fn get_high_y(&self) -> u64 {
        self.blocks.last().unwrap().y
    }
    // pub fn get_low_y(&self) {
    //     self.blocks.first().unwrap().y;
    // }

    pub(crate) fn move_down(&self) -> Rock {
        let mut rtn = self.clone();
        rtn.blocks.iter_mut().for_each(|point| {
            point.y -= 1;
        });
        return rtn;
    }

    pub(crate) fn move_left(&self) -> Rock {
        let mut rtn = self.clone();
        rtn.blocks.iter_mut().for_each(|point| {
            point.x -= 1;
        });
        return rtn;
    }

    pub(crate) fn move_right(&self) -> Rock {
        let mut rtn = self.clone();
        rtn.blocks.iter_mut().for_each(|point| {
            point.x += 1;
        });
        return rtn;
    }
}

fn create_line(lo_y: u64) -> Rock {
    Rock {
        blocks: Vec::from([
            Point { x: 3, y: lo_y },
            Point { x: 4, y: lo_y },
            Point { x: 5, y: lo_y },
            Point { x: 6, y: lo_y },
        ]),
    }
}

fn create_long(lo_y: u64) -> Rock {
    Rock {
        blocks: Vec::from([
            Point { x: 3, y: lo_y },
            Point { x: 3, y: lo_y + 1 },
            Point { x: 3, y: lo_y + 2 },
            Point { x: 3, y: lo_y + 3 },
        ]),
    }
}

fn create_plus(lo_y: u64) -> Rock {
    Rock {
        blocks: Vec::from([
            Point { x: 4, y: lo_y },
            Point { x: 3, y: lo_y + 1 },
            Point { x: 4, y: lo_y + 1 },
            Point { x: 5, y: lo_y + 1 },
            Point { x: 4, y: lo_y + 2 },
        ]),
    }
}

fn create_reverse_l(lo_y: u64) -> Rock {
    Rock {
        blocks: Vec::from([
            Point { x: 3, y: lo_y },
            Point { x: 4, y: lo_y },
            Point { x: 5, y: lo_y },
            Point { x: 5, y: lo_y + 1 },
            Point { x: 5, y: lo_y + 2 },
        ]),
    }
}

fn create_block(lo_y: u64) -> Rock {
    Rock {
        blocks: Vec::from([
            Point { x: 3, y: lo_y },
            Point { x: 4, y: lo_y },
            Point { x: 3, y: lo_y + 1 },
            Point { x: 4, y: lo_y + 1 },
        ]),
    }
}

#[derive()]
enum RockEnum {
    Line,
    Plus,
    ReverseL,
    Long,
    Block,
}

impl RockEnum {
    pub fn create_rock(&self, hi_y: u64) -> Rock {
        match self {
            RockEnum::Line => create_line(hi_y),
            RockEnum::Plus => create_plus(hi_y),
            RockEnum::ReverseL => create_reverse_l(hi_y),
            RockEnum::Long => create_long(hi_y),
            RockEnum::Block => create_block(hi_y),
        }
    }

    pub fn next(&self) -> RockEnum {
        match self {
            RockEnum::Line => RockEnum::Plus,
            RockEnum::Plus => RockEnum::ReverseL,
            RockEnum::ReverseL => RockEnum::Long,
            RockEnum::Long => RockEnum::Block,
            RockEnum::Block => RockEnum::Line,
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    const STOP_AFTER: usize = 2022;

    // Parse
    let file = File::open(input_file).unwrap();
    let mut reader = BufReader::new(file);

    let mut streams = Vec::new();
    reader
        .read_to_end(&mut streams)
        .expect("Failed to read file");

    // Prepare
    let mut stream_pos = 0;
    let mut rock = RockEnum::Line;
    let mut rocks_stopped: usize = 0;
    let mut grid: Grid = Grid::new();
    let mut current_high: u64 = grid.get_highest_row();

    const LEFT: u8 = b'<';

    // Solve
    while rocks_stopped < STOP_AFTER {
        // Start
        let mut current_rock = rock.create_rock(current_high + 4);
        grid.fill_grid_upto(current_rock.get_high_y());
        loop {
            let mut next_block;

            // Stream
            if streams.get(stream_pos).unwrap() == &LEFT {
                next_block = current_rock.move_left();
            } else {
                next_block = current_rock.move_right();
            }

            if !grid.hits(&next_block) {
                current_rock = next_block;
            }

            stream_pos += 1;
            stream_pos = stream_pos % streams.len();

            //  Move down
            next_block = current_rock.move_down();
            if grid.hits(&next_block) {
                grid.insert_rock(&current_rock);
                break;
            }
            current_rock = next_block;
        }

        // Next round
        current_high = grid.get_highest_row();
        rocks_stopped += 1;

        // println!("{}, {}", rocks_stopped, current_high);
        rock = rock.next();
    }

    // Result
    // grid.print();
    println!("Result is {}", current_high);
}

fn run2(input_file: &str) {
    // Preamble
    const STOP_AFTER: usize = 1000000000000;

    // Parse
    let file = File::open(input_file).unwrap();
    let mut reader = BufReader::new(file);

    let mut streams = Vec::new();
    reader
        .read_to_end(&mut streams)
        .expect("Failed to read file");

    // Prepare
    let mut stream_pos = 0;
    let mut rock = RockEnum::Line;
    let mut rocks_stopped: usize = 0;
    let mut grid: Grid = Grid::new();
    let mut current_high: u64 = grid.get_highest_row();

    const LEFT: u8 = b'<';
    let mut hash_list: Vec<u64> = Vec::new();
    let mut last_hash = 1;
    let mut hash_hight_list = Vec::new();
    let mut hash_rocks_list = Vec::new();
    let mut skipped = 0;

    // Solve
    while rocks_stopped < STOP_AFTER {
        // Start
        let mut current_rock = rock.create_rock(current_high + 4);
        grid.fill_grid_upto(current_rock.get_high_y());
        loop {
            let mut next_block;

            // Stream
            if streams.get(stream_pos).unwrap() == &LEFT {
                next_block = current_rock.move_left();
            } else {
                next_block = current_rock.move_right();
            }

            if !grid.hits(&next_block) {
                current_rock = next_block;
            }

            stream_pos += 1;
            if stream_pos == streams.len() {
                let mut hasher: DefaultHasher = DefaultHasher::new();
                grid.insert_rock(&current_rock);
                let slice = &grid.rows[last_hash..];
                slice.hash(&mut hasher);
                let hash = hasher.finish();
                if skipped == 0 {
                    if let Some(idx) = hash_list.iter().position(|t| &hash == t) {
                        let hight_at_idx = hash_hight_list.get(idx).unwrap();
                        let hight_delta = current_high - hight_at_idx;
                        let rocks_stopped_at_idx = hash_rocks_list.get(idx).unwrap();
                        let skipped_blocks_per_itr = rocks_stopped - rocks_stopped_at_idx;

                        let skipped_iterations =
                            (STOP_AFTER - rocks_stopped) / skipped_blocks_per_itr;

                        let next_block_position = skipped_blocks_per_itr * skipped_iterations;
                        rocks_stopped += next_block_position;
                        skipped = hight_delta * skipped_iterations as u64;
                    }
                }
                hash_hight_list.push(current_high);
                hash_rocks_list.push(rocks_stopped);
                grid.remove_rock(&current_rock);
                hash_list.push(hash);
                last_hash = current_high as usize + 1;

                stream_pos = 0;
            }

            //  Move down
            next_block = current_rock.move_down();
            if grid.hits(&next_block) {
                grid.insert_rock(&current_rock);
                break;
            }
            current_rock = next_block;
        }

        // Next round
        current_high = grid.get_highest_row();
        rocks_stopped += 1;

        // println!("{}, {}", rocks_stopped, current_high + skipped);
        rock = rock.next();
    }

    // Result
    // grid.print();
    println!("Result is {}", current_high + skipped);
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
