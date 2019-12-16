const INPUT : &'static str = include_str!("../inputs/day15.txt");


use crate::intcode::{Program,Computer,InvalidInstruction,IOEvent};
use std::collections::HashMap;
use rand::prelude::*;

type Point = (isize, isize);

const WALL    : char = '#';
const OPEN    : char = ' ';
const OXYGEN  : char = 'o';
const UNKNOWN : char = '?';


struct Area {
    map: HashMap<Point, char>,
    droid: Point,
    dir:   Point,
    translate: HashMap<Point, isize>
}

impl Area {
    pub fn new() -> Area {
        let mut translate = HashMap::new();
        translate.insert((0, -1), 1);
        translate.insert((1, 0), 4);
        translate.insert((0, 1), 2);
        translate.insert((-1, 0), 3);

        let mut map = HashMap::new();
        map.insert((0, 0), OPEN);
        Area {
            map: map,
            droid: (0, 0),
            dir: (0, -1),
            translate: translate
        }
    }

    pub fn turn(&mut self) {
        self.dir = (-self.dir.1, self.dir.0);
    }

    pub fn left_of_current(&self) -> Point {
        let (dx, dy) = self.droid;
        let (ox, oy) = (self.dir.0, self.dir.1);
        (dx + ox, dy + oy)
    }

    pub fn print(&self) {
        let min_x = self.map.keys().min_by_key(|p| p.0).unwrap().0;
        let max_x = self.map.keys().max_by_key(|p| p.0).unwrap().0;
        let min_y = self.map.keys().min_by_key(|p| p.1).unwrap().1;
        let max_y = self.map.keys().max_by_key(|p| p.1).unwrap().1;

        for y in min_y..(max_y + 1) {
            for x in min_x..(max_x + 1) {
                let v = *self.map.get(&(x, y)).unwrap_or(&UNKNOWN);
                if (x, y) == self.droid {
                    print!("x");
                } else {
                    print!("{}", v);
                }
            }
            println!("");
        }
        println!("");
    }

    pub fn find_oxygen(&mut self, mut p: &mut Program) -> Result<(),InvalidInstruction> {
        let mut next = (0, 0);
        let mut rng = rand::thread_rng();
        Computer::run(&mut p, |event, computer| {
            match event {
                IOEvent::Output(rc) => {
                    match rc {
                        0 => {
                            self.map.insert(next, WALL);
                            next = self.droid;
                        }
                        1 => {
                            self.map.insert(next, OPEN);
                            self.droid = next;
                        },
                        2 => {
                            self.map.insert(next, OXYGEN);
                            self.droid = next;
                            computer.abort();
                            println!("found oxygen at {:?}", next);
                        },
                        _ => unreachable!()
                    };

                    None
                },
                IOEvent::Input => {
                    let around = vec![
                        ((self.droid.0, self.droid.1 - 1), 1),
                        ((self.droid.0, self.droid.1 + 1), 2),
                        ((self.droid.0 - 1, self.droid.1), 3),
                        ((self.droid.0 + 1, self.droid.1), 4)
                    ];

                    let mut possible = around
                        .iter()
                        .filter(|(p, _)| self.map.get(&p).unwrap_or(&UNKNOWN) == &UNKNOWN)
                        .collect::<Vec<&(Point, isize)>>();

                    if possible.is_empty() {
                        possible = around
                            .iter()
                            .filter(|(p, _)| self.map.get(&p).unwrap_or(&UNKNOWN) == &OPEN)
                            .collect::<Vec<&(Point, isize)>>();
                    }

                    let (n, instr) = possible[rng.next_u32() as usize % possible.len()];
                    next = *n;
                    Some(*instr)
                }
            }
        })?;

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_solution() {
        let mut p = Program::parse(INPUT).expect("Failed to parse program");
        let mut a = Area::new();

        a.find_oxygen(&mut p);
    }
}
