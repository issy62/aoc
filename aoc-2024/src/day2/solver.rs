use crate::day2::input;

pub fn first(input_path: &str) {
    let input = input::read(input_path);
    let mut res = 0;

    for n in &input {
        let mut asc = false;
        let mut dec = false;
        let mut seq_ok = 0;

        for i in 1..n.len() {
            let diff = (n[i - 1] - n[i]).abs();

            if (n[i - 1] > n[i]) && (diff >= 1 && diff <= 3) {
                asc = true;
                if !(asc && dec) {
                    seq_ok += 1;
                }
            } else {
                dec = true;
                if diff >= 1 && diff <= 3 {
                    if !(asc && dec) {
                        seq_ok += 1;
                    }
                }
            }
        }

        if !(seq_ok != (n.len() - 1) || (asc && dec)) {
            res += 1;
            // println!("SAFE");
        }

        seq_ok = 0;
    }

    println!("Safe Reports: {}", res);
}

pub fn second(input_path: &str) {
    let _input = input::read(input_path);

    println!("Hi");
}

