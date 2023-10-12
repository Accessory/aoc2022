use std::fs;

use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new(".")
        .into_iter()
        .map(|e| e.unwrap())
        .filter(|f| f.file_name() == "input.txt")
    {
        println!("{:?}", entry.path().as_os_str());
        fs::write(entry.path(), "your input here").expect(&format!(
            "Failed to write to file: {:?}",
            entry.path().as_os_str()
        ));
    }
}
