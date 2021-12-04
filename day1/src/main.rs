use std::env;
use std::process;
use std::fs;

use crate::Part::{First, Second};

enum Part {
    First(String),
    Second(String),
}

fn first_part(filename: String) {
    let file_content = fs::read_to_string(filename).expect("Error reading input file.");

    let mut count = 0;
    let mut previous_depth = None;

    for line in file_content.lines() {
        let depth = line.parse::<i32>().expect("Not a number.");

        match previous_depth {
            Some(previous) => if previous < depth {
                count += 1;
            },
            None => (),
        }

        previous_depth = Some(depth);
    }

    println!("{}", count);
}

fn second_part(filename: String) {
    let file_content = fs::read_to_string(filename).expect("Error reading input file.");

    let mut count = 0;
    let mut measurement_window = Vec::new();

    for line in file_content.lines() {
        let depth = line.parse::<i32>().expect("Not a number.");

        if measurement_window.len() == 3 {
            let previous_sum: i32 = measurement_window.iter().sum();

            measurement_window.drain(0..1);
            let new_sum = measurement_window.iter().sum::<i32>() + depth;

            if previous_sum < new_sum {
                count += 1;
            }
        }

        measurement_window.push(depth);
    }

    println!("{}", count);
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
    match parse_arguments() {
        First(filename) => first_part(filename),
        Second(filename) => second_part(filename),
    }
}
