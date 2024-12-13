use std::fs;
use std::path::Path;

pub fn read(input_path: &str) -> Vec<i64> {
    let path = Path::new(input_path);
    let file_content = fs::read_to_string(path).expect("Could not read file");

    let data: Vec<i64> = file_content
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap_or(0))
        .collect();

    data
}

