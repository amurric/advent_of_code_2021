use std::cmp;
use std::env;
use std::process;
use std::fs;

use crate::Part::{First, Second};

enum Part {
    First(String),
    Second(String),
}

fn get_fuel(positions: &Vec<i32>, align_position: i32) -> i32 {
    let mut fuel = 0;

    for position in positions {
        fuel += (position - align_position).abs();
    }

    fuel
}

fn first_part(file_content: String) -> i32 {
    let mut positions = Vec::new();

    for value in file_content.trim().split(',') {
        positions.push(value.parse::<i32>().expect("Not a number!?"))
    }

    positions.sort();

    let median1 = positions[positions.len() / 2];
    let median2 = positions[positions.len() / 2 - 1];

    cmp::min(get_fuel(&positions, median1), get_fuel(&positions, median2))
}

fn get_right_fuel(positions: &Vec<i32>, align_position: i32) -> i32 {
    let mut fuel = 0;

    for position in positions {
        let distance = (position - align_position).abs();
        fuel += distance * (distance + 1) / 2;
    }

    fuel
}

fn second_part(file_content: String) -> i32 {
    let mut positions = Vec::new();

    for value in file_content.trim().split(',') {
        positions.push(value.parse::<i32>().expect("Not a number!?"))
    }

    let mean1 = (positions.iter().sum::<i32>() as f32 / positions.len() as f32).floor() as i32;
    let mean2 = (positions.iter().sum::<i32>() as f32 / positions.len() as f32).ceil() as i32;

    cmp::min(get_right_fuel(&positions, mean1), get_right_fuel(&positions, mean2))
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
