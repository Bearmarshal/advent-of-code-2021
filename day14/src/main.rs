use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::once;

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
        static ref INSERTION_REGEX: Regex =
            Regex::new(r"(?P<pair>[A-Z]{2}) -> (?P<element>[A-Z])").unwrap();
    }
    let (polymer_template, insertions_section) = input.split_once("\n\n").unwrap();
    let insertions = INSERTION_REGEX
        .captures_iter(insertions_section)
        .map(|capture| {
            (
                (
                    capture["pair"].chars().nth(0).unwrap(),
                    capture["pair"].chars().nth(1).unwrap(),
                ),
                capture["element"].chars().nth(0).unwrap(),
            )
        })
        .collect::<HashMap<(char, char), char>>();
    let mut polymer = polymer_template.trim().to_string();

    for _ in 0..10 {
        let polymer_elements = polymer.chars().chain(once(' ')).collect::<Vec<char>>();
        polymer = polymer_elements
            .windows(2)
            .flat_map(|pair| {
                if let Some(insertion) = insertions.get(&(pair[0], pair[1])) {
                    vec![pair[0], *insertion]
                } else {
                    vec![pair[0]]
                }
            })
            .collect::<String>();
    }
    let element_counts =
        polymer
            .chars()
            .fold(HashMap::<char, u32>::new(), |mut counts, element| {
                *counts.entry(element).or_insert(0) += 1;
                counts
            });

    println!(
        "Part 1: {}",
        element_counts.values().max().unwrap() - element_counts.values().min().unwrap()
    );
}

fn part2(input: &str) {
    lazy_static! {
        static ref INSERTION_REGEX: Regex =
            Regex::new(r"(?P<pair>[A-Z]{2}) -> (?P<element>[A-Z])").unwrap();
    }
    let (polymer_template, insertions_section) = input.split_once("\n\n").unwrap();
    let insertions = INSERTION_REGEX
        .captures_iter(insertions_section)
        .map(|capture| {
            (
                (
                    capture["pair"].chars().nth(0).unwrap(),
                    capture["pair"].chars().nth(1).unwrap(),
                ),
                capture["element"].chars().nth(0).unwrap(),
            )
        })
        .collect::<HashMap<(char, char), char>>();

    let mut partial_computations: HashMap<(char, char, i32), HashMap<char, u64>> = HashMap::new();
    let mut element_counts: HashMap<char, u64> = HashMap::new();
    for pair in polymer_template
        .chars()
        .chain(once(' '))
        .collect::<Vec<char>>()
        .windows(2)
    {
        for (element, count) in count_elements_in_subpolymer_after_insertions(
            pair[0],
            pair[1],
            40,
            &insertions,
            &mut partial_computations,
        ) {
            *element_counts.entry(*element).or_insert(0) += count;
        }
    }

    println!(
        "Part 2: {}",
        element_counts.values().max().unwrap() - element_counts.values().min().unwrap()
    )
}

fn count_elements_in_subpolymer_after_insertions<'a>(
    element: char,
    next_element: char,
    steps: i32,
    insertions: &HashMap<(char, char), char>,
    partial_computations: &'a mut HashMap<(char, char, i32), HashMap<char, u64>>,
) -> &'a HashMap<char, u64> {
    if !partial_computations.contains_key(&(element, next_element, steps)) {
        match steps {
            _ if !insertions.contains_key(&(element, next_element)) => partial_computations.insert(
                (element, next_element, steps),
                HashMap::from([(element, 1)]),
            ),
            0 => partial_computations.insert(
                (element, next_element, steps),
                HashMap::from([(element, 1)]),
            ),
            _ => {
                let inserted_element = insertions[&(element, next_element)];
                let mut element_counts = HashMap::<char, u64>::new();
                for (element, count) in count_elements_in_subpolymer_after_insertions(
                    element,
                    inserted_element,
                    steps - 1,
                    insertions,
                    partial_computations,
                ) {
                    *element_counts.entry(*element).or_insert(0) += count;
                }
                for (element, count) in count_elements_in_subpolymer_after_insertions(
                    inserted_element,
                    next_element,
                    steps - 1,
                    insertions,
                    partial_computations,
                ) {
                    *element_counts.entry(*element).or_insert(0) += count;
                }
                partial_computations.insert((element, next_element, steps), element_counts)
            }
        };
    }
    &partial_computations[&(element, next_element, steps)]
}
