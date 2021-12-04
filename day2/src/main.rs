use std::env;
use std::process;
use std::fs;

use crate::Part::{First, Second};

enum Part {
    First(String),
    Second(String),
}

fn first_part(file_content: String) -> i32
{
    let mut x = 0;
    let mut depth = 0;

    for line in file_content.lines() {
        let command: Vec<&str> = line.split(' ').collect();
        let value = command[1].parse::<i32>().expect("Not a number.");

        match command[0] {
            "forward" => x += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => panic!("Unknown command."),
        }
    }

    x * depth
}

fn second_part(file_content: String) -> i32
{
    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in file_content.lines() {
        let command: Vec<&str> = line.split(' ').collect();
        let value = command[1].parse::<i32>().expect("Not a number.");

        match command[0] {
            "forward" => {
                x += value;
                depth += aim * value;
            },
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!("Unknown command."),
        }
    }

    x * depth
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
