const INPUT : &'static str = include_str!("../inputs/day9.txt");

use crate::day5::Program;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_example1() {
        let mut p = Program::parse("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99").unwrap();
        let rc = p.execute(&vec![]).unwrap();

        assert_eq!(rc, vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
    }

    #[test]
    fn p1_example2() {
        let mut p = Program::parse("104,1125899906842624,99").unwrap();
        let rc = p.execute(&vec![]).unwrap();
        assert_eq!(rc[0], 1125899906842624);
    }

    #[test]
    fn p1_solution() {
        let mut p = Program::parse(INPUT).expect("Failed to parse program");

        let output = p.execute(&vec![1]).expect("Failed to execute program");

        assert_eq!(output[0], 3507134798);
    }
}
