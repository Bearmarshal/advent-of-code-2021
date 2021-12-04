use regex::Match;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;
    part1(&input);
    part2(&input);
    Ok(())
}

fn part1(input: &str) {
    let board_regex = Regex::new(r"(?:( *\d+){5}\n){5}").unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();
    let bingo_numbers = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(str::trim)
        .map(str::parse::<i32>)
        .flatten();
    let mut bingo_boards: Vec<Vec<Vec<i32>>> = board_regex
        .find_iter(input)
        .map(|bingo_match| {
            bingo_match
                .as_str()
                .lines()
                .map(|line| {
                    number_regex
                        .find_iter(line)
                        .map(|number_match| number_match.as_str().parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    println!("Part 1: {}", "");
}

fn part2(input: &str) {
    println!("Part 2: {}", "");
}
