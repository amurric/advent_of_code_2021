use std::collections::HashMap;
use std::env;
use std::process;
use std::fs;

use crate::Part::{First, Second};

enum Part {
    First(String),
    Second(String),
}

fn parse_input(lines: Vec<&str>) -> Vec<(Vec<&str>, Vec<&str>)> {
    let mut data = Vec::new();

    for line in lines {
        let line_split: Vec<&str> = line.split(" | ").collect();
        let first_part: Vec<&str> = line_split[0].split(' ').collect();
        let second_part: Vec<&str> = line_split[1].split(' ').collect();
        data.push((first_part, second_part));
    }

    data
}

fn first_part(file_content: String) -> i32 {
    let lines: Vec<&str> = file_content.lines().collect();
    let input_data = parse_input(lines);

    let mut count = 0;

    for (_, output_value) in input_data {
        for value in output_value {
            count += match value.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            }
        }
    }

    count
}

fn to_sorted_chars_vectors(input: Vec<(Vec<&str>, Vec<&str>)>) -> Vec<(Vec<Vec<char>>, Vec<Vec<char>>)> {
    let mut output = Vec::new();

    for (first_part, second_part) in input {
        let mut first_part_chars = Vec::new();
        for digit in first_part {
            let mut digit_chars: Vec<char> = digit.chars().collect();
            digit_chars.sort();
            first_part_chars.push(digit_chars);
        }

        let mut second_part_chars = Vec::new();
        for digit in second_part {
            let mut digit_chars: Vec<char> = digit.chars().collect();
            digit_chars.sort();
            second_part_chars.push(digit_chars);
        }

        output.push((first_part_chars, second_part_chars));
    }

    output
}

fn not_contains_all_segments_in_number(digits: &mut Vec<Vec<char>>, number: &Vec<char>) -> Vec<char> {
    for (index, digit) in digits.iter().enumerate() {
        for segment in number {
            if !digit.contains(segment) {
                return digits.remove(index);
            }
        }
    }

    panic!("Digit not found!");
}

fn contains_all_segments_in_number(digits: &mut Vec<Vec<char>>, number: &Vec<char>) -> Vec<char> {
    for (index, digit) in digits.iter().enumerate() {
        let mut found = true;
        for segment in number {
            if !digit.contains(segment) {
                found = false;
            }
        }
        if found {
            return digits.remove(index);
        }
    }

    panic!("Digit not found!");
}

fn contains_all_segments_in_digit(digits: &mut Vec<Vec<char>>, number: &Vec<char>) -> Vec<char> {
    for (index, digit) in digits.iter().enumerate() {
        let mut found = true;
        for segment in digit {
            if !number.contains(segment) {
                found = false;
            }
        }
        if found {
            return digits.remove(index);
        }
    }

    panic!("Digit not found!");
}

fn get_digits_map(digits_data: Vec<Vec<char>>) -> HashMap<Vec<char>, i32> {
    let mut output = HashMap::new();
    let mut segments = HashMap::new();

    let mut six_segments = Vec::new();
    let mut five_segments = Vec::new();

    for digit in digits_data {
        match digit.len() {
            2 => {segments.insert(1, digit);},
            3 => {output.insert(digit, 7);},
            4 => {segments.insert(4, digit);},
            5 => five_segments.push(digit),
            6 => six_segments.push(digit),
            7 => {output.insert(digit, 8);},
            _ => panic!("Unknown digit."),
        }
    }

    segments.insert(6, not_contains_all_segments_in_number(&mut six_segments, segments.get(&1).unwrap()));

    output.insert(not_contains_all_segments_in_number(&mut six_segments, segments.get(&4).unwrap()), 0);
    output.insert(six_segments.remove(0), 9);

    output.insert(contains_all_segments_in_number(&mut five_segments, segments.get(&1).unwrap()), 3);
    output.insert(contains_all_segments_in_digit(&mut five_segments, segments.get(&6).unwrap()), 5);
    output.insert(five_segments.remove(0), 2);

    output.insert(segments.remove(&1).unwrap(), 1);
    output.insert(segments.remove(&4).unwrap(), 4);
    output.insert(segments.remove(&6).unwrap(), 6);

    output
}

fn second_part(file_content: String) -> i32 {
    let lines: Vec<&str> = file_content.lines().collect();
    let input_data = to_sorted_chars_vectors(parse_input(lines));

    let mut sum = 0;

    for (digits_data, output_value) in input_data {
        let digits_map = get_digits_map(digits_data);

        let value = digits_map.get(&*output_value[0]).unwrap() * 1000 +
                    digits_map.get(&*output_value[1]).unwrap() * 100 +
                    digits_map.get(&*output_value[2]).unwrap() * 10 +
                    digits_map.get(&*output_value[3]).unwrap();

        sum += value;
    }

    sum
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
