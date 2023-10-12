use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

pub fn get_input_path(src_path: &str) -> PathBuf {
    let file_path = Path::new(src_path);
    if Path::exists(file_path) {
        file_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("input")
            .join("input.txt")
    } else {
        current_dir().unwrap().join("input").join("input.txt")
    }
}

pub fn get_test_input_path(src_path: &str) -> PathBuf {
    let file_path = Path::new(src_path);
    if Path::exists(file_path) {
        file_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("input")
            .join("input_test.txt")
    } else {
        current_dir().unwrap().join("input").join("input_test.txt")
    }
}

pub fn get_test_input_2_path(src_path: &str) -> PathBuf {
    let file_path = Path::new(src_path);
    if Path::exists(file_path) {
        file_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("input")
            .join("input_test_2.txt")
    } else {
        current_dir().unwrap().join("input").join("input_test.txt")
    }
}

#[cfg(test)]
mod tests {
    use crate::get_input_path;
    use crate::get_test_input_path;

    #[test]
    fn test_get_test_input_path() {
        println!("{}", get_test_input_path(file!()).to_string_lossy());
    }

    #[test]
    fn test_get_input_path() {
        println!("{}", get_input_path(file!()).to_string_lossy());
    }
}
