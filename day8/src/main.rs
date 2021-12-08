use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
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
    lazy_static! {
        static ref LINE_REGEX: Regex =
            Regex::new(r"(?P<pattern>(?:\w+ ){10})\|(?P<number>(?: \w+){4})").unwrap();
    }

    let mut sum = 0;
    for capture in LINE_REGEX.captures_iter(input) {
        let mut digit_patterns = [
            HashSet::<char>::new(),
            HashSet::<char>::new(),
            HashSet::<char>::new(),
            HashSet::<char>::new(),
            HashSet::<char>::new(),
            HashSet::<char>::new(),
            HashSet::<char>::new(),
            HashSet::<char>::new(),
            HashSet::<char>::new(),
            HashSet::<char>::new(),
        ];
        let mut five_segments = Vec::<HashSet<char>>::new(); // 2, 3, 5
        let mut six_segments = Vec::<HashSet<char>>::new(); // 0, 6, 9

        for digit_pattern in capture
            .name("pattern")
            .unwrap()
            .as_str()
            .trim()
            .split(" ")
            .map(|pattern| pattern.chars().collect::<HashSet<char>>())
        {
            match digit_pattern.len() {
                2 => digit_patterns[1] = digit_pattern,
                3 => digit_patterns[7] = digit_pattern,
                4 => digit_patterns[4] = digit_pattern,
                5 => five_segments.push(digit_pattern),
                6 => six_segments.push(digit_pattern),
                7 => digit_patterns[8] = digit_pattern,
                _ => panic!(),
            }
        }

        for digit_pattern in six_segments {
            if (&digit_patterns[4] - &digit_pattern).is_empty() {
                digit_patterns[9] = digit_pattern;
            } else if (&digit_patterns[1] - &digit_pattern).is_empty() {
                digit_patterns[0] = digit_pattern;
            } else {
                digit_patterns[6] = digit_pattern;
            }
        }

        for digit_pattern in five_segments {
            if (&digit_pattern - &digit_patterns[6]).is_empty() {
                digit_patterns[5] = digit_pattern;
            } else if (&digit_pattern - &digit_patterns[9]).is_empty() {
                digit_patterns[3] = digit_pattern;
            } else {
                digit_patterns[2] = digit_pattern;
            }
        }

        let mut number = 0;
        for digit_pattern in capture
            .name("number")
            .unwrap()
            .as_str()
            .trim()
            .split(" ")
            .map(|pattern| pattern.chars().collect::<HashSet<char>>())
        {
            for (digit, pattern) in digit_patterns.iter().enumerate() {
                if digit_pattern == *pattern {
                    number *= 10;
                    number += digit;
                }
            }
        }

        sum += number;
    }

    println!("Part 2: {}", sum);
}
