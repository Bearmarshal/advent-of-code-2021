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
    println!("Part 2: {}", "");
}
