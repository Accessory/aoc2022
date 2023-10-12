#![feature(array_chunks)]
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;
use utils::get_input_path;

fn peekstep(x: usize, y: usize, o: &Direction, max_x: usize, max_y: usize) -> (usize, usize) {
    match o {
        Direction::UP => (x, (y + max_y - 1) % max_y),
        Direction::RIGHT => ((x + 1) % max_x, y),
        Direction::DOWN => (x, (y + 1) % max_y),
        Direction::LEFT => ((x + max_x - 1) % max_x, y),
    }
}

struct Point {
    pub x: usize,
    pub y: usize,
    pub max_x: usize,
    pub max_y: usize,
    pub o: Direction,
    pub walk: Vec<usize>,
}

impl Point {
    pub(crate) fn walk(&mut self, data: usize, map: &Vec<Vec<char>>) {
        let mut next_x = self.x;
        let mut next_y = self.y;
        for _ in 0..data {
            loop {
                (next_x, next_y) = peekstep(next_x, next_y, &self.o, self.max_x, self.max_y);
                if map[next_y][next_x] != ' ' {
                    break;
                }
            }
            if map[next_y][next_x] == '#' {
                break;
            }
            self.walk.push(next_x);
            self.walk.push(next_y);
            self.x = next_x;
            self.y = next_y;
        }
    }
    pub(crate) fn turn(&mut self, data: usize) {
        if data == L {
            self.o = self.o.left();
        } else {
            self.o = self.o.right();
        }
    }

    // fn peekstep(&self) -> (usize, usize) {
    //     peekstep(self.x, self.y, &self.o, self.max_x, self.max_y)
    // }

    pub(crate) fn calc_points(&self) -> usize {
        self.o.points() + 1000 * (self.y + 1) + 4 * (self.x + 1)
    }
}

enum InstructionType {
    WALK,
    TURN,
}

struct Instruction {
    pub instruction_type: InstructionType,
    pub data: usize,
}

const R: usize = 0;
const L: usize = 1;

impl Instruction {
    fn new_walk(data: usize) -> Self {
        Self {
            instruction_type: InstructionType::WALK,
            data,
        }
    }

    fn new_turn(turn: char) -> Self {
        Self {
            instruction_type: InstructionType::TURN,
            data: if turn == 'R' { R } else { L },
        }
    }
}

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::UP => Direction::LEFT,
            Direction::RIGHT => Direction::UP,
            Direction::DOWN => Direction::RIGHT,
            Direction::LEFT => Direction::DOWN,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }

    fn points(&self) -> usize {
        match self {
            Direction::UP => 3,
            Direction::RIGHT => 0,
            Direction::DOWN => 1,
            Direction::LEFT => 2,
        }
    }
}

fn run(input_file: &str) {
    // Preamble
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut instructions = Vec::new();
    let mut max_x = 0;

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let mut is_last_line = false;
    let mut last_line: Option<String> = None;

    for line in reader.lines() {
        let line = line.unwrap().replace("\r", "");

        if line.is_empty() {
            is_last_line = true;
            continue;
        }
        if is_last_line {
            last_line = Some(line);
            break;
        }

        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        max_x = max_x.max(row.len());
        map.push(row);
    }

    let mut start = 0;

    let line = last_line.unwrap();
    for (idx, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            continue;
        }

        let split = &line[start..idx];

        let number: usize = split.parse().unwrap();
        instructions.push(Instruction::new_walk(number));

        instructions.push(Instruction::new_turn(c));
        start = idx + 1;
    }
    let split = &line[start..];
    let number: usize = split.parse().unwrap();
    instructions.push(Instruction::new_walk(number));

    // Prepare
    let max_y = map.len();

    for row in map.iter_mut() {
        for _ in row.len()..max_x {
            row.push(' ');
        }
    }

    let mut you = Point {
        x: 0,
        y: 0,
        max_x: max_x,
        max_y: max_y,
        o: Direction::RIGHT,
        walk: Vec::new(),
    };

    'outer: for y in 0..max_y {
        for x in 0..max_x {
            if map[y][x] == '.' {
                you.x = x;
                you.y = y;
                break 'outer;
            }
        }
    }

    // Solve
    for instruction in instructions {
        match instruction.instruction_type {
            InstructionType::WALK => you.walk(instruction.data, &map),
            InstructionType::TURN => you.turn(instruction.data),
        }
    }

    draw_map(&mut map, &you);
    // Result
    let result = you.calc_points();
    println!("Result is {}", result);
}

