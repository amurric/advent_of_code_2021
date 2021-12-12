use std::collections::HashMap;
use std::env;
use std::process;
use std::fs;

use crate::Part::{First, Second};

enum Part {
    First(String),
    Second(String),
}

fn add_relation<'a>(cave_system: &mut HashMap<&'a str, Vec<&'a str>>, start: &'a str, end: &'a str) {
    match cave_system.get_mut(start) {
        Some(end_caves) => {
            end_caves.push(end);
        },
        None => {
            let mut end_caves = Vec::new();
            end_caves.push(end);
            cave_system.insert(start, end_caves);
        },
    }
}

fn parse_cave_system(lines: Vec<&str>) -> HashMap<&str, Vec<&str>> {
    let mut cave_system = HashMap::new();

    for line in lines {
        let caves: Vec<&str> = line.split('-').collect();
        add_relation(&mut cave_system, caves[0], caves[1]);
        add_relation(&mut cave_system, caves[1], caves[0]);
    }

    cave_system
}

fn explore_path<'a>(paths_list: &mut Vec<Vec<&'a str>>, cave_system: &HashMap<&str, Vec<&'a str>>,
                    current_path: Vec<&'a str>, small_cave_visited_twice: bool) {
    let current_cave = current_path.last().unwrap();

    for next_cave in cave_system.get(current_cave).unwrap() {
        let is_lowercase = next_cave.chars().any(|x| x.is_lowercase());
        if is_lowercase {
            if *next_cave == "start" || (small_cave_visited_twice && current_path.contains(next_cave)) {
                continue;
            }
        }
        let mut new_path = current_path.clone();
        new_path.push(next_cave);
        if *next_cave == "end" {
            paths_list.push(new_path);
        }
        else {
            explore_path(paths_list, cave_system, new_path,
                         if is_lowercase && current_path.contains(next_cave) {
                             true
                         } else {
                             small_cave_visited_twice
                         });
        }
    }
}

fn first_part(file_content: String) -> i32 {
    let cave_system = parse_cave_system(file_content.lines().collect());
    let mut paths_list = Vec::new();

    let mut current_path = Vec::new();
    current_path.push("start");

    explore_path(&mut paths_list, &cave_system, current_path, true);

    paths_list.len() as i32
}

fn second_part(file_content: String) -> i32 {
    let cave_system = parse_cave_system(file_content.lines().collect());
    let mut paths_list = Vec::new();

    let mut current_path = Vec::new();
    current_path.push("start");

    explore_path(&mut paths_list, &cave_system, current_path, false);

    paths_list.len() as i32
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
