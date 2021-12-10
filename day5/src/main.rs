use std::collections::HashMap;
use std::env;
use std::process;
use std::fs;

use crate::Part::{First, Second};

enum Part {
    First(String),
    Second(String),
}

fn parse_vents(lines: Vec<&str>) -> Vec<((i32, i32), (i32, i32))> {
    let mut vents = Vec::new();

    for line in lines {
        let line_no_spaces = line.replace(" ", "");
        let points: Vec<&str> = line_no_spaces.split("->").collect();

        let coordinates: Vec<&str> = points[0].split(',').collect();
        let start_coordinates = (coordinates[0].parse::<i32>().unwrap(), coordinates[1].parse::<i32>().unwrap());

        let coordinates: Vec<&str> = points[1].split(',').collect();
        let end_coordinates = (coordinates[0].parse::<i32>().unwrap(), coordinates[1].parse::<i32>().unwrap());

        vents.push((start_coordinates, end_coordinates));
    }

    vents
}

fn minmax(value1: i32, value2: i32) -> (i32, i32) {
    if value1 > value2 {
        return (value2, value1)
    }
    (value1, value2)
}

fn update_point(points: &mut HashMap<(i32, i32), i32>, x: i32, y: i32) {
    let point = (x, y);
    match points.get_mut(&point) {
        Some(value) => *value += 1,
        None => {points.insert(point, 1);},
    }
}

fn count_equal_or_greater_than(points: &HashMap<(i32, i32), i32>, value: i32) -> i32 {
    let mut count = 0;

    for overlaping_lines in points.values() {
        if *overlaping_lines >= value {
            count += 1;
        }
    }

    count
}

fn first_part(file_content: String) -> i32 {
    let lines: Vec<&str> = file_content.lines().collect();
    let vents = parse_vents(lines);

    let mut points = HashMap::new();

    for ((x_start, y_start), (x_end, y_end)) in vents {
        if x_start == x_end {
            let (y_min, y_max) = minmax(y_start, y_end);
            for y in y_min..(y_max + 1) {
                update_point(&mut points, x_start, y);
            }
        }
        else if y_start == y_end {
            let (x_min, x_max) = minmax(x_start, x_end);
            for x in x_min..(x_max + 1) {
                update_point(&mut points, x, y_start);
            }
        }
    }

    count_equal_or_greater_than(&points, 2)
}

fn second_part(file_content: String) -> i32 {
    let lines: Vec<&str> = file_content.lines().collect();
    let vents = parse_vents(lines);

    let mut points = HashMap::new();

    for ((x_start, y_start), (x_end, y_end)) in vents {
        if x_start == x_end {
            let (y_min, y_max) = minmax(y_start, y_end);
            for y in y_min..(y_max + 1) {
                update_point(&mut points, x_start, y);
            }
        }
        else if y_start == y_end {
            let (x_min, x_max) = minmax(x_start, x_end);
            for x in x_min..(x_max + 1) {
                update_point(&mut points, x, y_start);
            }
        }
        else {
            let x_inc = if x_start < x_end { 1 } else { -1 };
            let y_inc = if y_start < y_end { 1 } else { -1 };

            let mut x = x_start;
            let mut y = y_start;

            while x != x_end && y != y_end {
                update_point(&mut points, x, y);
                x += x_inc;
                y += y_inc;
            }
            update_point(&mut points, x, y);
        }
    }

    count_equal_or_greater_than(&points, 2)
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
