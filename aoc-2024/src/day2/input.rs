use std::fs;
use std::path::Path;

pub fn read(input_path: &str) -> Vec<Vec<i64>> {
    let path = Path::new(input_path);
    let content = fs::read_to_string(path).expect("Could not read file");

    let data: Vec<Vec<i64>> = content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<&str>>()
        .iter()
        .map(|n| {
            n.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap_or(0))
                .collect()
        })
        .collect();
    data
}

