use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<i16> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn count_increasing(input: &[i16]) -> usize {
    input
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

#[aoc(day1, part2)]
fn count_window_increasing(input: &[i16]) -> usize {
    input
        .windows(3)
        .map(|s| s.iter().sum())
        .collect::<Vec<i16>>()
        .as_slice()
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}
