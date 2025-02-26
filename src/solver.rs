use std::fmt::Display;

pub struct Solution<'a, T: Display> {
    pub problem: &'a str,
    pub answer: &'a T
}

impl Display for Solution<_> {
    fn fmt(self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.problem, self.answer)
    }
}

pub struct Puzzle<TProblemInput, TProblemOutput> {
    pub solver: fn(&TProblemInput) -> Solution<TProblemOutput>
}

pub struct AoCDay<'a, 'b, TPuzzleIn, TPuzzle1Out, TPuzzle2Out, TPuzzleErr> {
    pub parser: fn(&str) -> Result<TPuzzleIn, TPuzzleErr>,
    pub puzzle_part1: Puzzle<&'a TPuzzleIn, TPuzzle1Out>,
    pub puzzle_part2: Option<Puzzle<&'b TPuzzleIn, TPuzzle2Out>>
}

impl AoCDay<_, _, _, _> {
    fn solve(self, puzzle_input: &str) {
        let parse = self.parser;
        
        parse(puzzle_input)
            .map(|input| {
                let result = String::new();
                let day1 = self.puzzle_part1.solver(&input);
                let maybe_day2 = self.puzzle_part2.map(|pt2| pt2.solver(&input));
                
                write!(result, "{}\n{}", day1, maybe_day2.unwrap_or("")).unwrap();
            })
    }
}
