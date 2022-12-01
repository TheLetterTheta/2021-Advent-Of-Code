use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type Input = HashMap<u8, usize>;

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Input {
    let mut map = HashMap::with_capacity(8);

    for fish in input
        .split(",")
        .map(|f| f.parse::<u8>().expect("Invalid input"))
    {
        map.entry(fish).and_modify(|count| *count += 1).or_insert(1);
    }

    map
}

#[aoc(day6, part1)]
fn days_80(input: &Input) -> usize {
    let mut input = input.into_iter().map(|(&l, &r)| (l, r)).collect::<Vec<(u8, usize)>>();

    for _ in 0..80 {
        let mut map = HashMap::with_capacity(8);

        for (days, fish) in input.into_iter().flat_map(|(days, count)| {
            if days == 0 {
                vec![(6, count), (8, count)]
            } else {
                vec![(days - 1, count)]
            }
        }) {
            map.entry(days)
                .and_modify(|count| *count += fish)
                .or_insert(fish);
        }

        input = map.into_iter().collect::<Vec<(u8, usize)>>();
    }

    input.iter().map(|(_, f)| f).sum()
}

#[aoc(day6, part2)]
fn days_256(input: &Input) -> usize {
    let mut input = input.into_iter().map(|(&l, &r)| (l, r)).collect::<Vec<(u8, usize)>>();

    for _ in 0..256 {
        let mut map = HashMap::with_capacity(8);

        for (days, fish) in input.into_iter().flat_map(|(days, count)| {
            if days == 0 {
                vec![(6, count), (8, count)]
            } else {
                vec![(days - 1, count)]
            }
        }) {
            map.entry(days)
                .and_modify(|count| *count += fish)
                .or_insert(fish);
        }

        input = map.into_iter().collect::<Vec<(u8, usize)>>();
    }

    input.iter().map(|(_, f)| f).sum()
}
