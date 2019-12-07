const INPUT : &'static str = include_str!("../inputs/day7.txt");

use crate::day5::Program;
use crate::day5::InvalidInstruction;

use itertools::Itertools;

struct Amp {
    program: Program
}

impl Amp {
    pub fn new(program: Program) -> Amp {
        Amp { program: program }
    }

    pub fn execute(&self, phase: isize, input: isize) -> Result<Vec<isize>, InvalidInstruction> {
        let mut p = self.program.clone();

        p.execute(&vec![phase, input])
    }

    pub fn new_array(program: &Program, count: usize) -> Vec<Amp> {
        (0..count)
            .map(|i| Amp::new(program.clone()) )
            .collect()
    }

    pub fn exec_array(amps: &Vec<Amp>, phases: &Vec<isize>) -> Result<isize, InvalidInstruction> {
        let mut input = 0;
        for i in 0..amps.len() {
            let output = amps[i].execute(phases[i], input)?;
            input = output[0];
        }

        Ok(input)
    }

    pub fn best_output(amps: &Vec<Amp>) -> isize {
        (0..5)
            .permutations(5)
            .map(|phases| Amp::exec_array(amps, &phases).unwrap())
            .max()
            .unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_examples() {
        let p = Program::parse("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")
            .expect("Failed to parse program");
        let amps = Amp::new_array(&p, 5);
        assert_eq!(Amp::best_output(&amps), 43210);

        let p = Program::parse("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")
            .expect("Failed to parse program");

        let amps = Amp::new_array(&p, 5);

        assert_eq!(Amp::exec_array(&amps, &vec![0, 1, 2, 3, 4]), Ok(54321));

    }

    #[test]
    fn p1_solution() {
        let p = Program::parse(INPUT).unwrap();
        let amps = Amp::new_array(&p, 5);

        assert_eq!(Amp::best_output(&amps), 0);
    }
}
