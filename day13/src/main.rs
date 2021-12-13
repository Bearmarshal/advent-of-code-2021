use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;

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
        static ref DOT_REGEX: Regex = Regex::new(r"(?P<x>\d+),(?P<y>\d+)").unwrap();
        static ref FOLD_REGEX: Regex =
            Regex::new(r"fold along (?P<axis>[xy])=(?P<coordinate>\d+)").unwrap();
    }
    let (dots, folds) = input.split_once("\n\n").unwrap();

    let mut paper = DOT_REGEX
        .captures_iter(dots)
        .map(|capture| {
            (
                capture.name("y").unwrap().as_str().parse().unwrap(),
                capture.name("x").unwrap().as_str().parse().unwrap(),
            )
        })
        .collect::<HashSet<(i32, i32)>>();

    for capture in FOLD_REGEX.captures_iter(folds).take(1) {
        let coordinate = capture
            .name("coordinate")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        match capture.name("axis").unwrap().as_str() {
            "y" => {
                paper = paper
                    .into_iter()
                    .filter_map(|(y, x)| {
                        if y < coordinate {
                            Some((y, x))
                        } else if y > coordinate {
                            Some((2 * coordinate - y, x))
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            "x" => {
                paper = paper
                    .into_iter()
                    .filter_map(|(y, x)| {
                        if x < coordinate {
                            Some((y, x))
                        } else if x > coordinate {
                            Some((y, 2 * coordinate - x))
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            _ => panic!(),
        }
    }

    println!("Part 1: {}", paper.len());
}

fn part2(input: &str) {
    println!("Part 2: {}", "");
}
