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
        static ref INSERTION_REGEX: Regex =
            Regex::new(r"target area: x=(?P<x_min>\d+)\.\.(?P<x_max>\d+), y=(?P<y_min>-?\d+)\.\.(?P<y_max>-?\d+)").unwrap();
    }

    let target_capture = INSERTION_REGEX.captures(input).unwrap();
    let target_x_min = target_capture["x_min"].parse::<i32>().unwrap();
    let target_x_max = target_capture["x_max"].parse::<i32>().unwrap();
    let target_y_min = target_capture["y_min"].parse::<i32>().unwrap();
    let target_y_max = target_capture["y_max"].parse::<i32>().unwrap();
    let target_x = target_x_min..=target_x_max;
    let target_y = target_y_min..=target_y_max;

    let mut highest_y_velocity = 0;
    'y_velocity: for y_velocity in target_y_min..-1 {
        let mut steps = 0;

        loop {
            let depth = steps * (2 * y_velocity - steps - 1) / 2;
            if target_y.contains(&depth) {
                steps += 2 * -y_velocity;
                for x_velocity in 0..=(target_x_max / steps + steps / 2) {
                    if x_velocity < steps {
                        if target_x.contains(&(x_velocity * (x_velocity + 1) / 2)) {
                            highest_y_velocity = -y_velocity;
                            break 'y_velocity;
                        }
                    } else {
                        if target_x.contains(&(steps * (x_velocity + 1 - steps) / 2)) {
                            highest_y_velocity = -y_velocity;
                            break 'y_velocity;
                        } else if target_x_max < steps * (x_velocity + 1 - steps) / 2 {
                            break;
                        }
                    }
                }
                steps -= 2 * y_velocity;
            } else if depth < target_y_min {
                break;
            }
            steps += 1;
        }
    }

    println!(
        "Part 1: {}",
        highest_y_velocity * (highest_y_velocity + 1) / 2
    );
}

fn part2(input: &str) {
    lazy_static! {
        static ref INSERTION_REGEX: Regex =
            Regex::new(r"target area: x=(?P<x_min>\d+)\.\.(?P<x_max>\d+), y=(?P<y_min>-?\d+)\.\.(?P<y_max>-?\d+)").unwrap();
    }

    let target_capture = INSERTION_REGEX.captures(input).unwrap();
    let target_x_min = target_capture["x_min"].parse::<i32>().unwrap();
    let target_x_max = target_capture["x_max"].parse::<i32>().unwrap();
    let target_y_min = target_capture["y_min"].parse::<i32>().unwrap();
    let target_y_max = target_capture["y_max"].parse::<i32>().unwrap();
    let target_x = target_x_min..=target_x_max;
    let target_y = target_y_min..=target_y_max;

    let mut applicable_velocities = HashSet::new();
    for y_velocity in target_y_min..=0 {
        let mut steps = 0;

        loop {
            let depth = steps * (2 * y_velocity - (steps - 1)) / 2;
            if target_y.contains(&depth) {
                for x_velocity in (target_x_min / (steps + 1)).. {
                    if x_velocity < steps {
                        let distance = x_velocity * (x_velocity + 1) / 2;
                        if target_x.contains(&distance) {
                            applicable_velocities.insert((x_velocity, y_velocity));
                        } else if distance > target_x_max {
                            break;
                        }
                    } else {
                        let distance = steps * (2 * x_velocity + 1 - steps) / 2;
                        if target_x.contains(&distance) {
                            applicable_velocities.insert((x_velocity, y_velocity));
                        } else if distance > target_x_max {
                            break;
                        }
                    }
                }

                if y_velocity != 0 {
                    steps += 2 * -y_velocity - 1;
                    for x_velocity in (target_x_min / (steps + 1)).. {
                        if x_velocity < steps {
                            let distance = x_velocity * (x_velocity + 1) / 2;
                            if target_x.contains(&distance) {
                                applicable_velocities.insert((x_velocity, -y_velocity - 1));
                            } else if distance > target_x_max {
                                break;
                            }
                        } else {
                            let distance = steps * (2 * x_velocity + 1 - steps) / 2;
                            if target_x.contains(&distance) {
                                applicable_velocities.insert((x_velocity, -y_velocity - 1));
                            } else if distance > target_x_max {
                                break;
                            }
                        }
                    }
                    steps -= 2 * -y_velocity - 1;
                }
            } else if depth < target_y_min {
                break;
            }
            steps += 1;
        }
    }

    println!("Part 2: {}", applicable_velocities.len());
}
