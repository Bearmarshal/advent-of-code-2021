use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
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
            Regex::new(r"(?P<pattern>(?:\w+ ){10})\|(?P<number>(?: \w+){4})").unwrap();
    }
    let count = LINE_REGEX
        .captures_iter(input)
        .map(|capture| capture.name("number").unwrap().as_str().trim().split(" "))
        .flatten()
        .filter(|segments| [2, 3, 4, 7].contains(&segments.len()))
        .count();

    println!("Part 1: {}", count);
}

fn part2(input: &str) {
    

    println!("Part 2: {}", "");
}