fn draw_map(map: &mut Vec<Vec<char>>, you: &Point) {
    for [x, y] in you.walk.array_chunks() {
        map[*y][*x] = 'x';
    }
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!("\n");
    }
}

fn run2(input_file: &str) {
    // Premable
    let mut max_x = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    for l in &lines {
        if l.trim().is_empty() {
            break;
        }
        let mut row: Vec<char> = Vec::new();
        for c in l.trim_end().chars() {
            row.push(c);
        }
        max_x = max_x.max(row.len());
        grid.push(row);
    }

    let sequence = lines.last().unwrap().to_owned();

    // Prepare
    let mut r: i64 = 0;
    let mut c: i64 = 0;
    let mut dr: i64 = 0;
    let mut dc: i64 = 1;

    while grid[r as usize][c as usize] != '.' {
        c += 1;
    }

    let rgx: Regex = Regex::new(r#"(\d+)([RL]?)"#).unwrap();
    let captures = rgx.captures_iter(&sequence);

    for capture in captures {
        let x: usize = capture[1].parse().unwrap();
        for _ in 0..x {
            let cdr: i64 = dr;
            let cdc: i64 = dc;
            let mut nr: i64 = r + dr;
            let mut nc: i64 = c + dc;

            if nr < 0 && 50 <= nc && nc < 100 && dr == -1 {
                (dr, dc) = (0, 1);
                (nr, nc) = (nc + 100, 0);
            } else if nc < 0 && 150 <= nr && nr < 200 && dc == -1 {
                (dr, dc) = (1, 0);
                (nr, nc) = (0, nr - 100);
            } else if nr < 0 && 100 <= nc && nc < 150 && dr == -1 {
                (nr, nc) = (199, nc - 100);
            } else if nr >= 200 && 0 <= nc && nc < 50 && dr == 1 {
                (nr, nc) = (0, nc + 100);
            } else if nc >= 150 && 0 <= nr && nr < 50 && dc == 1 {
                dc = -1;
                (nr, nc) = (149 - nr, 99);
            } else if nc == 100 && 100 <= nr && nr < 150 && dc == 1 {
                dc = -1;
                (nr, nc) = (149 - nr, 149);
            } else if nr == 50 && 100 <= nc && nc < 150 && dr == 1 {
                (dr, dc) = (0, -1);
                (nr, nc) = (nc - 50, 99);
            } else if nc == 100 && 50 <= nr && nr < 100 && dc == 1 {
                (dr, dc) = (-1, 0);
                (nr, nc) = (49, nr + 50);
            } else if nr == 150 && 50 <= nc && nc < 100 && dr == 1 {
                (dr, dc) = (0, -1);
                (nr, nc) = (nc + 100, 49);
            } else if nc == 50 && 150 <= nr && nr < 200 && dc == 1 {
                (dr, dc) = (-1, 0);
                (nr, nc) = (149, nr - 100);
            } else if nr == 99 && 0 <= nc && nc < 50 && dr == -1 {
                (dr, dc) = (0, 1);
                (nr, nc) = (nc + 50, 50);
            } else if nc == 49 && 50 <= nr && nr < 100 && dc == -1 {
                (dr, dc) = (1, 0);
                (nr, nc) = (100, nr - 50);
            } else if nc == 49 && 0 <= nr && nr < 50 && dc == -1 {
                dc = 1;
                (nr, nc) = (149 - nr, 0);
            } else if nc < 0 && 100 <= nr && nr < 150 && dc == -1 {
                dc = 1;
                (nr, nc) = (149 - nr, 50);
            }

            if grid[nr as usize][nc as usize] == '#' {
                dr = cdr;
                dc = cdc;
                break;
            }
            r = nr;
            c = nc;
        }
       
        let y = &capture[2];
        if y == "R" {
            (dr, dc) = (dc, -dr);
        } else if y == "L" {
            (dr, dc) = (-dc, dr);
        }
    }

    let k;

    if dr == 0 {
        if dc == 1 {
            k = 0;
        } else {
            k = 2;
        }
    } else {
        if dr == 1 {
            k = 1;
        } else {
            k = 3
        }
    }

    println!("Result 2 {}", 1000 * (r + 1) + 4 * (c + 1) + k);
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
    use utils::get_input_path;
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
        let input_path = get_input_path(file!());
        run2(input_path.to_str().unwrap());
    }
}
