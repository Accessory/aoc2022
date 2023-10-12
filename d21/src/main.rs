use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

use regex::Regex;
use utils::get_input_path;

#[derive(Debug)]
struct Operation {
    pub name: String,
    pub name1: String,
    pub operation: String,
    pub name2: String,
    pub number: Option<i64>,
    pub should_be: i64,
}

fn get_number(name: &str, items: Rc<RefCell<HashMap<String, Rc<RefCell<Operation>>>>>) -> i64 {
    let item_rc = items.borrow_mut().get_mut(name).unwrap().clone();
    let mut item = (*item_rc).borrow_mut();
    if item.number.is_none() {
        item.number = Some(match item.operation.as_str() {
            "+" => get_number(&item.name1, items.clone()) + get_number(&item.name2, items.clone()),
            "-" => get_number(&item.name1, items.clone()) - get_number(&item.name2, items.clone()),
            "*" => get_number(&item.name1, items.clone()) * get_number(&item.name2, items.clone()),
            "/" => get_number(&item.name1, items.clone()) / get_number(&item.name2, items.clone()),
            _ => panic!("Should not be here"),
        });
    }

    return item.number.unwrap();
}

fn find_item(
    from: &str,
    to: &str,
    items: Rc<RefCell<HashMap<String, Rc<RefCell<Operation>>>>>,
) -> Option<VecDeque<Rc<RefCell<Operation>>>> {
    let item_rc = (*items).borrow().get(from).unwrap().clone();
    let item = (*item_rc).borrow();

    if item.name1 == to || item.name2 == to {
        let next_item = (*items).borrow().get(to).unwrap().clone();
        return Some(VecDeque::from([item_rc.clone(), next_item]));
    }

    if item.number.is_some() {
        return None;
    }

    let result = find_item(&item.name1, to, items.clone());
    if result.is_some() {
        let mut results = result.unwrap();
        results.push_front(item_rc.clone());
        return Some(results);
    }

    let result2 = find_item(&item.name2, to, items.clone());
    if result2.is_some() {
        let mut results = result2.unwrap();
        results.push_front(item_rc.clone());
        return Some(results);
    }

    return None;
}

fn recalculate(
    item_rc: Rc<RefCell<Operation>>,
    given: String,
    is_left: bool,
    items: Rc<RefCell<HashMap<String, Rc<RefCell<Operation>>>>>,
) -> i64 {
    let item = (*item_rc).borrow();

    match item.operation.as_str() {
        "+" => item.should_be - get_number(given.as_str(), items.clone()),
        "-" => {
            if is_left {
                item.should_be + get_number(given.as_str(), items.clone())
            } else {
                get_number(given.as_str(), items.clone()) - item.should_be
            }
        }
        "*" => item.should_be / get_number(given.as_str(), items.clone()),
        "/" => {
            if is_left {
                item.should_be * get_number(given.as_str(), items.clone())
            } else {
                get_number(given.as_str(), items.clone()) / item.should_be
            }
        }
        _ => panic!("Something is wrong"),
    }
}

fn run(input_file: &str) {
    // Prepare
    let mut items: HashMap<String, Rc<RefCell<Operation>>> = HashMap::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        let rgx = Regex::new(r#"(\D\D\D\D): (\D\D\D\D) ([+\-*/]) (\D\D\D\D)"#).unwrap();

        let rgx_result = rgx.captures(&line);
        if rgx_result.is_some() {
            let captures = rgx_result.unwrap();

            let name = &captures[1];
            let name1 = &captures[2];
            let operation = &captures[3];
            let name2 = &captures[4];

            items.insert(
                name.to_string(),
                Rc::new(RefCell::new(Operation {
                    name: name.to_string(),
                    name1: name1.to_string(),
                    operation: operation.to_string(),
                    name2: name2.to_string(),
                    number: None,
                    should_be: 0,
                })),
            );
        } else {
            let rgx2 = Regex::new(r#"(\D\D\D\D): (\d+)"#).unwrap();
            let rgx2_captures = rgx2.captures(&line).unwrap();
            let name = &rgx2_captures[1];
            let number = &rgx2_captures[2].parse().unwrap();
            items.insert(
                name.to_string(),
                Rc::new(RefCell::new(Operation {
                    name: name.to_string(),
                    name1: String::from(""),
                    operation: String::from(""),
                    name2: String::from(""),
                    number: Some(*number),
                    should_be: 0,
                })),
            );
        }
    }

    let items_rc: Rc<RefCell<HashMap<String, Rc<RefCell<Operation>>>>> =
        Rc::new(RefCell::new(items));

    let result = get_number("root", items_rc.clone());
    println!("Result: {}", result);
}

fn run2(input_file: &str) {
    // Prepare
    let mut items: HashMap<String, Rc<RefCell<Operation>>> = HashMap::new();

    // Parse
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap().trim().to_string();
        let rgx = Regex::new(r#"(\D\D\D\D): (\D\D\D\D) ([+\-*/]) (\D\D\D\D)"#).unwrap();

        let rgx_result = rgx.captures(&line);
        if rgx_result.is_some() {
            let captures = rgx_result.unwrap();

            let name = &captures[1];
            let name1 = &captures[2];
            let operation = &captures[3];
            let name2 = &captures[4];

            items.insert(
                name.to_string(),
                Rc::new(RefCell::new(Operation {
                    name: name.to_string(),
                    name1: name1.to_string(),
                    operation: operation.to_string(),
                    name2: name2.to_string(),
                    number: None,
                    should_be: 0,
                })),
            );
        } else {
            let rgx2 = Regex::new(r#"(\D\D\D\D): (\d+)"#).unwrap();
            let rgx2_captures = rgx2.captures(&line).unwrap();
            let name = &rgx2_captures[1];
            let number = &rgx2_captures[2].parse().unwrap();
            items.insert(
                name.to_string(),
                Rc::new(RefCell::new(Operation {
                    name: name.to_string(),
                    name1: String::from(""),
                    operation: String::from(""),
                    name2: String::from(""),
                    number: Some(*number),
                    should_be: 0,
                })),
            );
        }
    }

    let items_rc: Rc<RefCell<HashMap<String, Rc<RefCell<Operation>>>>> =
        Rc::new(RefCell::new(items));

    // Solution Prepareration
    let chain = find_item("root", "humn", items_rc.clone()).unwrap();
    {
        let root_rc = items_rc.borrow_mut().get_mut("root").unwrap().clone();
        let mut root = (*root_rc).borrow_mut();
        let chain1 = chain.get(1).unwrap().clone();

        let to_match = if root.name1 == (*chain1).borrow().name {
            get_number(&root.name2, items_rc.clone())
        } else {
            get_number(&root.name1, items_rc.clone())
        };
        root.should_be = to_match * 2;
    }

    // Solve
    for i in 0..chain.len() - 1 {
        let chl = chain.get(i).unwrap().clone();
        let next = chain.get(i + 1).unwrap().clone();

        let given = if (*chl).borrow().name1 != (*next).borrow().name {
            (*chl).borrow().name1.clone()
        } else {
            (*chl).borrow().name2.clone()
        };

        let is_left = (*chl).borrow().name1 == (*next).borrow().name;

        (*next).borrow_mut().should_be = recalculate(chl.clone(), given, is_left, items_rc.clone());
        // println!("{:?}", (*next).borrow());
    }

    let human_rc = (*items_rc).borrow().get("humn").unwrap().clone();
    let human = (*human_rc).borrow();
    println!("Result is {}", human.should_be);
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
