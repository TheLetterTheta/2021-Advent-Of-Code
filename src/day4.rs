use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cell {
    Unmarked(u8),
    Marked(u8),
}

#[derive(Debug, Clone)]
struct Board(Vec<Vec<Cell>>);

impl Board {
    fn new(board: Vec<Vec<Cell>>) -> Self {
        Board(board)
    }

    fn mark(&mut self, called: u8) {
        self.0.iter_mut().for_each(|t| {
            t.iter_mut().for_each(|cell| {
                if *cell == Cell::Unmarked(called) {
                    *cell = Cell::Marked(called);
                }
            })
        })
    }

    fn cells(&self) -> Vec<&Cell> {
        self.0.iter().flat_map(|i| i.iter()).collect()
    }

    fn is_bingo(&self) -> bool {
        if self
            .0
            .iter()
            .any(|column| column.iter().all(|c| matches!(c, Cell::Marked(_))))
        {
            return true;
        }

        'outer: for j in 0..5 {
            for i in 0..5 {
                if !matches!(self.0[i][j], Cell::Marked(_)) {
                    continue 'outer;
                }
            }

            return true;
        }

        false
    }
}

#[derive(Debug)]
struct Bingo {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

impl Bingo {
    fn new(numbers: Vec<u8>, boards: Vec<Board>) -> Self {
        Bingo { numbers, boards }
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Bingo {
    let mut lines = input.lines();
    let numbers: Vec<u8> = lines
        .next()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .expect("Invalid Input");

    let mut boards = Vec::new();

    while let Some(_) = lines.next() {
        // input has empty line
        let mut board = Vec::with_capacity(5);
        for _ in 0..5 {
            let nums: Vec<Cell> = lines
                .next()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| Cell::Unmarked(num.parse().unwrap()))
                        .collect()
                })
                .expect("Invalid Input");
            board.push(nums);
        }
        boards.push(Board::new(board));
    }

    Bingo::new(numbers, boards)
}

#[aoc(day4, part1)]
fn first_bingo(input: &Bingo) -> u16 {
    let mut boards = input.boards.clone();
    for number in &input.numbers {
        boards.iter_mut().for_each(|board| board.mark(*number));

        if let Some(board) = boards.iter().find(|board| board.is_bingo()) {
            return (*number as u16)
                * board
                    .cells()
                    .iter()
                    .filter_map(|c| match c {
                        Cell::Unmarked(n) => Some(*n as u16),
                        _ => None,
                    })
                    .sum::<u16>();
        }
    }
    unreachable!();
}

#[aoc(day4, part2)]
fn last_bingo(input: &Bingo) -> u16 {
    let mut boards = input.boards.clone();
    for number in &input.numbers {
        boards.iter_mut().for_each(|board| board.mark(*number));

        if boards.len() > 1 {
            while let Some((index, _)) = boards
                .iter()
                .enumerate()
                .find(|(_, board)| board.is_bingo())
            {
                boards.remove(index);
            }
        } else if boards[0].is_bingo() {
            return (*number as u16)
                * boards[0]
                    .cells()
                    .iter()
                    .filter_map(|c| match c {
                        Cell::Unmarked(n) => Some(*n as u16),
                        _ => None,
                    })
                    .sum::<u16>();
        }
    }
    unreachable!();
}
