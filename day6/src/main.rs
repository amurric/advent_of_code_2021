use std::collections::HashMap;
use std::env;
use std::process;
use std::fs;

use crate::Part::{First, Second};

enum Part {
    First(String),
    Second(String),
}

fn num_fishes_in_days(timer: i64, mut days: i64) -> i64 {
    let mut count = 1;
    days -= timer + 1;

    while days > 0 {
        count += num_fishes_in_days(8, days);
        days -= 7;
    }

    count
}

fn first_part(file_content: String) -> i64 {
    let mut count = 0;

    for value in file_content.trim().split(',') {
        let timer = value.parse::<i64>().expect("Not a number!?");
        count += num_fishes_in_days(timer, 81);
    }

    count
}

fn second_part(file_content: String) -> i64 {
    let mut count = 0;

    let mut result_cache = HashMap::new();

    for value in file_content.trim().split(',') {
        let timer = value.parse::<i64>().expect("Not a number!?");

        match result_cache.get(&timer) {
            Some(value) => count += value,
            None => {
                let value = num_fishes_in_days(timer, 257);
                count += value;
                result_cache.insert(timer, value);
            }
        }
    }

    count
}

fn parse_arguments() -> Part {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        process::exit(1);
    }

    match args[1].as_str() {
        "P1" => First(String::from(&args[2])),
        "P2" => Second(String::from(&args[2])),
        _=> process::exit(1),
    }
}

fn main() {
    let result = match parse_arguments() {
        First(filename) => first_part(fs::read_to_string(filename).unwrap()),
        Second(filename) => second_part(fs::read_to_string(filename).unwrap()),
    };

    println!("{}", result);
}
