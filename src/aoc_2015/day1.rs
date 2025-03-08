use crate::solver::{AoCDay, Solution, Puzzle};

struct Santa {
    current_floor: i64,
    steps_to_basement: u64,
}

enum Move {
    Up,
    Down,
}

enum Errs {
    ParseError,
}

fn apply_move(val: &i64, dir: &Move) -> i64 {
    match dir {
        Move::Up => val + 1,
        Move::Down => val - 1,
    }
}

fn parse(s: &str) -> Result<&Vec<Move>, Errs> {
    s.chars().collect().map(|c| match c {
        '(' => Ok(Move::Up),
        ')' => Ok(Move::Down),
        _ => Err(Errs::ParseError),
    })
}

pub fn aoc_day_1 (s: &str) -> AoCDay<&Vec<Move>, i64, u64, Errs> {
    AoCDay {
        parser: parse,
        puzzle_part1:
    }
}

fn follow_directions(santa: Santa, directions: &Vec<Move>) -> Santa {
    Santa {
        current_floor: directions.iter().fold(0, apply_move),
        steps_to_basement: santa.steps_to_basement,
    }
}

fn locate_basement(santa: Santa, directions: &Vec<Move>) -> Santa {
    Santa {
        current_floor: santa.current_floor,
        steps_to_basement: directions
            .iter()
            .scan(0i64, |acc, dir| match acc {
                -1 => None,
                acc => Some(apply_move(acc, dir)),
            })
            .count() as u64,
    }
}

struct AoCDay1<'a, 'b, T, U> {
    answers: AoCDay<'a, 'b, T, U>,
}

impl Solver<i64, u64, Errs> for AoCDay1<i64, u64> {
    fn solve(&self, dataset: &str) -> Result<(), Errs> {
        parse(dataset).map(|data| {
            let santa = Santa {
                current_floor: 0,
                steps_to_basement: 0,
            };

            let santa = follow_directions(santa, &data);
            let santa = locate_basement(santa, &data);

            self.answers = AoCDay {
                answer_part_1: Solution {
                    problem: "Final Floor: ",
                    answer: &santa.current_floor,
                },
                answer_part_2: Some(Solution {
                    problem: "Basement Step: ",
                    answer: &santa.steps_to_basement,
                }),
            }
        })
    }
}
