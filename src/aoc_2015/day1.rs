use crate::solver::{AoCDay, Solution, Solver};

struct Santa {
    current_floor: i64,
    steps_to_basement: u64,
}

enum Direction {
    MoveUp,
    MoveDown,
}

enum Errs {
    ParseError,
}

fn apply_dir(val: &i64, dir: &Direction) -> i64 {
    match dir {
        Direction::MoveUp => val + 1,
        Direction::MoveDown => val - 1,
    }
}

fn parse(s: &str) -> Result<Vec<Direction>, Errs> {
    s.chars().collect().map(|c| match c {
        '(' => Ok(Direction::MoveUp),
        ')' => Ok(Direction::MoveDown),
        _ => Err(Errs::ParseError),
    })
}

fn follow_directions(santa: Santa, directions: &Vec<Direction>) -> Santa {
    Santa {
        current_floor: directions.iter().fold(0, apply_dir),
        steps_to_basement: santa.steps_to_basement,
    }
}

fn locate_basement(santa: Santa, directions: &Vec<Direction>) -> Santa {
    Santa {
        current_floor: santa.current_floor,
        steps_to_basement: directions
            .iter()
            .scan(0i64, |acc, dir| match acc {
                -1 => None,
                acc => Some(apply_dir(acc, dir)),
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
