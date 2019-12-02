use std::num::ParseIntError;

const INPUT : &'static str  = include_str!("../inputs/day2.txt");

#[derive(Debug,PartialEq,Clone)]
struct IntcodeComputer {
    pub ops: Vec<usize>
}

impl IntcodeComputer {
    /// Create a new computer from a list of instructions.
    pub fn new(ops: Vec<usize>) -> IntcodeComputer {
        IntcodeComputer { ops: ops }
    }

    /// Create a new computer, if possible, from an input string of comma delimited
    /// integers.
    pub fn parse(line: &str) -> Result<IntcodeComputer, ParseIntError> {
        let ops = line
            .trim()
            .split(",")
            .map(|part| part.trim().parse::<usize>())
            .collect::<Result<Vec<usize>, ParseIntError>>()?;

        Ok(IntcodeComputer::new(ops))
    }

    pub fn noun(&mut self, noun: usize) -> &mut IntcodeComputer {
        self.ops[1] = noun;
        self
    }

    pub fn verb(&mut self, verb: usize) -> &mut IntcodeComputer {
        self.ops[2] = verb;
        self
    }

    pub fn output(&self) -> usize {
        self.ops[0]
    }

    /// Evaluate the instructions contained in the computer.
    pub fn eval(&mut self) -> &IntcodeComputer {
        let range = (0..self.ops.len()).step_by(4);

        for i in range {
            match self.ops[i] {
                1 => {
                    let i1 = self.ops[i + 1];
                    let i2 = self.ops[i + 2];
                    let target = self.ops[i + 3];
                    self.ops[target] = self.ops[i1] + self.ops[i2];
                },
                2 => {
                    let i1 = self.ops[i + 1];
                    let i2 = self.ops[i + 2];
                    let target = self.ops[i + 3];
                    self.ops[target] = self.ops[i1] * self.ops[i2];
                },
                _ => { break; }
            }
        };
        self
    }

    /// Given a target, determine the inputs that must go at positions
    /// 1 and 2 in the instruction list to cause target to be stored at
    /// position 0.
    /// If no combination is found, returns None.
    pub fn solve(&self, target: usize) -> Option<(usize, usize)> {

        for i in 0..99 {
            for j in 0..99 {
                let mut working = self.clone();

                let output = working
                    .noun(i)
                    .verb(j)
                    .eval()
                    .output();

                if output == target {
                    return Some((i, j))
                }
            }

        };
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        let i = IntcodeComputer::parse("1,9,10,3,2,3,11,0,99,30,40,50");
        assert_eq!(i.is_ok(), true);
        assert_eq!(i.unwrap().ops, vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn eval_works() {
        let mut i = IntcodeComputer::parse("1,9,10,3,2,3,11,0,99,30,40,50").expect("Parse failed");
        i.eval();

        assert_eq!(i.ops, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);

        i = IntcodeComputer::parse("1,0,0,0,99").unwrap();
        assert_eq!(i.eval().ops, vec![2, 0, 0, 0, 99]);

        i = IntcodeComputer::parse("2,3,0,3,99").unwrap();
        assert_eq!(i.eval().ops, vec![2, 3, 0, 6, 99]);

        i = IntcodeComputer::parse("2,4,4,5,99,0").unwrap();
        assert_eq!(i.eval().ops, vec![2, 4, 4, 5, 99, 9801]);

        i = IntcodeComputer::parse("1,1,1,4,99,5,6,0,99").unwrap();
        assert_eq!(i.eval().ops, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn part1_solution() {
        let mut i = IntcodeComputer::parse(INPUT).expect("Failed to parse input");
        let output = i.noun(12).verb(2).eval().output();

        assert_eq!(output, 7210630);
    }

    #[test]
    fn part2_solution() {
        let cpu = IntcodeComputer::parse(INPUT).expect("Failed to parse input");

        let result = cpu.solve(19690720);
        assert_eq!(result.is_some(), true);

        let (noun, verb) = result.unwrap();
        assert_eq!(noun * 100 + verb, 3892);
    }

}
