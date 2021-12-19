use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::mem::swap;
use std::ops::{Add, Mul, Sub};

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
        static ref EMPTY_LINE_REGEX: Regex = Regex::new(r"\r?\n\r?\n").unwrap();
        static ref SCANNER_REGEX: Regex = Regex::new(r"--- scanner (?P<scanner>\d+) ---").unwrap();
        static ref BEACON_REGEX: Regex = Regex::new(r"(-?\d+),(-?\d+),(-?\d+)").unwrap();
    }

    let mut absolute_positions = HashSet::<Coordinate3D>::new();

    let mut unresolved_set = HashMap::<u32, Scanner>::new();
    let mut open_set = Vec::<Scanner>::new();
    let mut closed_set = Vec::<Scanner>::new();

    let mut rotation_sets = Vec::new();
    let mut rotation_matrices = Vec::<RotationMatrix3D>::new();
    let mut i = 0;
    let signs = [1, -1];
    for x_sign in signs {
        for x in 0..3 {
            let mut x_vector = [0, 0, 0];
            x_vector[x] = x_sign;
            for y_sign in signs {
                for y in (x + 1..=x + 2).map(|j| j % 3) {
                    let mut y_vector = [0, 0, 0];
                    y_vector[y] = y_sign;

                    let z = 3 - x - y;
                    let z_sign = signs[i % 2] * x_sign * y_sign;
                    let mut z_vector = [0, 0, 0];
                    z_vector[z] = z_sign;

                    rotation_matrices.push(RotationMatrix3D([x_vector, y_vector, z_vector]));

                    i += 1;
                }
            }
        }
    }
    for _ in 0..24 {
        rotation_sets.push(HashSet::<Coordinate3D>::new())
    }

    let mut scanner_data_blocks = EMPTY_LINE_REGEX.split(input);
    let mut beacons = HashSet::new();

    for beacon_capture in BEACON_REGEX.captures_iter(scanner_data_blocks.next().unwrap()) {
        let beacon_position = Coordinate3D([
            beacon_capture[1].parse().unwrap(),
            beacon_capture[2].parse().unwrap(),
            beacon_capture[3].parse().unwrap(),
        ]);
        beacons.insert(beacon_position);
        absolute_positions.insert(beacon_position);
    }
    let scanner = Scanner::new(0, Coordinate3D([0, 0, 0]), beacons);
    open_set.push(scanner);

    for scanner_data in scanner_data_blocks {
        let scanner_id: u32 = SCANNER_REGEX.captures(scanner_data).unwrap()["scanner"]
            .parse()
            .unwrap();
        let mut beacons = HashSet::new();
        for beacon_capture in BEACON_REGEX.captures_iter(scanner_data) {
            let relative_position = Coordinate3D([
                beacon_capture[1].parse().unwrap(),
                beacon_capture[2].parse().unwrap(),
                beacon_capture[3].parse().unwrap(),
            ]);
            beacons.insert(relative_position);
        }
        let scanner = Scanner::new(scanner_id, Coordinate3D([0, 0, 0]), beacons);
        unresolved_set.insert(scanner_id, scanner);
    }

    while let Some(scanner) = open_set.pop() {
        let mut resolved_set = HashSet::new();
        for (
            id,
            Scanner {
                beacons: relative_beacons,
                inner_vectors,
                ..
            },
        ) in unresolved_set.iter()
        {
            if scanner.inner_vectors.intersection(inner_vectors).count() < 66 {
                // 12 * (12 - 1) / 2
                continue;
            }

            'rotations: for rotated_coordinates in rotation_matrices.iter().map(|rotation| {
                relative_beacons
                    .iter()
                    .map(|coordinates| *rotation * *coordinates)
                    .collect::<HashSet<Coordinate3D>>()
            }) {
                for rotated_coordinate in rotated_coordinates.iter() {
                    'next_comparison: for absolute_coordinate in scanner.beacons.iter() {
                        let offset = absolute_coordinate - rotated_coordinate;
                        let offset_set = &rotated_coordinates + offset;

                        //let overlap_count = scanner.beacons.intersection(&offset_set).count();
                        let mut overlap_count = 0;
                        for absolute_coordinate in scanner.beacons.iter() {
                            if offset_set.contains(absolute_coordinate) {
                                overlap_count += 1;
                            } else {
                                let [ox, oy, oz] = offset.0;
                                let [ax, ay, az] = absolute_coordinate.0;
                                if (ox - 1000..=ox + 1000).contains(&ax)
                                    && (oy - 1000..=oy + 1000).contains(&ay)
                                    && (oz - 1000..=oz + 1000).contains(&az)
                                {
                                    continue 'next_comparison;
                                }
                            }
                        }

                        if overlap_count >= 12 {
                            resolved_set.insert(Scanner {
                                id: *id,
                                position: offset,
                                beacons: offset_set,
                                inner_vectors: inner_vectors.clone(),
                            });
                            break 'rotations;
                        }
                    }
                }
            }
        }
        for resolved_scanner in resolved_set {
            for coordinate in resolved_scanner.beacons.iter() {
                absolute_positions.insert(*coordinate);
            }
            unresolved_set.remove(&resolved_scanner.id);
            open_set.push(resolved_scanner);
        }
        closed_set.push(scanner);
    }

    println!("Part 1: {}", absolute_positions.len());

    let manhattan_distance: i32 = closed_set
        .iter()
        .flat_map(|scanner| {
            closed_set.iter().map(|other| {
                (&scanner.position - other.position)
                    .0
                    .into_iter()
                    .fold(0, |a, b| a + b.abs())
            })
        })
        .max()
        .unwrap();
    println!("Part 2: {}", manhattan_distance);
}

