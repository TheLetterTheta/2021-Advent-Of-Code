use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

#[derive(Eq, PartialEq)]
enum Input {
    Parenthesis,
    Bracket,
    CurlyBracket,
    Tag,
}

enum OpenClose {
    Open(Input),
    Close(Input),
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<Vec<OpenClose>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '(' => OpenClose::Open(Input::Parenthesis),
                    '[' => OpenClose::Open(Input::Bracket),
                    '{' => OpenClose::Open(Input::CurlyBracket),
                    '<' => OpenClose::Open(Input::Tag),
                    ')' => OpenClose::Close(Input::Parenthesis),
                    ']' => OpenClose::Close(Input::Bracket),
                    '}' => OpenClose::Close(Input::CurlyBracket),
                    '>' => OpenClose::Close(Input::Tag),
                    _ => unreachable!("Invalid Input"),
                })
                .collect()
        })
        .collect()
}

#[aoc(day10, part1)]
fn overlapping(input: &[Vec<OpenClose>]) -> usize {
    input
        .iter()
        .filter_map(|line| {
            line.iter().fold_while(vec![], |mut acc, c| match c {
                OpenClose::Open(open) => {
                    acc.push(open);
                    Continue(acc)
                }
                OpenClose::Close(close) => {
                    if let Some(open) = acc.pop() {
                        if open == close {
                            Continue(acc)
                        } else {
                            Done(vec![close])
                        }
                    } else {
                        Continue(acc)
                    }
                }
            })
        })
        .count()
}
