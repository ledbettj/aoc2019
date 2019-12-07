const INPUT : &'static str = include_str!("../inputs/day7.txt");

use crate::day5::{Program,ProgramState};
use crate::day5::InvalidInstruction;

use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug,Clone)]
struct Amp {
    program: Program,
    program_state: ProgramState
}

impl Amp {
    pub fn new(program: Program) -> Amp {
        Amp { program: program, program_state: ProgramState::Running(None) }
    }

    pub fn execute(&self, phase: isize, input: isize) -> Result<Vec<isize>, InvalidInstruction> {
        let mut p = self.program.clone();

        p.execute(&vec![phase, input])
    }

    pub fn step_mut(&mut self, input: Option<isize>) -> Result<ProgramState, InvalidInstruction> {
        let rc = self.program.step(input);
        if let Ok(state) = rc {
            self.program_state = state;
        }
        rc
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

    pub fn best_output_feedback(amps: &Vec<Amp>) -> isize {
        (5..10)
            .permutations(5)
            .map(|phases|{
                let mut temp_amps = amps.clone();
                Amp::exec_array_feedback(&mut temp_amps, &phases).unwrap()
            })
            .max()
            .unwrap()
    }
    
    pub fn exec_array_feedback(amps: &mut Vec<Amp>, phases: &Vec<isize>) -> Result<isize, InvalidInstruction> {
        let mut outputs : Vec<VecDeque<isize>> = (0..amps.len())
            .map(|i|{
                let mut q = VecDeque::new();
                let next = (i + 1) % amps.len();
                q.push_back(phases[next]);
                q
            })
            .collect();

        // input to Amp 1 is phase, then 0.
        outputs[amps.len() - 1].push_back(0);
        let mut done;

        loop {
            done = true;
            for i in 0..amps.len() {
                let prev = if i == 0 { amps.len() - 1 } else { i - 1 };

                match amps[i].program_state {
                    ProgramState::Blocked => {
                        if let ProgramState::Running(Some(v)) = amps[i].step_mut(outputs[prev].pop_front())? {
                            outputs[i].push_back(v);
                        }
                        done = false;
                    },
                    ProgramState::Running(_) => {
                        if let ProgramState::Running(Some(v)) = amps[i].step_mut(None)? {
                            outputs[i].push_back(v);
                        }
                        done = false;
                    }
                    ProgramState::Halted => {
                    }
                };

            }

            if done { break }
        }

        let result = *outputs.last().map(|o| o.back().unwrap()).unwrap();
        Ok(result)
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

        assert_eq!(Amp::best_output(&amps), 70597);
    }

    #[test]
    fn p2_example() {
        let p = Program::parse("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5")
            .expect("Failed to parse program");

        let mut amps = Amp::new_array(&p, 5);

        assert_eq!(
            Amp::exec_array_feedback(&mut amps, &vec![9, 8, 7, 6, 5]),
            Ok(139629729))

    }


    #[test]
    fn p2_solution() {
        let p = Program::parse(INPUT)
            .expect("Failed to parse program");

        let mut amps = Amp::new_array(&p, 5);

        assert_eq!(Amp::best_output_feedback(&mut amps), 30872528);

    }

}
