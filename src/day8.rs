use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::str::FromStr;

struct Line(Vec<HashSet<u8>>, Vec<HashSet<u8>>);

type Input = Vec<Line>;

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let (left, right) = s.split_once(" | ").unwrap();

        Ok(Line(
            left.split_whitespace()
                .map(|s| s.chars().map(|c| c as u8).collect::<HashSet<u8>>())
                .collect::<Vec<_>>(),
            right
                .split_whitespace()
                .map(|s| s.chars().map(|c| c as u8).collect::<HashSet<u8>>())
                .collect::<Vec<_>>(),
        ))
    }
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Input {
    input.lines().map(|l| Line::from_str(l).unwrap()).collect()
}

#[aoc(day8, part1)]
fn solve_part1(input: &Input) -> usize {
    input
        .iter()
        .map(|Line(_, r)| {
            r.iter()
                .map(|c| c.len())
                .filter(|&c| c == 2 || c == 3 || c == 7 || c == 4)
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
fn solve_part2(input: &Input) -> u64 {
    todo!();
}
