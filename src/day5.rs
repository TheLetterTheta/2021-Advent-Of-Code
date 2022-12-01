use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_input_str(input: &str) -> Point {
        let (x, y) = input.split_once(",").expect("Invalid Input");
        let x = x.parse::<usize>().expect("Invalid Input");
        let y = y.parse::<usize>().expect("Invalid Input");

        Point { x, y }
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<(Point, Point)> {
    input
        .lines()
        .filter_map(|l| {
            l.split_once(" -> ")
                .map(|(left, right)| (Point::from_input_str(left), Point::from_input_str(right)))
        })
        .collect()
}

#[aoc(day5, part1)]
fn overlapping(input: &[(Point, Point)]) -> usize {
    let mut map: HashMap<(usize, usize), usize> = HashMap::with_capacity(input.len());

    for (from, to) in input.iter().filter(|(l, r)| l.x == r.x || l.y == r.y) {
        if from.x == to.x {
            let x = from.x;

            if from.y < to.y {
                for y in from.y..=to.y {
                    map.entry((x, y))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            } else {
                for y in to.y..=from.y {
                    map.entry((x, y))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        } else {
            // guranteed that y's ==
            let y = from.y;

            if from.x < to.x {
                for x in from.x..=to.x {
                    map.entry((x, y))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            } else {
                for x in to.x..=from.x {
                    map.entry((x, y))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        }
    }

    map.values().filter(|&&v| v > 1).count()
}

#[aoc(day5, part2)]
fn part2(input: &[(Point, Point)]) -> usize {
    let mut map: HashMap<(usize, usize), usize> = HashMap::with_capacity(input.len());

    for (from, to) in input.iter().filter(|(l, r)| l.x == r.x || l.y == r.y) {
        if from.x == to.x {
            let x = from.x;

            if from.y < to.y {
                for y in from.y..=to.y {
                    map.entry((x, y))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            } else {
                for y in to.y..=from.y {
                    map.entry((x, y))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        } else {
            // guranteed that y's ==
            let y = from.y;

            if from.x < to.x {
                for x in from.x..=to.x {
                    map.entry((x, y))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            } else {
                for x in to.x..=from.x {
                    map.entry((x, y))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        }
    }

    for (from, to) in input.iter().filter(|(l, r)| l.x != r.x && l.y != r.y) {
        // both move - and by the same amount

        let xs = if from.x < to.x {
            (from.x..=to.x).collect::<Vec<_>>()
        } else {
            (to.x..=from.x).rev().collect::<Vec<_>>()
        };

        let ys = if from.y < to.y {
            (from.y..=to.y).collect::<Vec<_>>()
        } else {
            (to.y..=from.y).rev().collect::<Vec<_>>()
        };

        for (x, y) in xs.into_iter().zip(ys) {
            map.entry((x, y))
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    map.values().filter(|&&v| v > 1).count()
}
