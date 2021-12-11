use std::env;
use std::process;
use std::fs;

use crate::Part::{First, Second};

enum Part {
    First(String),
    Second(String),
}

fn parse_map(lines: Vec<&str>) -> Vec<Vec<i32>> {
    let mut map = Vec::new();

    for line in lines {
        let mut map_line = Vec::new();

        for char_value in line.chars() {
            map_line.push(char_value.to_digit(10).unwrap() as i32);
        }

        map.push(map_line);
    }

    map
}

fn is_low_point(map: &Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    let value = map[y][x];

   !(x != 0 && map[y][x - 1] <= value) &&
       !(x + 1 < map[y].len() && map[y][x + 1] <= value) &&
       !(y != 0 && map[y - 1][x] <= value) &&
       !(y + 1 < map.len() && map[y + 1][x] <= value)
}

fn first_part(file_content: String) -> i32 {
    let lines: Vec<&str> = file_content.lines().collect();
    let map = parse_map(lines);

    let mut sum = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if is_low_point(&map, x, y) {
                sum += map[y][x] + 1;
            }
        }
    }

    sum
}

fn add_basin_points(basin_points: &mut Vec<(usize, usize)>, map: &Vec<Vec<i32>>, x: usize, y: usize) {
    if map[y][x] == 9 || basin_points.contains(&(x, y)) {
        return
    }

    basin_points.push((x, y));

    if x != 0 {
        add_basin_points(basin_points, map, x - 1, y);
    }

    if x + 1 < map[y].len() {
        add_basin_points(basin_points, map, x + 1, y);
    }

    if y != 0 {
        add_basin_points(basin_points, map, x, y - 1);
    }

    if y + 1 < map.len() {
        add_basin_points(basin_points, map, x, y + 1);
    }
}

fn get_basin_size(map: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut basin_points = Vec::new();

    add_basin_points(&mut basin_points, map, x, y);

    basin_points.len() as i32
}

fn second_part(file_content: String) -> i32 {
    let lines: Vec<&str> = file_content.lines().collect();
    let map = parse_map(lines);

    let mut basin_sizes = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if is_low_point(&map, x, y) {
                basin_sizes.push(get_basin_size(&map, x, y));
            }
        }
    }

    basin_sizes.sort();

    basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap()
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