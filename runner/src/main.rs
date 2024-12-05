use clap::Parser;
use aoc_2024::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Flags {
    #[arg(short, long)]
    year: i32,
    #[arg(short, long)]
    day: i32,
}

enum UsageCase {
    Day,
    Year,
}

fn print_usage(usage_case: UsageCase) {
    match usage_case {
        UsageCase::Day => {
            println!("Usage: cargo run -- --day <day>");
            println!("Usage: runner -y <year> -d <day>");
        }
        UsageCase::Year => {
            println!("Usage: cargo run -- --year <year>");
            println!("Usage: runner -y 2024 -d 1");
        }
    }
}

fn main() {
    let flags = Flags::parse();

    match flags.year {
        2024 => match flags.day {
            1 => {
                day1::solver::first("inputs/2024/day1.txt");
                day1::solver::second("inputs/2024/day1.txt");
            }
            2 => {
                day2::solver::first("inputs/2024/day2.txt");
                day2::solver::second("inputs/2024/day2.txt");
            }
            _ => {
                print_usage(UsageCase::Day);
            }
        },
        _ => {
            print_usage(UsageCase::Year);
        }
    }
}

