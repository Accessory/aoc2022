use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;
use utils::get_input_path;

struct Valve {
    pub name: String,
    pub rate: u64,
    pub to_valves: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct CacheObject {
    pub time: i64,
    pub valve: String,
    pub bitmask: u64,
}

struct DistanceObject {
    pub distance: i64,
    pub valve_name: String,
}

fn dfs(
    time: i64,
    valve: String,
    bitmask: u64,
    cache: &mut HashMap<CacheObject, u64>,
    valve_map: &HashMap<String, Valve>,
    idx_map: &HashMap<String, u64>,
    distance: &HashMap<String, HashMap<String, i64>>,
) -> u64 {
    let cache_object = CacheObject {
        time,
        valve: valve.clone(),
        bitmask,
    };

    // println!("Current Setting: {:?}", cache_object);
    let current_cache = cache.get(&cache_object);

    if current_cache.is_some() {
        return current_cache.unwrap().clone();
    }

    let mut max_val: u64 = 0;
    let current_valve = valve_map.get(&valve).unwrap();

    for neighbor in distance.get(&current_valve.name).unwrap().keys() {
        let idx = idx_map.get(neighbor).unwrap();
        let bit: u64 = 1 << idx;
        if (bitmask & bit) != 0 {
            continue;
        }
        let neighbor_valve = valve_map.get(neighbor).unwrap();
        let neighbor_distance = distance.get(&valve).unwrap().get(neighbor).unwrap();
        let rem_time = time - neighbor_distance - 1;
        if rem_time <= 0 {
            continue;
        }

        let new_bitmask = bitmask | bit;
        let new_max_value = dfs(
            rem_time,
            neighbor.clone(),
            new_bitmask,
            cache,
            valve_map,
            idx_map,
            distance,
        ) + rem_time as u64 * neighbor_valve.rate;
        max_val = max(max_val, new_max_value)
    }
    cache.insert(cache_object, max_val);
    return max_val;
}

fn run(input_file: &str) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut valve_map: HashMap<String, Valve> = HashMap::new();
    let mut valve_list: Vec<String> = Vec::new();
    let mut cache: HashMap<CacheObject, u64> = HashMap::new();
    let mut distance: HashMap<String, HashMap<String, i64>> = HashMap::new();

    let start_valve = String::from("AA");
    let time = 30;

    // Parse
    for (_index, line) in reader.lines().enumerate() {
        let trimmed_line = line.unwrap().trim().to_string();

        if trimmed_line.is_empty() {
            continue;
        }
        let rgx =
            Regex::new(r#"Valve (\w\w) has flow rate=(\d\d?); tunnels? leads? to valves? (.+)"#)
                .unwrap();

        let captures = rgx.captures(&trimmed_line).unwrap();
        let valve: &str = &captures[1];
        let rate: &u64 = &captures[2].parse().unwrap();
        let lead_to: &str = &captures[3];

        let splits = lead_to.split(", ");
        let mut to_valves: Vec<String> = Vec::new();

        for s in splits {
            to_valves.push(s.to_string());
        }

        let valve = Valve {
            name: valve.to_string(),
            rate: rate.clone(),
            to_valves: to_valves,
        };

        valve_list.push(valve.name.clone());
        valve_map.insert(valve.name.clone(), valve);
    }

    // Prepare
    // Distance
    let mut non_empty: Vec<String> = Vec::new();
    for key in &valve_list {
        if key != &start_valve && valve_map.get(key).unwrap().rate == 0 {
            continue;
        }

        if key != &start_valve {
            non_empty.push(key.clone());
        }

        distance.insert(key.clone(), HashMap::from([(start_valve.clone(), 0)]));
        let mut visited: HashSet<String> = HashSet::from([key.clone()]);

        let mut queue: VecDeque<DistanceObject> = VecDeque::from([DistanceObject {
            distance: 0,
            valve_name: key.clone(),
        }]);

        while !queue.is_empty() {
            let distance_object = queue.pop_front().unwrap();
            let current_distance = distance_object.distance;
            let valve_name = distance_object.valve_name;

            let current_valve = valve_map.get(&valve_name).unwrap();

            for neighbor in &current_valve.to_valves {
                if visited.contains(neighbor) {
                    continue;
                }
                visited.insert(neighbor.clone());
                let neighbor_valve = valve_map.get(neighbor).unwrap();
                if neighbor_valve.rate != 0 {
                    distance
                        .get_mut(key)
                        .unwrap()
                        .insert(neighbor.clone(), current_distance + 1);
                }
                queue.push_back(DistanceObject {
                    distance: current_distance + 1,
                    valve_name: neighbor.clone(),
                })
            }
        }

        distance.get_mut(key).unwrap().remove(key);
        if key != &start_valve {
            distance.get_mut(key).unwrap().remove(&start_valve);
        }
    }

    // Index
    let mut idx_map: HashMap<String, u64> = HashMap::new();
    for (i, value) in non_empty.iter().enumerate() {
        idx_map.insert(value.clone(), i.try_into().unwrap());
    }

    // Solve
    let result = dfs(
        time,
        start_valve.to_string(),
        0,
        &mut cache,
        &valve_map,
        &idx_map,
        &distance,
    );

    println!("The end result is: {}", result);
}

fn run2(input_file: &str) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let mut valve_map: HashMap<String, Valve> = HashMap::new();
    let mut valve_list: Vec<String> = Vec::new();
    let mut cache: HashMap<CacheObject, u64> = HashMap::new();
    let mut distance: HashMap<String, HashMap<String, i64>> = HashMap::new();

