use crate::solver::{*};
use std::fmt::Debug;

struct Santa {
    current_floor: i64,
    steps_to_basement: u64,
}

pub enum Move {
    Up,
    Down,
}

#[derive(Debug)]
pub enum Errs {
    ParseError,
}

fn apply_move(floor: i64, dir: &Move) -> i64 {
    match dir {
        Move::Up => floor + 1,
        Move::Down => floor - 1,
    }
}

fn parse(s: &str) -> Result<Vec<Move>, Errs> {
    s.chars().map(
        |c| match c {
        '(' => Ok(Move::Up),
        ')' => Ok(Move::Down),
        _ => Err(Errs::ParseError),
    }).collect()
}

pub fn aoc_day_1 (s: &str) -> AoCDay<Vec<Move>, i64, u64, Errs> {
    AoCDay {
        parser: parse,
        puzzle_part1: Puzzle {
            solver: (|movements| Solution { problem: "Final Floor".to_string(), answer: Some(movements.iter().fold(0, apply_move)) })
        },
        puzzle_part2: Puzzle {
            solver: (|movements| Solution {
                problem: "Basement Position".to_string(),
                answer: Some (
                    movements
                        .iter()
                        .scan(
                            0, 
                            |acc, dir| match acc {
                                -1 => None,
                                acc => Some(apply_move(*acc, dir))
                            })
                        .count()
                        as u64
                )
            })
        },
    }
}
