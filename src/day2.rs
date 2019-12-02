use std::num::ParseIntError;

const INPUT : &'static str  = include_str!("../inputs/day2.txt");

#[derive(Debug,PartialEq)]
struct IntcodeComputer {
    pub ops: Vec<usize>
}

impl IntcodeComputer {
    pub fn new(ops: Vec<usize>) -> IntcodeComputer {
        IntcodeComputer { ops: ops }
    }

    pub fn parse(line: &str) -> Result<IntcodeComputer, ParseIntError> {
        let ops = line
            .trim()
            .split(",")
            .map(|part| part.trim().parse::<usize>())
            .collect::<Result<Vec<usize>, ParseIntError>>()?;

        Ok(IntcodeComputer::new(ops))
    }

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
        i.ops[1] = 12;
        i.ops[2] = 2;

        i.eval();

        assert_eq!(i.ops[0], 7210630);
    }

}
