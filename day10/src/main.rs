use std::collections::HashMap;
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
    let matching_brace = HashMap::from([
        (')', ('(', 3)),
        (']', ('[', 57)),
        ('}', ('{', 1197)),
        ('>', ('<', 25137)),
    ]);
    let mut corruption_score = 0;
    let mut opening_braces = Vec::<char>::new();
    for line in input.lines() {
        for brace in line.trim().chars() {
            if !matching_brace.contains_key(&brace) {
                opening_braces.push(brace);
            } else {
                let (matching_brace, score) = matching_brace[&brace];
                match opening_braces.pop() {
                    Some(opening_brace) if opening_brace == matching_brace => (),
                    _ => corruption_score += score,
                }
            }
        }
    }

    println!("Part 1: {}", corruption_score);
}

fn part2(input: &str) {
    let matching_brace = HashMap::from([
        ('(', (')', 1)),
        ('[', (']', 2)),
        ('{', ('}', 3)),
        ('<', ('>', 4)),
    ]);
    let mut completion_scores = Vec::<i64>::new();
    let mut opening_braces = Vec::<char>::new();
    'lines: for line in input.lines() {
        for brace in line.trim().chars() {
            if matching_brace.contains_key(&brace) {
                opening_braces.push(brace);
            } else if let Some(opening_brace) = opening_braces.pop() {
                match matching_brace[&opening_brace] {
                    (closing_brace, _) if closing_brace == brace => (),
                    _ => {
                        opening_braces.clear();
                        continue 'lines;
                    }
                }
            }
        }

        let mut score: i64 = 0;
        for brace in opening_braces.drain(..).rev() {
            score *= 5;
            score += matching_brace[&brace].1;
        }
        completion_scores.push(score);
    }
    completion_scores.sort();

    println!("Part 2: {}", completion_scores[completion_scores.len() / 2]);
}
