use std::collections::{HashMap, VecDeque};
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
    let height_map = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let y_max = height_map.len();
    let x_max = height_map[0].len();

    let mut risk_level = 0;
    for y in 0..y_max {
        'height_loop: for x in 0..x_max {
            let height = height_map[y][x];
            for yy in y.checked_sub(1).unwrap_or(0)..=(y + 1).min(y_max - 1) {
                for xx in x.checked_sub(1).unwrap_or(0)..=(x + 1).min(x_max - 1) {
                    if (yy == y || xx == x) && !(yy == y && xx == x) && height_map[yy][xx] <= height
                    {
                        continue 'height_loop;
                    }
                }
            }
            risk_level += height + 1;
        }
    }

    println!("Part 1: {}", risk_level);
}

fn part2(input: &str) {
    let mut height_map = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .enumerate()
        })
        .enumerate()
        .flat_map(move |(y, x_heights)| x_heights.map(move |(x, height)| ((x + 1, y + 1), height)))
        .collect::<HashMap<(usize, usize), u32>>();
    let mut coordinates_to_check = VecDeque::<(usize, usize)>::new();
    let mut basin_sizes = Vec::<i32>::new();

    while !height_map.is_empty() {
        let (y, x) = match height_map.keys().next().unwrap() {
            (y, x) => (*y, *x),
        };
        let height = height_map.remove(&(y, x)).unwrap();

        if height < 9 {
            let mut basin_size = 0;
            coordinates_to_check.push_back((y, x));
            while !coordinates_to_check.is_empty() {
                let (y, x) = coordinates_to_check.pop_front().unwrap();
                basin_size += 1;
                for coordinates in [(y - 1, x), (y, x - 1), (y, x + 1), (y + 1, x)] {
                    if let Some(height) = height_map.remove(&coordinates) {
                        if height < 9 {
                            coordinates_to_check.push_back(coordinates);
                        }
                    }
                }
            }
            basin_sizes.push(basin_size);
        }
    }
    basin_sizes.sort();

    println!(
        "Part 2: {}",
        basin_sizes.iter().rev().take(3).fold(1, |a, b| a * b)
    );
}
