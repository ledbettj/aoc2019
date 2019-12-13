const INPUT : &'static str = include_str!("../inputs/day13.txt");

use std::collections::HashMap;
use crate::intcode::{Program, Computer, InvalidInstruction, IOEvent};

const EMPTY : isize = 0;
const WALL  : isize = 1;
const BLOCK : isize = 2;
const PADDLE: isize = 3;
const BALL  : isize = 4;

type Point = (isize, isize);

pub struct Arcade {
    grid: HashMap<Point, isize>
}

pub enum LoadState {
    XPos,
    YPos,
    Type
}

impl Arcade {
    pub fn initialize(p: &mut Program) -> Result<Arcade, InvalidInstruction> {
        let mut grid = HashMap::new();
        let mut state = LoadState::XPos;
        let mut x = 0;
        let mut y = 0;
        let mut t = 0;

        Computer::run( p, |event| {
            match event {
                IOEvent::Input => None,
                IOEvent::Output(v) => {
                    match state {
                        LoadState::XPos => {
                            x = v;
                            state = LoadState::YPos;
                        },
                        LoadState::YPos => {
                            y = v;
                            state = LoadState::Type;
                        },
                        LoadState::Type => {
                            t = v;
                            grid.insert((x, y), t);
                            state = LoadState::XPos;
                        }

                    }
                    None
                }
            }

        })?;

        Ok(Arcade{
            grid: grid
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_solution() {
        let mut p = Program::parse(INPUT).expect("Failed to load input");

        let a = Arcade::initialize(&mut p).unwrap();
        let c = a.grid.values()
            .filter(|&&v| v == BLOCK)
            .count();

        assert_eq!(c, 0);
    }
    

}
