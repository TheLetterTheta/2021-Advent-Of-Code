use aoc_runner_derive::{aoc, aoc_generator};

enum Direction {
    Forward(usize),
    Up(usize),
    Down(usize),
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|line| {
            let mut line = line.split(' ');
            let units = line.next_back().unwrap().parse::<usize>().unwrap();
            match line.next().unwrap() {
                "forward" => Direction::Forward(units),
                "up" => Direction::Up(units),
                "down" => Direction::Down(units),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn multiply_directions(input: &[Direction]) -> usize {
    let (horizontal, vertical) = input.iter().fold((0, 0), |pos, dir| match dir {
        Direction::Forward(units) => (pos.0 + units, pos.1),
        Direction::Up(units) => (pos.0, pos.1 - units),
        Direction::Down(units) => (pos.0, pos.1 + units),
    });

    horizontal * vertical
}

#[aoc(day2, part2)]
fn depth_aim_vert(input: &[Direction]) -> usize {
    let (horizontal, vertical, _) = input.iter().fold((0, 0, 0), |pos, dir| match dir {
        Direction::Forward(units) => (pos.0 + units, pos.1 + (units * pos.2), pos.2),
        Direction::Up(units) => (pos.0, pos.1, pos.2 - units),
        Direction::Down(units) => (pos.0, pos.1, pos.2 + units),
    });

    horizontal * vertical
}
