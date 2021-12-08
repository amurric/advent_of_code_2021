use std::env;
use std::process;
use std::fs;

use crate::Part::{First, Second};

enum Part {
    First(String),
    Second(String),
}

fn get_gamma_mul_epsilon(bit_counters: Vec<i32>, number_count: i32) -> i32 {
    let mut gamma = 0;
    let mut epsilon = 0;
    let bit_count = bit_counters.len();

    for (i, counter) in bit_counters.iter().enumerate() {
        if *counter > (number_count / 2) {
            gamma |= 1 << (bit_count - i - 1);
        }
        else {
            epsilon |= 1 << (bit_count - i - 1);
        }
    }

    gamma * epsilon
}

fn first_part(file_content: String) -> i32 {
    let lines: Vec<&str> = file_content.lines().collect();
    let lines_length = lines.len() as i32;
    let bit_count = lines[0].chars().count();
    let mut bit_counters = vec![0; bit_count];

    for line in lines {
        for (i, bit) in line.chars().enumerate() {
            if bit == '1' {
                bit_counters[i] += 1;
            }
        }
    }

    get_gamma_mul_epsilon(bit_counters, lines_length)
}

fn get_zero_one_values(values: Vec<&str>, position: usize) -> (Vec<&str>, Vec<&str>) {
    let mut values_zero = Vec::new();
    let mut values_one = Vec::new();

    for value in values {
        let char_value: Vec<char> = value.chars().collect();
        match char_value[position] {
            '0' => values_zero.push(value),
            '1' => values_one.push(value),
            _ => panic!("Unknown value."),
        }
    }

    (values_zero, values_one)
}

fn get_oxygen_generator_rating(mut values: Vec<&str>) -> i32 {
    let mut index = 0;

    while values.len() > 1 {
        let (values_zero, values_one) = get_zero_one_values(values, index);
        values = if values_zero.len() > values_one.len() {
            values_zero
        }
        else {
            values_one
        };
        index += 1;
    }

    i32::from_str_radix(values[0], 2).unwrap()
}

fn get_co2_scrubber_rating(mut values: Vec<&str>) -> i32 {
    let mut index = 0;

    while values.len() > 1 {
        let (values_zero, values_one) = get_zero_one_values(values, index);
        values = if values_one.len() < values_zero.len() {
            values_one
        }
        else {
            values_zero
        };
        index += 1
    }

    i32::from_str_radix(values[0], 2).unwrap()
}

fn second_part(file_content: String) -> i32 {
    let lines: Vec<&str> = file_content.lines().collect();
    get_oxygen_generator_rating(lines.clone()) * get_co2_scrubber_rating(lines)
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
