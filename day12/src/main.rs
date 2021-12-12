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
        static ref LINE_REGEX: Regex = Regex::new(r"\w+-\w+").unwrap();
        static ref NODE_REGEX: Regex = Regex::new(r"(?P<lower>[a-z]+)|(?P<upper>[A-Z]+)").unwrap();
    }
    let mut caves = HashMap::<&str, (HashSet<&str>, bool)>::new();

    for line in input.lines() {
        let mut nodes = [("", false); 2];
        for (i, capture) in NODE_REGEX.captures_iter(line).enumerate() {
            if let Some(node) = capture.name("lower") {
                nodes[i] = (node.as_str(), true);
            } else if let Some(node) = capture.name("upper") {
                nodes[i] = (node.as_str(), false);
            }
        }
        caves
            .entry(nodes[0].0)
            .or_insert((HashSet::<&str>::new(), nodes[0].1))
            .0
            .insert(nodes[1].0);
        caves
            .entry(nodes[1].0)
            .or_insert((HashSet::<&str>::new(), nodes[1].1))
            .0
            .insert(nodes[0].0);
    }

    let mut visited_small_caves = HashSet::<&str>::new();
    let mut paths_to_explore = vec![vec!["start"]];
    let mut current_path = Vec::<&str>::new();
    let mut num_paths = 0;
    while !paths_to_explore.is_empty() {
        if let Some(alternatives) = paths_to_explore.last_mut() {
            if let Some(cave) = alternatives.pop() {
                if cave == "end" {
                    num_paths += 1;
                } else {
                    current_path.push(cave);
                    if caves[cave].1 {
                        visited_small_caves.insert(&cave);
                    }
                    paths_to_explore.push(
                        caves[cave]
                            .0
                            .iter()
                            .filter(|other_cave| !visited_small_caves.contains(**other_cave))
                            .map(|other_cave| *other_cave)
                            .collect::<Vec<&str>>(),
                    );
                }
            } else {
                paths_to_explore.pop();
                if let Some(explored_cave) = current_path.pop() {
                    if caves[explored_cave].1 {
                        visited_small_caves.remove(&explored_cave);
                    }
                }
            }
        }
    }

    println!("Part 1: {}", num_paths);
}

fn part2(input: &str) {
    lazy_static! {
        static ref LINE_REGEX: Regex = Regex::new(r"\w+-\w+").unwrap();
        static ref NODE_REGEX: Regex = Regex::new(r"(?P<lower>[a-z]+)|(?P<upper>[A-Z]+)").unwrap();
    }
    let mut caves = HashMap::<&str, (HashSet<&str>, bool)>::new();

    for line in input.lines() {
        let mut nodes = [("", false); 2];
        for (i, capture) in NODE_REGEX.captures_iter(line).enumerate() {
            if let Some(node) = capture.name("lower") {
                nodes[i] = (node.as_str(), true);
            } else if let Some(node) = capture.name("upper") {
                nodes[i] = (node.as_str(), false);
            }
        }
        caves
            .entry(nodes[0].0)
            .or_insert((HashSet::<&str>::new(), nodes[0].1))
            .0
            .insert(nodes[1].0);
        caves
            .entry(nodes[1].0)
            .or_insert((HashSet::<&str>::new(), nodes[1].1))
            .0
            .insert(nodes[0].0);
    }

    let mut visited_small_caves = HashSet::<&str>::new();
    let mut paths_to_explore = vec![vec!["start"]];
    let mut current_path = Vec::<&str>::new();
    let mut num_paths = 0;
    let mut visited_twice: Option<&str> = None;
    while !paths_to_explore.is_empty() {
        if let Some(alternatives) = paths_to_explore.last_mut() {
            if let Some(cave) = alternatives.pop() {
                if cave == "end" {
                    num_paths += 1;
                } else {
                    current_path.push(cave);
                    if caves[cave].1 {
                        if visited_small_caves.contains(&cave) {
                            visited_twice = Some(&cave);
                        } else {
                            visited_small_caves.insert(&cave);
                        }
                    }
                    paths_to_explore.push(
                        caves[cave]
                            .0
                            .iter()
                            .filter(|other_cave| {
                                visited_twice.is_none() && **other_cave != "start"
                                    || !visited_small_caves.contains(**other_cave)
                            })
                            .map(|other_cave| *other_cave)
                            .collect::<Vec<&str>>(),
                    );
                }
            } else {
                paths_to_explore.pop();
                if let Some(explored_cave) = current_path.pop() {
                    if caves[explored_cave].1 {
                        if visited_twice == Some(&explored_cave) {
                            visited_twice = None;
                        } else {
                            visited_small_caves.remove(&explored_cave);
                        }
                    }
                }
            }
        }
    }

    println!("Part 2: {}", num_paths);
}
