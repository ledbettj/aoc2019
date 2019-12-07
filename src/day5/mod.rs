mod instruction;
mod program;

pub use program::Program;
pub use instruction::InvalidInstruction;

const INPUT : &'static str  = include_str!("../../inputs/day5.txt");


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_example() {
        let mut p = Program::parse("1002,4,3,4,33").expect("Failed to parse input");
        p.execute(&vec![0]).expect("Failed to execute program");
    }

    #[test]
    fn p1_solution() {
        let mut p = Program::parse(INPUT).expect("Failed to parse input");
        let output = p.execute(&vec![1]).expect("Failed to execute program");
        assert_eq!(output.last(), Some(&5074395_isize));
    }

    #[test]
    fn p2_solution() {
        let mut p = Program::parse(INPUT).expect("Failed to parse input");
        let output = p.execute(&vec![5]).expect("Failed to execute program");
        assert_eq!(output.last(), Some(&8346937_isize));
    }

}
