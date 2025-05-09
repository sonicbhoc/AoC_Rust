use std::fmt::{Debug, Display};

pub struct Solution<T: Display> {
    pub problem: String,
    pub answer: Option<T>,
}

impl<'a, T: Display> Display for Solution<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.answer {
            Some(ans) => write!(f, "{}: {}", self.problem, ans),
            None => write!(f, "{}", self.problem),
        }
    }
}

pub struct Puzzle<TProblemInput, TProblemOutput: Display> {
    pub solver: fn(&TProblemInput) -> Solution<TProblemOutput>,
}

pub struct AoCDay<TPuzzleIn, TPuzzle1Out: Display, TPuzzle2Out: Display, TPuzzleErr: Debug> {
    pub parser: fn(&str) -> Result<TPuzzleIn, TPuzzleErr>,
    pub puzzle_part1: Puzzle<TPuzzleIn, TPuzzle1Out>,
    pub puzzle_part2: Puzzle<TPuzzleIn, TPuzzle2Out>,
}

impl<TPuzzleIn, TPuzzle1Out: Display, TPuzzle2Out: Display, TPuzzleErr: Debug>
    AoCDay<TPuzzleIn, TPuzzle1Out, TPuzzle2Out, TPuzzleErr>
{
    fn solve(self, puzzle_input: &str) {
        let parse = self.parser;

        parse(puzzle_input)
            .map(|input| {
                let day1 = (self.puzzle_part1.solver)(&input);
                let day2 = (self.puzzle_part2.solver)(&input);

                println!("Solutions:\n{day1}\n{day2}")
            })
            .unwrap()
    }
}
