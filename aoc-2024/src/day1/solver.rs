use crate::day1::input;

pub fn first(input_path: &str) {
    let input = input::read(input_path);

    let col: Vec<i64> = input
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap_or(0))
        .collect();

    let reserve_size: usize = col.len() / 2;

    let mut lhs: Vec<i64> = Vec::new();
    let mut rhs: Vec<i64> = Vec::new();

    lhs.reserve(reserve_size);
    rhs.reserve(reserve_size);

    for n in 0..col.len() {
        if n == 0 {
            lhs.push(col[n]);
        } else if n % 2 == 0 {
            lhs.push(col[n]);
        } else {
            rhs.push(col[n]);
        }
    }

    lhs.sort_unstable();
    rhs.sort_unstable();

    let sum: i64 = lhs.iter().zip(rhs.iter()).map(|(a, b)| (a - b).abs()).sum();

    println!("{}", sum);
}

pub fn second(input_path: &str) {
    let input = input::read(input_path);

    let col: Vec<i64> = input
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap_or(0))
        .collect();

    let reserve_size: usize = col.len() / 2;

    let mut lhs: Vec<i64> = Vec::new();
    let mut rhs: Vec<i64> = Vec::new();

    rhs.reserve(reserve_size);
    lhs.reserve(reserve_size);

    for n in 0..col.len() {
        if n == 0 {
            lhs.push(col[n]);
        } else if n % 2 == 0 {
            lhs.push(col[n]);
        } else {
            rhs.push(col[n]);
        }
    }

    lhs.sort_unstable();
    rhs.sort_unstable();

    let mut sim: Vec<i64> = Vec::new();
    sim.reserve(reserve_size);
    for n in lhs {
        let c: i64 = rhs
            .iter()
            .filter(|x| x == &&n)
            .count()
            .try_into()
            .unwrap_or(0);
        sim.push(n * c);
    }

    println!("{}", sim.iter().sum::<i64>());
}

