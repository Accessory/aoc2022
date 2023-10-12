#![feature(map_many_mut)]

use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

use utils::get_input_path;

#[allow(dead_code)]
struct LFile {
    pub name: String,
    pub size: usize,
}

#[allow(dead_code)]
struct Leaf {
    pub path: String,
    pub files: Vec<LFile>,
    pub directories: HashMap<String, Rc<RefCell<Leaf>>>,
    pub folder_size: usize,
}

fn create_new_leaf(leaf_path: String) -> Leaf {
    Leaf {
        path: leaf_path,
        files: Vec::new(),
        directories: HashMap::new(),
        folder_size: 0,
    }
}

fn create_new_file(file_line: &str) -> LFile {
    let mut split = file_line.split(" ");
    LFile {
        size: split.next().unwrap().parse().unwrap(),
        name: split.next().unwrap().to_string().clone(),
    }
}

fn add_file_to_leaf(current_leaf: Rc<RefCell<Leaf>>, trimmed_line: String) {
    let new_file = create_new_file(&trimmed_line);
    (*current_leaf).borrow_mut().files.push(new_file);
}

fn get_current_leaf<'a>(cwd: &'a str, tree: Rc<RefCell<Leaf>>) -> Rc<RefCell<Leaf>> {
    let splits = cwd.split("/");
    let mut rtn = tree.clone();
    for s in splits {
        if s.is_empty() {
            continue;
        }
        let tmp = (*rtn).borrow().directories.get(s).unwrap().clone();
        rtn = tmp;
    }
    return rtn;
}

fn udpate_folder_size<'a>(
    tree: Rc<RefCell<Leaf>>,
    result_list: &mut Vec<Rc<RefCell<Leaf>>>,
    limit: usize,
) {
    for leaf in (*tree).borrow_mut().directories.values() {
        udpate_folder_size(leaf.clone(), result_list, limit);
    }

    let tmp = &mut (*(*tree).borrow_mut());

    for leaf in tmp.directories.values() {
        tmp.folder_size += (*(*leaf)).borrow().folder_size;
    }

    for lfile in &tmp.files {
        tmp.folder_size += &lfile.size;
    }

    result_list.push(tree.clone());
}

fn run(input_file: &str) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let root: Rc<RefCell<Leaf>> = Rc::new(RefCell::new(create_new_leaf(String::from(""))));

    let mut current_directory: String = String::new();

    for (_index, line) in reader.lines().enumerate() {
        let trimmed_line = line.unwrap().trim().to_string();

        if trimmed_line.is_empty() {
            continue;
        }

        if trimmed_line == "$ cd /" {
        } else if trimmed_line.starts_with("$ cd ..") {
            let index_of = &current_directory.rfind('/').unwrap();
            current_directory = current_directory[0..*index_of].to_string();
        } else if trimmed_line.starts_with("$ cd ") {
            current_directory = format!("{}/{}", &current_directory, &trimmed_line[5..]);
        } else if trimmed_line == "$ ls" {
            continue;
        } else if trimmed_line.starts_with("dir ") {
            let folder_name = &trimmed_line[4..];
            let current_leaf = get_current_leaf(&current_directory, root.clone());
            (*current_leaf).borrow_mut().directories.insert(
                folder_name.to_string(),
                Rc::new(RefCell::new(create_new_leaf(String::from(format!(
                    "{}/{}",
                    current_directory, folder_name
                ))))),
            );
        } else if trimmed_line.chars().nth(0).unwrap().is_ascii_digit() {
            let current_leaf = get_current_leaf(&current_directory, root.clone());
            add_file_to_leaf(current_leaf, trimmed_line);
        } else {
            panic!("Something is wrong");
        }
    }

    let mut result_list: Vec<Rc<RefCell<Leaf>>> = Vec::new();
    udpate_folder_size(root.clone(), &mut result_list, 10000);

    let mut result: usize = 0;
    for item in &result_list {
        if (**item).borrow().folder_size < 100000 {
            result += (**item).borrow().folder_size;
        }
    }
    println!("Result {}", result);
}

fn run2(input_file: &str) {
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let root: Rc<RefCell<Leaf>> = Rc::new(RefCell::new(create_new_leaf(String::from(""))));

    let mut current_directory: String = String::new();

    for (_index, line) in reader.lines().enumerate() {
        let trimmed_line = line.unwrap().trim().to_string();

        if trimmed_line.is_empty() {
            continue;
        }

        if trimmed_line == "$ cd /" {
        } else if trimmed_line.starts_with("$ cd ..") {
            let index_of = &current_directory.rfind('/').unwrap();
            current_directory = current_directory[0..*index_of].to_string();
        } else if trimmed_line.starts_with("$ cd ") {
            current_directory = format!("{}/{}", &current_directory, &trimmed_line[5..]);
        } else if trimmed_line == "$ ls" {
            continue;
        } else if trimmed_line.starts_with("dir ") {
            let folder_name = &trimmed_line[4..];
            let current_leaf = get_current_leaf(&current_directory, root.clone());
            (*current_leaf).borrow_mut().directories.insert(
                folder_name.to_string(),
                Rc::new(RefCell::new(create_new_leaf(String::from(format!(
                    "{}/{}",
                    current_directory, folder_name
                ))))),
            );
        } else if trimmed_line.chars().nth(0).unwrap().is_ascii_digit() {
            let current_leaf = get_current_leaf(&current_directory, root.clone());
            add_file_to_leaf(current_leaf, trimmed_line);
        } else {
            panic!("Something is wrong");
        }
    }

    let mut result_list: Vec<Rc<RefCell<Leaf>>> = Vec::new();
    udpate_folder_size(root.clone(), &mut result_list, usize::MAX);

    let mut result: usize = usize::MAX;
    let _at_least_free: i64 = (70000000 - (*root).borrow().folder_size as i64 - 30000000) * -1;
    for item in &result_list {
        if _at_least_free as usize <= (**item).borrow().folder_size {
            result = min((**item).borrow().folder_size, result);
        }
    }
    println!("Result {}", result);
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
