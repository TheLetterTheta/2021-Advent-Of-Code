use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Vec<u32>>;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[aoc(day9, part1)]
fn solve_part1(heightmap: &Input) -> u32 {
    heightmap
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(column_index, _)| {
                    let curr = heightmap[row_index][column_index];

                    // Check if this location is lower than any of its adjacent locations
                    if !((row_index > 0 && heightmap[row_index - 1][column_index] < curr)
                        || (column_index > 0 && heightmap[row_index][column_index - 1] < curr)
                        || (row_index < heightmap.len() - 1
                            && heightmap[row_index + 1][column_index] < curr)
                        || (column_index < row.len() - 1
                            && heightmap[row_index][column_index + 1] < curr))
                    {
                        // Calculate the risk level of this low point (1 plus its height) and add it to the sum
                        Some(heightmap[row_index][column_index] + 1)
                    } else {
                        None
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

#[aoc(day9, part2)]
fn solve_part2(input: &Input) -> usize {
    todo!();
}
