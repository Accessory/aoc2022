use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;
use utils::get_input_path;

struct Beacon {
    pub x: i64,
    pub y: i64,
}

struct Sensor {
    pub x: i64,
    pub y: i64,
}

#[derive(PartialEq, Eq, PartialOrd)]
struct Interval {
    pub lo: i64,
    pub hi: i64,
}
impl Interval {
    pub(crate) fn delta(&self) -> i64 {
        self.hi - self.lo + 1
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.lo.cmp(&other.lo)
    }
}

struct SensorBeaconGroup {
    pub beacon: Beacon,
    pub sensor: Sensor,
    distance: Option<i64>,
}

impl SensorBeaconGroup {
    pub fn sensor_beacon_distance(&mut self) -> i64 {
        if self.distance.is_none() {
            self.distance =
                Some((self.sensor.x - self.beacon.x).abs() + (self.sensor.y - self.beacon.y).abs());
        }
        return self.distance.unwrap();
    }

    pub fn get_min_y(&self) -> i64 {
        self.sensor.y.min(self.beacon.y)
    }

    pub fn get_min_x(&self) -> i64 {
        self.sensor.x.min(self.beacon.x)
    }

    pub fn get_max_y(&self) -> i64 {
        self.sensor.y.max(self.beacon.y)
    }

    pub fn get_max_x(&self) -> i64 {
        self.sensor.x.max(self.beacon.x)
    }

    pub(crate) fn distance_to_y(&self, y: i64) -> i64 {
        (self.sensor.y - y).abs()
    }
}

fn intersect(row_to_check: i64, lo: i64, hi: i64, x: i64, y: i64) -> bool {
    if row_to_check == y {
        return x >= lo && x <= hi;
    }
    return false;
}

fn run(input_file: &str) {
    // Preamble
    let mut min_x: i64 = i64::MAX;
    let mut min_y: i64 = i64::MAX;
    let mut max_x: i64 = i64::MIN;
    let mut max_y: i64 = i64::MIN;
    let mut max_distance = i64::MIN;
    let mut sensor_beacons = Vec::new();
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let rgx = Regex::new(
        "Sensor at x=([-0-9]+), y=([-0-9]+): closest beacon is at x=([-0-9]+), y=([-0-9]+)",
    )
    .unwrap();
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let captures = rgx.captures(&line).unwrap();
        let sensor = Sensor {
            x: captures[1].parse().unwrap(),
            y: captures[2].parse().unwrap(),
        };
        let beacon = Beacon {
            x: captures[3].parse().unwrap(),
            y: captures[4].parse().unwrap(),
        };
        let mut sensor_beacon_group = SensorBeaconGroup {
            beacon,
            sensor,
            distance: None,
        };
        min_x = min_x.min(sensor_beacon_group.get_min_x());
        min_y = min_y.min(sensor_beacon_group.get_min_y());
        max_x = max_x.max(sensor_beacon_group.get_max_x());
        max_y = max_y.max(sensor_beacon_group.get_max_y());
        max_distance = max_distance.max(sensor_beacon_group.sensor_beacon_distance());
        sensor_beacons.push(sensor_beacon_group);
    }

    // Prepare
    // let min_working_x: i64 = min_x - max_distance;
    // let min_working_y: i64 = min_y - max_distance;
    // let max_working_x: i64 = max_x + max_distance;
    // let max_working_y: i64 = max_y + max_distance;
    let row_to_check: i64 = if sensor_beacons.len() == 14 {
        10
    } else {
        2000000
    };

    // Solve
    let mut intervals: Vec<Interval> = Vec::new();
    for sensor_beacon in sensor_beacons.iter_mut() {
        let sensor_distance = sensor_beacon.distance_to_y(row_to_check);

        if sensor_distance > sensor_beacon.sensor_beacon_distance() {
            continue;
        }

        let singal_half_length = sensor_beacon.sensor_beacon_distance() - sensor_distance;

        let lo = sensor_beacon.sensor.x - singal_half_length;
        let hi = sensor_beacon.sensor.x + singal_half_length;

        intervals.push(Interval { lo, hi });
    }

    intervals.sort_unstable();

    let mut q: Vec<Interval> = Vec::new();
    for interval in intervals {
        let lo = interval.lo;
        let hi = interval.hi;

        if q.is_empty() {
            q.push(interval);
            continue;
        }

        let q_hi = q.last().unwrap().hi;

        if lo > q_hi {
            q.push(interval);
            continue;
        }

        q.last_mut().unwrap().hi = max(hi, q_hi);
    }

    // Result
    let mut result = 0;
    let mut skip_x: HashSet<i64> = HashSet::new();
    for i in q {
        result += i.delta();
        for sensor_beacon in sensor_beacons.iter() {
            if !skip_x.contains(&sensor_beacon.beacon.x)
                && intersect(
                    row_to_check,
                    i.lo,
                    i.hi,
                    sensor_beacon.beacon.x,
                    sensor_beacon.beacon.y,
                )
            {
                skip_x.insert(sensor_beacon.beacon.x);
                result -= 1;
            }
        }
    }

    println!("Result is {}", result);
}

