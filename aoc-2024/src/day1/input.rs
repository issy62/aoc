use std::path::Path;
use std::fs;

pub fn read(input_path: &str) -> String {
    let path = Path::new(input_path);
    let content = fs::read_to_string(path).expect("Could not read file");
    content
}

