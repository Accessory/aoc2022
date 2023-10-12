use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

use regex::Regex;
use utils::get_input_path;

#[derive(Hash, PartialEq, Eq, Clone)]
struct RobotSet {
    pub ore: u64,
    pub clay: u64,
    pub obsidian: u64,
    pub geode: u64,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct MineralSet {
    pub ore: u64,
    pub clay: u64,
    pub obsidian: u64,
    pub geode: u64,
}

#[derive(Hash, PartialEq, Eq)]
struct CacheKey {
    pub robot_key: u64,
    pub mineral_key: u64,
    pub time: i64,
}

#[derive(Debug)]
struct Blueprint {
    pub blueprint_number: u64,
    pub ore_robot_ore_cost: u64,
    pub clay_robot_ore_cost: u64,
    pub obsidian_robot_ore_cost: u64,
    pub obsidian_robot_clay_cost: u64,
    pub geode_robot_ore_cost: u64,
    pub geode_robot_obsidian_cost: u64,
    pub max_spend: MineralSet,
}

fn calc_ceil_of(blueprint_resource: &u64, remaining_resource: &u64, robot_amount: &u64) -> i64 {
    f64::ceil((*blueprint_resource as f64 - *remaining_resource as f64) / *robot_amount as f64)
        as i64
}

// With nasty speedup hacks like '&& robots.obsidian <= 4'
fn dfs(
    blueprint: &Blueprint,
    cache: &mut HashMap<CacheKey, u64>,
    time: i64,
    robots: RobotSet,
    minerals: MineralSet,
) -> u64 {
    //  Is at the end
    if time == 0 {
        return minerals.geode;
    }

    // Caching
    let hash_key = CacheKey {
        robot_key: calculate_hash_robot(&robots),
        mineral_key: calculate_hash_minerals(&minerals),
        time,
    };

    if cache.contains_key(&hash_key) {
        return cache.get(&hash_key).unwrap().clone();
    }

    // Preparing
    let mut max_val = minerals.geode + robots.geode * time as u64;

    // Walking

    if robots.obsidian > 0 {
        let wait_time = i64::max(
            0,
            calc_ceil_of(&blueprint.geode_robot_ore_cost, &minerals.ore, &robots.ore),
        )
        .max(calc_ceil_of(
            &blueprint.geode_robot_obsidian_cost,
            &minerals.obsidian,
            &robots.obsidian,
        )) + 1;

        let rem_time: i64 = time - wait_time as i64;
        if rem_time > 0 {
            let mut new_minerals = update_minerals(minerals.clone(), &robots, &(wait_time as u64));
            new_minerals.ore -= blueprint.geode_robot_ore_cost;
            new_minerals.obsidian -= blueprint.geode_robot_obsidian_cost;

            let mut new_robots = robots.clone();
            new_robots.geode += 1;

            max_val = max_val.max(dfs(blueprint, cache, rem_time, new_robots, new_minerals));
        }
    }

    if robots.obsidian < blueprint.max_spend.obsidian && robots.clay > 0 && robots.geode <= 4  {
        let wait_time = i64::max(
            0,
            calc_ceil_of(
                &blueprint.obsidian_robot_ore_cost,
                &minerals.ore,
                &robots.ore,
            ),
        )
        .max(calc_ceil_of(
            &blueprint.obsidian_robot_clay_cost,
            &minerals.clay,
            &robots.clay,
        )) + 1;
        let rem_time: i64 = time - wait_time as i64;
        if rem_time > 0 {
            let mut new_minerals = update_minerals(minerals.clone(), &robots, &(wait_time as u64));
            new_minerals.ore -= blueprint.obsidian_robot_ore_cost;
            new_minerals.clay -= blueprint.obsidian_robot_clay_cost;

            let mut new_robots = robots.clone();
            new_robots.obsidian += 1;

            max_val = max_val.max(dfs(blueprint, cache, rem_time, new_robots, new_minerals));
        }
    }

    if robots.clay < blueprint.max_spend.clay && robots.obsidian <= 4 {
        let wait_time = i64::max(
            0,
            calc_ceil_of(&blueprint.clay_robot_ore_cost, &minerals.ore, &robots.ore),
        ) + 1;
        let rem_time: i64 = time - wait_time as i64;
        if rem_time > 0 {
            let mut new_minerals = update_minerals(minerals.clone(), &robots, &(wait_time as u64));
            new_minerals.ore -= blueprint.clay_robot_ore_cost;

            let mut new_robots = robots.clone();
            new_robots.clay += 1;

            max_val = max_val.max(dfs(blueprint, cache, rem_time, new_robots, new_minerals));
        }
    }

    if robots.ore < blueprint.max_spend.ore && robots.clay <= 2 {
        let wait_time = i64::max(
            0,
            calc_ceil_of(&blueprint.ore_robot_ore_cost, &minerals.ore, &robots.ore),
        ) + 1;
        let rem_time: i64 = time - wait_time as i64;
        if rem_time > 0 {
            let mut new_minerals = update_minerals(minerals.clone(), &robots, &(wait_time as u64));
            new_minerals.ore -= blueprint.ore_robot_ore_cost;

            let mut new_robots = robots.clone();
            new_robots.ore += 1;

            max_val = max_val.max(dfs(blueprint, cache, rem_time, new_robots, new_minerals));
        }
    }

    cache.insert(hash_key, max_val);
    return max_val;
}

fn update_minerals(mut minerals: MineralSet, new_robots: &RobotSet, times: &u64) -> MineralSet {
    minerals.ore = minerals.ore + new_robots.ore * times;
    minerals.clay = minerals.clay + new_robots.clay * times;
    minerals.obsidian = minerals.obsidian + new_robots.obsidian * times;
    minerals.geode = minerals.geode + new_robots.geode * times;
    return minerals;
}

fn calculate_hash_minerals(ores: &MineralSet) -> u64 {
    let mut o_hasher: DefaultHasher = DefaultHasher::new();
    ores.hash(&mut o_hasher);
    o_hasher.finish()
}

fn calculate_hash_robot(ores: &RobotSet) -> u64 {
    let mut r_hasher: DefaultHasher = DefaultHasher::new();
    ores.hash(&mut r_hasher);
    r_hasher.finish()
}

fn run(input_file: &str) {
    // Vars
    let mut blueprints: Vec<Blueprint> = Vec::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    for (_index, line) in reader.lines().enumerate() {
        let trimmed_line = line.unwrap().trim().to_string();

        if trimmed_line.is_empty() {
            continue;
        }

        let rgx =
            Regex::new(r#"Blueprint ([-0-9]+): Each ore robot costs ([-0-9]+) ore. Each clay robot costs ([-0-9]+) ore. Each obsidian robot costs ([-0-9]+) ore and ([-0-9]+) clay. Each geode robot costs ([-0-9]+) ore and ([-0-9]+) obsidian."#)
                .unwrap();

        let captures = rgx.captures(&trimmed_line).unwrap();
        let blueprint_number = captures[1].parse().unwrap();
        let ore_robot_ore_cost = captures[2].parse().unwrap();
        let clay_robot_ore_cost = captures[3].parse().unwrap();
        let obsidian_robot_ore_cost = captures[4].parse().unwrap();
        let obsidian_robot_clay_cost = captures[5].parse().unwrap();
        let geode_robot_ore_cost = captures[6].parse().unwrap();
        let geode_robot_obsidian_cost = captures[7].parse().unwrap();

        let mut max_spend = MineralSet {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: u64::MAX,
        };

        max_spend.ore = max_spend.ore.max(ore_robot_ore_cost);
        max_spend.ore = max_spend.ore.max(clay_robot_ore_cost);
        max_spend.ore = max_spend.ore.max(obsidian_robot_ore_cost);
        max_spend.ore = max_spend.ore.max(geode_robot_ore_cost);

        max_spend.clay = max_spend.ore.max(obsidian_robot_clay_cost);
        max_spend.obsidian = max_spend.obsidian.max(geode_robot_obsidian_cost);

        let blueprint = Blueprint {
            blueprint_number,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
            max_spend,
        };
        blueprints.push(blueprint);
    }

    // Solve
    let mut result_map: HashMap<u64, u64> = HashMap::new();

    let mut final_result = 0;
    for blueprint in &blueprints {
        // Start values
        let mut cache: HashMap<CacheKey, u64> = HashMap::new();
        let time = 24;
        let robot_set = RobotSet {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        };

        let ore_set = MineralSet {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
        let result = dfs(blueprint, &mut cache, time, robot_set, ore_set);
        result_map.insert(blueprint.blueprint_number.clone(), result.clone());
        final_result += result * blueprint.blueprint_number;
        println!("Blueprint {} has a score of {}. That results in a quality of {}", blueprint.blueprint_number, result, result * blueprint.blueprint_number)
    }


    println!("Final Result {}", final_result);
}

fn run2(input_file: &str) {
        // Vars
        let mut blueprints: Vec<Blueprint> = Vec::new();

        // Parse
        let file = File::open(input_file).unwrap();
        let reader = BufReader::new(file);
        for (_index, line) in reader.lines().enumerate() {
            let trimmed_line = line.unwrap().trim().to_string();
    
            if trimmed_line.is_empty() {
                continue;
            }
    
            let rgx =
                Regex::new(r#"Blueprint ([-0-9]+): Each ore robot costs ([-0-9]+) ore. Each clay robot costs ([-0-9]+) ore. Each obsidian robot costs ([-0-9]+) ore and ([-0-9]+) clay. Each geode robot costs ([-0-9]+) ore and ([-0-9]+) obsidian."#)
                    .unwrap();
    
            let captures = rgx.captures(&trimmed_line).unwrap();
            let blueprint_number = captures[1].parse().unwrap();
            let ore_robot_ore_cost = captures[2].parse().unwrap();
            let clay_robot_ore_cost = captures[3].parse().unwrap();
            let obsidian_robot_ore_cost = captures[4].parse().unwrap();
            let obsidian_robot_clay_cost = captures[5].parse().unwrap();
            let geode_robot_ore_cost = captures[6].parse().unwrap();
            let geode_robot_obsidian_cost = captures[7].parse().unwrap();
    
            let mut max_spend = MineralSet {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: u64::MAX,
            };
    
            max_spend.ore = max_spend.ore.max(ore_robot_ore_cost);
            max_spend.ore = max_spend.ore.max(clay_robot_ore_cost);
            max_spend.ore = max_spend.ore.max(obsidian_robot_ore_cost);
            max_spend.ore = max_spend.ore.max(geode_robot_ore_cost);
    
            max_spend.clay = max_spend.ore.max(obsidian_robot_clay_cost);
            max_spend.obsidian = max_spend.obsidian.max(geode_robot_obsidian_cost);
    
            let blueprint = Blueprint {
                blueprint_number,
                ore_robot_ore_cost,
                clay_robot_ore_cost,
                obsidian_robot_ore_cost,
                obsidian_robot_clay_cost,
                geode_robot_ore_cost,
                geode_robot_obsidian_cost,
                max_spend,
            };
            blueprints.push(blueprint);
        }
    
        // Solve
        let mut result_map: HashMap<u64, u64> = HashMap::new();
    
        let mut final_result = 1;
        let mut break_after = 3;
        for blueprint in &blueprints {
            // Start values
            let mut cache: HashMap<CacheKey, u64> = HashMap::new();
            let time = 32;
            let robot_set = RobotSet {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            };
    
            let ore_set = MineralSet {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            };
            let result = dfs(blueprint, &mut cache, time, robot_set, ore_set);
            result_map.insert(blueprint.blueprint_number.clone(), result.clone());
            final_result *= result;
            println!("Blueprint {} has a score of {}", blueprint.blueprint_number, result);
            break_after -= 1;
            if break_after == 0{
                break;
            }
        }
    
        println!("Final Result {}", final_result);
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
