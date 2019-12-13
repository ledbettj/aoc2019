const INPUT : &'static str = include_str!("../inputs/day11.txt");

use std::collections::HashMap;
use crate::day5::{Program, ProgramState, InvalidInstruction};

type Point = (isize, isize);

const WHITE : isize = 1;
const BLACK : isize = 0;

enum RoboState {
    Paint,
    Turn
}

struct Robot {
    grid: HashMap<Point, isize>,
    pos:  Point,
    facing: Point,
    state: RoboState
}

impl Robot {
    pub fn new() -> Robot {
        Robot {
            grid: HashMap::new(),
            pos: (0, 0),
            facing: (0, -1),
            state: RoboState::Paint
        }
    }

    pub fn print(&self) {
        let min_x = self.grid.keys().min_by_key(|p| p.0).unwrap().0;
        let max_x = self.grid.keys().max_by_key(|p| p.0).unwrap().0;
        let min_y = self.grid.keys().min_by_key(|p| p.1).unwrap().1;
        let max_y = self.grid.keys().max_by_key(|p| p.1).unwrap().1;

        for y in min_y..(max_y + 1) {
            for x in min_x..(max_x + 1) {
                let v = *self.grid.get(&(x, y)).unwrap_or(&BLACK);

                match v {
                    WHITE => {
                        print!("#");
                    },
                    BLACK => {
                        print!(" ");
                    },
                    _ => unreachable!()
                };
            }
            println!("");
        }
    }

    pub fn paint(&mut self, pos: Point, color: isize) {
        self.grid.insert(pos, color);
    }

    pub fn painted_count(&self) -> usize {
        self.grid.len()
    }

    pub fn turn(&mut self, dir: isize) {
        // 0 means left 90 degrees
        // 1 means right 90 degrees

        // (0, -1) turn left -> (-1, 0)
        // (-1, 0) turn left -> (0, 1)
        // (0, 1)  turn left -> (1, 0)
        // (1, 0)  turn left -> (0, -1)
        match dir {
            0 => {
                self.facing = (self.facing.1, -self.facing.0);
            },
            1 => {
                self.facing = (-self.facing.1, self.facing.0);
            },
            _ => panic!("Invalid turn instruction")
        };
    }

    pub fn forward(&mut self) {
        self.pos = (self.pos.0 + self.facing.0,
                    self.pos.1 + self.facing.1);
    }

    pub fn execute(&mut self, mut p: Program) -> Result<(), InvalidInstruction> {
        let mut next_input = None;

        loop {
            match p.step(next_input)? {
                ProgramState::Halted => {
                    return Ok(())
                }
                ProgramState::Blocked => {
                    // needs input
                    let color = *self.grid.get(&self.pos).unwrap_or(&BLACK);
                    next_input = Some(color);
                },
                ProgramState::Running(None) => {
                    next_input = None;
                },
                ProgramState::Running(Some(v)) => {
                    next_input = None;
                    match self.state {
                        RoboState::Paint => {
                            self.paint(self.pos, v);
                            self.state = RoboState::Turn;
                        },
                        RoboState::Turn => {
                            self.turn(v);
                            self.forward();
                            self.state = RoboState::Paint;
                        }
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_solution() {
        let mut r = Robot::new();
        let p = Program::parse(INPUT).expect("Invalid program");
        r.execute(p).expect("Failed to execute program");

        assert_eq!(r.painted_count(), 1907);
    }

    #[test]
    fn p2_solution() {
        let mut r = Robot::new();
        let p = Program::parse(INPUT).expect("Invalid program");

        r.paint((0, 0), WHITE);
        r.execute(p).expect("Failed to execute program");

        r.print();
    }

}
