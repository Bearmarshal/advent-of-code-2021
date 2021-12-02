use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    part1(&input);
    part2(&input);
    Ok(())
}

fn part1(input: &str) {
    let regex = Regex::new(r"forward (?P<forward>\d+)|down (?P<down>\d+)|up (?P<up>\d+)").unwrap();
    let mut distance = 0;
    let mut depth = 0;
    
    for capture in regex.captures_iter(input) {
        if let Some(forward) = capture.name("forward") {
            distance += forward.as_str().parse::<i32>().unwrap();
        } else if let Some(down) = capture.name("down") {
            depth += down.as_str().parse::<i32>().unwrap();
        } else if let Some(up) = capture.name("up") {
            depth -= up.as_str().parse::<i32>().unwrap();
        }
    }

    println!("Part 1: {}", distance * depth);
}

fn part2(input: &str) {
    let regex = Regex::new(r"forward (?P<forward>\d+)|down (?P<down>\d+)|up (?P<up>\d+)").unwrap();
    let mut distance = 0;
    let mut depth = 0;
    let mut aim = 0;
    
    for capture in regex.captures_iter(input) {
        if let Some(forward) = capture.name("forward") {
            let speed = forward.as_str().parse::<i32>().unwrap();
            distance += speed;
            depth += aim * speed;
        } else if let Some(down) = capture.name("down") {
            aim += down.as_str().parse::<i32>().unwrap();
        } else if let Some(up) = capture.name("up") {
            aim -= up.as_str().parse::<i32>().unwrap();
        }
    }
    
    println!("Part 2: {}", distance * depth);
}