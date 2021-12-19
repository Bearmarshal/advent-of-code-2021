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
            Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
    }
    let mut vents = HashMap::<(i32, i32), i32>::new();

    for capture in LINE_REGEX.captures_iter(input) {
        let mut x1 = capture["x1"].parse::<i32>().unwrap();
        let mut y1 = capture["y1"].parse::<i32>().unwrap();
        let mut x2 = capture["x2"].parse::<i32>().unwrap();
        let mut y2 = capture["y2"].parse::<i32>().unwrap();

        if x1 == x2 {
            if y1 > y2 {
                mem::swap(&mut y1, &mut y2);
            }
            for y in y1..=y2 {
                *vents.entry((x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            if x1 > x2 {
                mem::swap(&mut x1, &mut x2);
            }
            for x in x1..=x2 {
                *vents.entry((x, y1)).or_insert(0) += 1;
            }
        }
    }

    println!("Part 1: {}", vents.values().filter(|v| **v > 1).count());
}

fn part2(input: &str) {
    lazy_static! {
        static ref LINE_REGEX: Regex =
            Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
    }
    let mut vents = HashMap::<(i32, i32), i32>::new();

    for capture in LINE_REGEX.captures_iter(input) {
        let x1 = capture["x1"].parse::<i32>().unwrap();
        let y1 = capture["y1"].parse::<i32>().unwrap();
        let x2 = capture["x2"].parse::<i32>().unwrap();
        let y2 = capture["y2"].parse::<i32>().unwrap();

        for (x, y) in Line2dIter::new((x1, y1), (x2, y2)) {
            *vents.entry((x, y)).or_insert(0) += 1;
        }
    }

    println!("Part 2: {}", vents.values().filter(|v| **v > 1).count());
}

struct Line2dIter {
    x0: i32,
    y0: i32,
    dx: i32,
    dy: i32,
    at: i32,
}

impl Line2dIter {
    pub fn new(p1: (i32, i32), p2: (i32, i32)) -> Line2dIter {
        let (x1, y1) = p1;
        let (x2, y2) = p2;

        Line2dIter {
            x0: x1,
            y0: y1,
            dx: x2 - x1,
            dy: y2 - y1,
            at: -1,
        }
    }
}

impl Iterator for Line2dIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        self.at += 1;
        if self.at > i32::max(self.dx.abs(), self.dy.abs()) {
            None
        } else {
            if self.dx.abs() == self.dy.abs() {
                Some((
                    self.x0 + self.at * self.dx.signum(),
                    self.y0 + self.at * self.dy.signum(),
                ))
            } else if self.dx.abs() > self.dy.abs() {
                Some((
                    self.x0 + self.at * self.dx.signum(),
                    self.y0 + self.at * self.dy / self.dx.abs(),
                ))
            } else {
                Some((
                    self.x0 + self.at * self.dx / self.dy.abs(),
                    self.y0 + self.at * self.dy.signum(),
                ))
            }
        }
    }
}