fn run2(input_file: &str) {
    // Preamble
    let mut result = i64::MIN;
    let mut min_x: i64 = i64::MAX;
    let mut min_y: i64 = i64::MAX;
    let mut max_x: i64 = i64::MIN;
    let mut max_y: i64 = i64::MIN;
    let mut max_distance = i64::MIN;
    let mut sensor_beacons = Vec::new();
    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let rgx = Regex::new(
        "Sensor at x=([-0-9]+), y=([-0-9]+): closest beacon is at x=([-0-9]+), y=([-0-9]+)",
    )
    .unwrap();
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        let captures = rgx.captures(&line).unwrap();
        let sensor = Sensor {
            x: captures[1].parse().unwrap(),
            y: captures[2].parse().unwrap(),
        };
        let beacon = Beacon {
            x: captures[3].parse().unwrap(),
            y: captures[4].parse().unwrap(),
        };
        let mut sensor_beacon_group = SensorBeaconGroup {
            beacon,
            sensor,
            distance: None,
        };
        min_x = min_x.min(sensor_beacon_group.get_min_x());
        min_y = min_y.min(sensor_beacon_group.get_min_y());
        max_x = max_x.max(sensor_beacon_group.get_max_x());
        max_y = max_y.max(sensor_beacon_group.get_max_y());
        max_distance = max_distance.max(sensor_beacon_group.sensor_beacon_distance());
        sensor_beacons.push(sensor_beacon_group);
    }

    // Prepare

    let check_range: i64 = if sensor_beacons.len() == 14 {
        20
    } else {
        4000000
    };

    let min_working_x: i64 = 0;
    let min_working_y: i64 = 0;
    let max_working_x: i64 = check_range;
    let max_working_y: i64 = check_range;

    // Solve
    for y in min_working_y..max_working_y + 1 {
        let mut intervals: Vec<Interval> = Vec::new();
        for sensor_beacon in sensor_beacons.iter_mut() {
            let sensor_distance = sensor_beacon.distance_to_y(y);

            if sensor_distance > sensor_beacon.sensor_beacon_distance() {
                continue;
            }

            let singal_half_length = sensor_beacon.sensor_beacon_distance() - sensor_distance;

            let lo = (sensor_beacon.sensor.x - singal_half_length).max(min_working_x);
            let hi = (sensor_beacon.sensor.x + singal_half_length).min(max_working_x);

            intervals.push(Interval { lo, hi });
        }

        intervals.sort_unstable();

        let mut q: Vec<Interval> = Vec::new();
        for interval in intervals {
            let lo = interval.lo;
            let hi = interval.hi;

            if q.is_empty() {
                q.push(interval);
                continue;
            }

            let q_hi = q.last().unwrap().hi;

            if lo > q_hi {
                q.push(interval);
                continue;
            }

            q.last_mut().unwrap().hi = max(hi, q_hi);
        }

        if q.len() == 1 {
            continue;
        } else {
           result = ((q.first().unwrap().hi+1) * 4000000) +  y;
           break;
        }
    }
    
    // Result
    if result == i64::MIN {
        println!("Failed to find anything");
    } else {
        println!("Found signal with frequenze {}", result);
    }
    
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