fn part2(_input: &str) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate3D([i32; 3]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RotationMatrix3D([[i32; 3]; 3]);

impl Mul<Coordinate3D> for RotationMatrix3D {
    type Output = Coordinate3D;

    fn mul(self, rhs: Coordinate3D) -> Self::Output {
        let mut output = Coordinate3D([0, 0, 0]);
        for i in 0..3 {
            for j in 0..3 {
                output.0[i] += self.0[j][i] * rhs.0[j];
            }
        }
        output
    }
}

impl Coordinate3D {
    fn abs(&self) -> Self {
        Self([self.0[0].abs(), self.0[1].abs(), self.0[2].abs()])
    }

    fn normalise(&mut self) {
        let [mut x, mut y, mut z] = self.0;
        if x > y {
            swap(&mut x, &mut y)
        }
        if x > z {
            swap(&mut x, &mut z)
        }
        if y > z {
            swap(&mut y, &mut z)
        }
        *self = Self([x, y, z]);
    }
}

impl Add<Self> for &Coordinate3D {
    type Output = Coordinate3D;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate3D([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl Sub<Self> for &Coordinate3D {
    type Output = Coordinate3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate3D([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Add<Coordinate3D> for &Coordinate3D {
    type Output = Coordinate3D;

    fn add(self, rhs: Coordinate3D) -> Self::Output {
        Coordinate3D([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl Sub<Coordinate3D> for &Coordinate3D {
    type Output = Coordinate3D;

    fn sub(self, rhs: Coordinate3D) -> Self::Output {
        Coordinate3D([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Add<Coordinate3D> for &HashSet<Coordinate3D> {
    type Output = HashSet<Coordinate3D>;

    fn add(self, rhs: Coordinate3D) -> Self::Output {
        self.iter()
            .map(|coordinate| coordinate + &rhs)
            .collect::<HashSet<Coordinate3D>>()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Scanner {
    id: u32,
    position: Coordinate3D,
    beacons: HashSet<Coordinate3D>,
    inner_vectors: HashSet<Coordinate3D>,
}

impl Scanner {
    fn new(id: u32, position: Coordinate3D, beacons: HashSet<Coordinate3D>) -> Self {
        let mut inner_vectors = HashSet::new();
        for beacon in beacons.iter() {
            for other in beacons.iter() {
                if beacon != other {
                    let mut inner_vector = (beacon - other).abs();
                    inner_vector.normalise();
                    inner_vectors.insert(inner_vector);
                }
            }
        }
        let num_lost_inner_vectors = beacons.len() * (beacons.len() - 1) / 2 - inner_vectors.len();
        if num_lost_inner_vectors != 0 {
            println!(
                "Warning: {:?} inner vectors lost from scanner {} due to shadowing",
                num_lost_inner_vectors, id
            );
        }

        Self {
            id,
            position,
            beacons,
            inner_vectors,
        }
    }
}

impl Hash for Scanner {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
