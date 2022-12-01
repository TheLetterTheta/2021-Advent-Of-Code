use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Vec<u16>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    _ => unreachable!("INVALID INPUT"),
                })
                .collect()
        })
        .collect()
}

#[aoc(day3, part1)]
fn count_increasing(input: &[Vec<u16>]) -> usize {
    let count_threshold = input.len() / 2;

    let mut input = input.iter();
    let first: &Vec<u16> = input.next().expect("");

    let column_totals = input.fold(first.clone(), |acc, x| {
        acc.iter().zip(x.iter()).map(|(l, r)| *l + r).collect()
    });

    let mut epsillon = 0;
    let mut delta = 0;

    for n in column_totals.iter() {
        epsillon <<= 1;
        delta <<= 1;
        if *n as usize > count_threshold {
            epsillon += 1;
        } else {
            delta += 1;
        }
    }

    epsillon * delta
}

struct BitArray<'a>(&'a Vec<u16>);
impl<'a> From<BitArray<'a>> for usize {
    fn from(arr: BitArray) -> usize {
        arr.0.iter().fold(0, |acc, x| (acc << 1) + x) as usize
    }
}

#[aoc(day3, part2)]
fn most_matching(input: &[Vec<u16>]) -> usize {
    let mut common_search = input.iter().cloned().collect_vec();

    let mut index = 0;
    while common_search.len() > 1 {
        // find most common value at {index}
        let (zeros, ones): (Vec<Vec<u16>>, Vec<Vec<u16>>) =
            common_search.iter().cloned().partition(|n| n[index] == 0);
        if zeros.len() > ones.len() {
            common_search = zeros.clone();
        } else {
            common_search = ones.clone();
        }

        index += 1;
    }

    let mut uncommon_search = input.iter().cloned().collect_vec();

    let mut index = 0;
    while uncommon_search.len() > 1 {
        // find most common value at {index}
        let (zeros, ones): (Vec<Vec<u16>>, Vec<Vec<u16>>) =
            uncommon_search.iter().cloned().partition(|n| n[index] == 0);
        if zeros.len() > ones.len() {
            uncommon_search = ones.clone();
        } else {
            uncommon_search = zeros.clone();
        }

        index += 1;
    }

    let common = usize::from(BitArray(&common_search[0]));
    let uncommon = usize::from(BitArray(&uncommon_search[0]));

    common * uncommon
}
