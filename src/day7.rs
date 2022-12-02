use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<u64>;

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Input {
    input
        .split(",")
        .map(|p| p.parse::<u64>().expect("Non numeric input"))
        .collect()
}

#[aoc(day7, part1)]
fn find_align(input: &Input) -> u64 {
    let (mut min, mut total) = (1, u64::MAX);

    for test in 1..*input.into_iter().max().unwrap() {
        let new_total = input.iter().map(|n| n.abs_diff(test)).sum();
        if new_total < total {
            min = test;
            total = new_total;
        } else {
            // as soon as we reach this - we are
            // moving away from the correct solution - stop now
            return total;
        }
    }

    unreachable!()
}

#[aoc(day7, part2)]
fn find_exp(input: &Input) -> u64 {
    let (mut min, mut total) = (1, u64::MAX);

    for test in 1..*input.into_iter().max().unwrap() {
        let new_total = input
            .iter()
            .map(|n| (0..=n.abs_diff(test)).sum::<u64>())
            .sum();
        if new_total < total {
            min = test;
            total = new_total;
        }
    }

    return total;
}
