use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter;
use std::mem;

fn main() -> std::io::Result<()> {
    let mut input_file = File::open(match env::args().skip(1).next() {
        Some(filename) => filename,
        None => "input.txt".to_string(),
    })?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    part1(&input);
    part2(&input);
    Ok(())
}

fn part1(input: &str) {
    lazy_static! {
        static ref LINE_REGEX: Regex =
            Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
    }
    let mut vents = HashMap::<(i32, i32), i32>::new();

    for capture in LINE_REGEX.captures_iter(input) {
        let mut x1 = capture["x1"].parse::<i32>().unwrap();
        let mut y1 = capture["y1"].parse::<i32>().unwrap();
        let mut x2 = capture["x2"].parse::<i32>().unwrap();
        let mut y2 = capture["y2"].parse::<i32>().unwrap();
        
        if x1 == x2 {
            if y1 > y2 {
                mem::swap(&mut y1, &mut y2);
            }
            for y in y1..=y2 {
                *vents.entry((x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            if x1 > x2 {
                mem::swap(&mut x1, &mut x2);
            }
            for x in x1..=x2 {
                *vents.entry((x, y1)).or_insert(0) += 1;
            }
        }
    }

    println!("Part 1: {}", vents.values().filter(|v| **v > 1).count());
}

fn part2(input: &str) {
    lazy_static! {
        static ref LINE_REGEX: Regex =
            Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
    }
    let mut vents = HashMap::<(i32, i32), i32>::new();

    for capture in LINE_REGEX.captures_iter(input) {
        let x1 = capture["x1"].parse::<i32>().unwrap();
        let y1 = capture["y1"].parse::<i32>().unwrap();
        let x2 = capture["x2"].parse::<i32>().unwrap();
        let y2 = capture["y2"].parse::<i32>().unwrap();

        assert!(!(x1 == x2 && y1 == y2));
        let x_range = if x1 == x2 { Box::new(iter::repeat(x1)) } else { true_inclusive_range(x1, x2) };
        let y_range = if y1 == y2 { Box::new(iter::repeat(y1)) } else { true_inclusive_range(y1, y2) };
        
        for (x, y) in x_range.zip(y_range) {
            *vents.entry((x, y)).or_insert(0) += 1;
        }
    }
    
    println!("Part 2: {}", vents.values().filter(|v| **v > 1).count());
}

fn true_inclusive_range(a: i32, b: i32) -> Box<dyn Iterator<Item = i32>> {
    if a > b {
        Box::new((b..=a).rev())
    } else {
        Box::new(a..=b)
    }
}