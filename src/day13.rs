const INPUT : &'static str = include_str!("../inputs/day13.txt");

use std::collections::HashMap;
//use std::io;
use crate::intcode::{Program, Computer, InvalidInstruction, IOEvent};

const EMPTY : isize = 0;
const WALL  : isize = 1;
const BLOCK : isize = 2;
const PADDLE: isize = 3;
const BALL  : isize = 4;

type Point = (isize, isize);

pub struct Arcade {
    grid: HashMap<Point, isize>,
    score: isize,
}

pub enum LoadState {
    XPos,
    YPos,
    Type
}

impl Arcade {
    pub fn draw(grid: &HashMap<Point, isize>) {
        let min_x = grid.keys().min_by_key(|p| p.0).unwrap().0;
        let max_x = grid.keys().max_by_key(|p| p.0).unwrap().0;
        let min_y = grid.keys().min_by_key(|p| p.1).unwrap().1;
        let max_y = grid.keys().max_by_key(|p| p.1).unwrap().1;

        for y in min_y..(max_y + 1) {
            for x in min_x..(max_x + 1) {
                let v = *grid.get(&(x, y)).unwrap_or(&EMPTY);

                match v {
                    EMPTY  => print!(" "),
                    WALL   => print!("#"),
                    BLOCK  => print!("."),
                    PADDLE => print!("-"),
                    BALL   => print!("o"),
                    _ => unreachable!()
                };
            }
            println!("");
        }
        println!("");
    }

    pub fn initialize(p: &mut Program) -> Result<Arcade, InvalidInstruction> {
        let mut grid = HashMap::new();
        let mut state = LoadState::XPos;
        let mut x = 0;
        let mut y = 0;
        let mut t = 0;
        let mut score = 0;
        let mut ball = (0, 0);
        let mut paddle = (0, 0);

        Computer::run( p, |event, _| {
            match event {
                IOEvent::Input => {
                    //  let mut s = String::new();
                    // std::io::stdin().read_line(&mut s).unwrap();
                    // match s.trim() {
                    //     "l" => Some(-1),
                    //     "r" => Some(1),
                    //     _   => Some(0)
                    // }
                    if ball.0 < paddle.0 {
                        Some(-1)
                    } else if ball.0 > paddle.0 {
                        Some(1)
                    } else {
                        Some(0)
                    }
                },
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
                            if x == -1 && y == 0 {
                                score = t;
                            } else {
                                grid.insert((x, y), t);

                                if t == PADDLE {
                                    paddle = (x, y);
                                } else if t == BALL {
                                    ball = (x, y);
                                }
                            }
                            state = LoadState::XPos;
                            Arcade::draw(&grid);
                        }

                    }
                    None
                }
            }

        })?;

        Ok(Arcade{
            grid: grid,
            score: score
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

        assert_eq!(c, 255);
    }

    #[test]
    fn p2_solution() {
        let mut p = Program::parse(INPUT).expect("Failed to load input");
        p.set_mem(0, 2);

        let a = Arcade::initialize(&mut p).unwrap();

        assert_eq!(a.score, 12338);

    }

}