    let start_valve = String::from("AA");
    let time = 26;

    // Parse
    for (_index, line) in reader.lines().enumerate() {
        let trimmed_line = line.unwrap().trim().to_string();

        if trimmed_line.is_empty() {
            continue;
        }
        let rgx =
            Regex::new(r#"Valve (\w\w) has flow rate=(\d\d?); tunnels? leads? to valves? (.+)"#)
                .unwrap();

        let captures = rgx.captures(&trimmed_line).unwrap();
        let valve: &str = &captures[1];
        let rate: &u64 = &captures[2].parse().unwrap();
        let lead_to: &str = &captures[3];

        let splits = lead_to.split(", ");
        let mut to_valves: Vec<String> = Vec::new();

        for s in splits {
            to_valves.push(s.to_string());
        }

        let valve = Valve {
            name: valve.to_string(),
            rate: rate.clone(),
            to_valves: to_valves,
        };

        valve_list.push(valve.name.clone());
        valve_map.insert(valve.name.clone(), valve);
    }

    // Prepare
    // Distance
    let mut non_empty: Vec<String> = Vec::new();
    for key in &valve_list {
        if key != &start_valve && valve_map.get(key).unwrap().rate == 0 {
            continue;
        }

        if key != &start_valve {
            non_empty.push(key.clone());
        }

        distance.insert(key.clone(), HashMap::from([(start_valve.clone(), 0)]));
        let mut visited: HashSet<String> = HashSet::from([key.clone()]);

        let mut queue: VecDeque<DistanceObject> = VecDeque::from([DistanceObject {
            distance: 0,
            valve_name: key.clone(),
        }]);

        while !queue.is_empty() {
            let distance_object = queue.pop_front().unwrap();
            let current_distance = distance_object.distance;
            let valve_name = distance_object.valve_name;

            let current_valve = valve_map.get(&valve_name).unwrap();

            for neighbor in &current_valve.to_valves {
                if visited.contains(neighbor) {
                    continue;
                }
                visited.insert(neighbor.clone());
                let neighbor_valve = valve_map.get(neighbor).unwrap();
                if neighbor_valve.rate != 0 {
                    distance
                        .get_mut(key)
                        .unwrap()
                        .insert(neighbor.clone(), current_distance + 1);
                }
                queue.push_back(DistanceObject {
                    distance: current_distance + 1,
                    valve_name: neighbor.clone(),
                })
            }
        }

        distance.get_mut(key).unwrap().remove(key);
        if key != &start_valve {
            distance.get_mut(key).unwrap().remove(&start_valve);
        }
    }

    // Index
    let mut idx_map: HashMap<String, u64> = HashMap::new();
    for (i, value) in non_empty.iter().enumerate() {
        idx_map.insert(value.clone(), i.try_into().unwrap());
    }

    // Solve
    let b = (1 << non_empty.len()) - 1;
    let mut result = 0;
    let end_range = (b + 1) / 2;
    // Range optimization for the case that it is more effective when both are working.
    let start_range =( end_range as f64* 0.4) as u64;
    let reduced_end_range =( end_range as f64* 0.7) as u64;


    for i in start_range..reduced_end_range {
        // let percentage = (i-start_range) * 100 / (reduced_end_range-start_range - 1);
        // print!(
        //     "\rStart solving {} of {} or {}%",
        //     i-start_range,
        //     reduced_end_range-start_range - 1,
        //     percentage
        // );

        let r1 = dfs(
            time,
            start_valve.to_string(),
            i,
            &mut cache,
            &valve_map,
            &idx_map,
            &distance,
        );
        let r2 = dfs(
            time,
            start_valve.to_string(),
            b ^ i,
            &mut cache,
            &valve_map,
            &idx_map,
            &distance,
        );
        let rtry = r1 + r2;
        result = result.max(rtry);
    }

    println!();
    println!("The end result is: {}", result);
}

fn main() {
    let input_path = get_input_path(file!());
    let input_file = input_path.to_str().unwrap();

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
