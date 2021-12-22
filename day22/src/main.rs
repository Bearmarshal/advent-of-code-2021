use crate::Action::*;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

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
        static ref CUBOID_REGEX: Regex =
            Regex::new(r"(?P<action>on|off) x=(?P<x_start>-?\d+)\.\.(?P<x_end>-?\d+),y=(?P<y_start>-?\d+)\.\.(?P<y_end>-?\d+),z=(?P<z_start>-?\d+)\.\.(?P<z_end>-?\d+)").unwrap();
    }

    let cuboids = CUBOID_REGEX
        .captures_iter(input)
        .map(|capture| parse_cuboid(capture).unwrap())
        .filter_map(|(action, cuboid)| match Cuboid::new(-50, 50, -50, 50, -50, 50).intersection(&cuboid) {
            Some(cuboid) => Some((action, cuboid)),
            None => None,
        })
        .collect::<Vec<_>>();

    let mut shadowing_set = Vec::new();
    let mut unshadowing_set = Vec::new();

    let mut num_active_cubes = 0;
    for (action, cuboid) in cuboids.into_iter().rev() {
        let (volume, new_shadowing_set, new_unshadowing_set) =
            cuboid.unshadowed_volume(shadowing_set, unshadowing_set);
        if action == On {
            num_active_cubes += volume;
        }
        shadowing_set = new_shadowing_set;
        unshadowing_set = new_unshadowing_set;
    }

    println!("Part 1: {}", num_active_cubes);
}

fn part2(input: &str) {
    lazy_static! {
        static ref CUBOID_REGEX: Regex =
        Regex::new(r"(?P<action>on|off) x=(?P<x_start>-?\d+)\.\.(?P<x_end>-?\d+),y=(?P<y_start>-?\d+)\.\.(?P<y_end>-?\d+),z=(?P<z_start>-?\d+)\.\.(?P<z_end>-?\d+)").unwrap();
    }

    let cuboids = CUBOID_REGEX
        .captures_iter(input)
        .map(|capture| parse_cuboid(capture).unwrap())
        .collect::<Vec<_>>();

    let mut shadowing_set = Vec::new();
    let mut unshadowing_set = Vec::new();

    let mut num_active_cubes = 0;
    for (action, cuboid) in cuboids.into_iter().rev() {
        let (volume, new_shadowing_set, new_unshadowing_set) =
            cuboid.unshadowed_volume(shadowing_set, unshadowing_set);
        if action == On {
            num_active_cubes += volume;
        }
        shadowing_set = new_shadowing_set;
        unshadowing_set = new_unshadowing_set;
    }

    println!("Part 2: {}", num_active_cubes);
}

fn parse_cuboid(capture: Captures) -> Option<(Action, Cuboid)> {
    let action = Action::from_str(&capture["action"]).ok()?;
    let x_start = capture["x_start"].parse::<i32>().ok()?;
    let x_end = capture["x_end"].parse::<i32>().ok()?;
    let y_start = capture["y_start"].parse::<i32>().ok()?;
    let y_end = capture["y_end"].parse::<i32>().ok()?;
    let z_start = capture["z_start"].parse::<i32>().ok()?;
    let z_end = capture["z_end"].parse::<i32>().ok()?;
    Some((
        action,
        Cuboid::new(x_start, x_end, y_start, y_end, z_start, z_end),
    ))
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Action {
    On,
    Off,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(On),
            "off" => Ok(Off),
            _ => Err(format!("{} is neither on nor off", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Cuboid {
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
    z_start: i32,
    z_end: i32,
}

impl Cuboid {
    fn new(x_start: i32, x_end: i32, y_start: i32, y_end: i32, z_start: i32, z_end: i32) -> Cuboid {
        Cuboid {
            x_start,
            x_end,
            y_start,
            y_end,
            z_start,
            z_end,
        }
    }

    fn overlaps(&self, cuboid: &Self) -> bool {
        let x_overlap = self.x_end >= cuboid.x_start && cuboid.x_end >= self.x_start;
        let y_overlap = self.y_end >= cuboid.y_start && cuboid.y_end >= self.y_start;
        let z_overlap = self.z_end >= cuboid.z_start && cuboid.z_end >= self.z_start;
        x_overlap && y_overlap && z_overlap
    }

    fn volume(&self) -> usize {
        (self.x_end + 1 - self.x_start) as usize
            * (self.y_end + 1 - self.y_start) as usize
            * (self.z_end + 1 - self.z_start) as usize
    }

    fn intersection(&self, cuboid: &Cuboid) -> Option<Cuboid> {
        if self.overlaps(cuboid) {
            Some(Cuboid::new(
                i32::max(self.x_start, cuboid.x_start),
                i32::min(self.x_end, cuboid.x_end),
                i32::max(self.y_start, cuboid.y_start),
                i32::min(self.y_end, cuboid.y_end),
                i32::max(self.z_start, cuboid.z_start),
                i32::min(self.z_end, cuboid.z_end),
            ))
        } else {
            None
        }
    }

    fn unshadowed_volume(
        &self,
        shadowing_set: Vec<Cuboid>,
        unshadowing_set: Vec<Cuboid>,
    ) -> (usize, Vec<Cuboid>, Vec<Cuboid>) {
        let shadowing_intersections = shadowing_set
            .iter()
            .filter_map(|cuboid| self.intersection(cuboid))
            .collect::<Vec<_>>();
        let mut unshadowing_intersections = unshadowing_set
            .iter()
            .filter_map(|cuboid| self.intersection(cuboid))
            .collect::<Vec<_>>();
        unshadowing_intersections.push(*self);
        let volume = unshadowing_intersections
            .iter()
            .map(Cuboid::volume)
            .sum::<usize>()
            - shadowing_intersections
                .iter()
                .map(Cuboid::volume)
                .sum::<usize>();
        (
            volume,
            shadowing_set.into_iter().chain(unshadowing_intersections.into_iter()).collect::<Vec<_>>(),
            unshadowing_set.into_iter().chain(shadowing_intersections.into_iter()).collect::<Vec<_>>(),
        )
    }
}
