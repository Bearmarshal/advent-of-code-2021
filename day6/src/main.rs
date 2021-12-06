use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;

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
    let mut lanternfish_by_time_to_spawn = [0; 9];

    for time_to_spawn in input
        .trim()
        .split(",")
        .map(str::parse::<usize>)
        .map(Result::unwrap)
    {
        lanternfish_by_time_to_spawn[time_to_spawn] += 1;
    }

    let mut lanternfish_by_time_to_spawn = VecDeque::from(lanternfish_by_time_to_spawn);
    for _ in 0..80 {
        let num_lanterfish_spawning = lanternfish_by_time_to_spawn[0];
        lanternfish_by_time_to_spawn.rotate_left(1);
        lanternfish_by_time_to_spawn[6] += num_lanterfish_spawning;
    }

    println!(
        "Part 1: {}",
        lanternfish_by_time_to_spawn.iter().fold(0, i32::add)
    );
}

fn part2(input: &str) {
    let mut lanternfish_by_time_to_spawn: [u128; 9] = [0; 9];

    for time_to_spawn in input
        .trim()
        .split(",")
        .map(str::parse::<usize>)
        .map(Result::unwrap)
    {
        lanternfish_by_time_to_spawn[time_to_spawn] += 1;
    }

    let mut lanternfish_by_time_to_spawn = VecDeque::from(lanternfish_by_time_to_spawn);
    for _ in 0..256 {
        let num_lanterfish_spawning = lanternfish_by_time_to_spawn[0];
        lanternfish_by_time_to_spawn.rotate_left(1);
        lanternfish_by_time_to_spawn[6] += num_lanterfish_spawning;
    }

    println!(
        "Part 2: {}",
        lanternfish_by_time_to_spawn.iter().fold(0, u128::add)
    );
}
