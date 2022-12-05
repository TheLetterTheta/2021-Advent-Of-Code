use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::str::FromStr;

struct Line(Vec<HashSet<char>>, Vec<HashSet<char>>);

type Input = Vec<Line>;

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let (left, right) = s.split_once(" | ").unwrap();

        Ok(Line(
            left.split_whitespace()
                .map(|s| s.chars().collect::<HashSet<char>>())
                .collect::<Vec<_>>(),
            right
                .split_whitespace()
                .map(|s| s.chars().collect::<HashSet<char>>())
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
fn solve_part2(input: &Input) -> usize {
    input
        .iter()
        .map(|Line(left, right)| {
            let one = left.iter().find(|chars| chars.len() == 2).unwrap();
            let seven = left.iter().find(|chars| chars.len() == 3).unwrap();
            let four = left.iter().find(|chars| chars.len() == 4).unwrap();
            let eight = left.iter().find(|chars| chars.len() == 7).unwrap();

            let nine = four.union(&seven).cloned().collect::<HashSet<_>>();
            let nine = left
                .iter()
                .find(|&set| set.len() == 6 && set.is_superset(&nine))
                .unwrap();

            let six = nine.difference(&one).cloned().collect::<HashSet<_>>();
            let six = left
                .iter()
                .find(|&set| set.len() == 6 && set != nine && set.is_superset(&six))
                .unwrap();

            let right_top = one.difference(&six).next().unwrap();
            let right_bottom = one.iter().find(|&bit| bit != right_top).unwrap();

            let zero = left
                .iter()
                .find(|&set| set.len() == 6 && set != six && set != nine)
                .unwrap();

            let three = left
                .iter()
                .find(|&set| set.len() == 5 && set.is_superset(&one))
                .unwrap();

            let five = left
                .iter()
                .find(|&set| set.len() == 5 && set.contains(right_bottom) && set != three)
                .unwrap();

            let right_top = one.difference(&five).next().unwrap();

            let two = left
                .iter()
                .find(|&set| set.len() == 5 && set != three && set.contains(right_top))
                .unwrap();

            let numbers = vec![zero, one, two, three, four, five, six, seven, eight, nine];

            right
                .iter()
                .map(|number| {
                    numbers
                        .iter()
                        .enumerate()
                        .find(|(_, &val)| val == number)
                        .unwrap()
                        .0
                })
                .fold(0, |acc, n| (acc * 10) + n)
        })
        .sum()
}
