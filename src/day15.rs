const INPUT : &'static str = include_str!("../inputs/day15.txt");


use crate::intcode::{Program,Computer,InvalidInstruction,IOEvent};
use std::collections::{HashMap,HashSet};
use rand::prelude::*;

type Point = (isize, isize);

const WALL    : char = '#';
const OPEN    : char = ' ';
const OXYGEN  : char = 'o';
const UNKNOWN : char = '?';


struct Area {
    map: HashMap<Point, char>,
    droid: Point
}

impl Area {
    pub fn new() -> Area {

        let mut map = HashMap::new();
        map.insert((0, 0), OPEN);
        Area {
            map: map,
            droid: (0, 0)
        }
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

    pub fn find_oxygen(&mut self, mut p: &mut Program) -> Result<Point,InvalidInstruction> {
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
                            self.print();
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

        Ok(self.droid)
    }

    pub fn min_distance(&self, from: Point, to: Point, mut used: &mut HashSet<Point>) -> Option<usize> {
        if from == to {
            println!("found dest at {:?}", from);
            return Some(0);
        }

        let neighbors = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
        let neighbors = neighbors
            .iter()
            .filter_map(|&(px, py)|{
                let point = (from.0 + px, from.1 + py);
                let value = self.map.get(&point).unwrap_or(&UNKNOWN);
                let ok = (value == &OPEN || value == &OXYGEN) && used.insert(point);
                if ok {
                    Some(point)
                } else {
                    None
                }
            })
            .inspect(|p| println!("exploring {:?}", p))
            .collect::<Vec<Point>>();

        neighbors
            .iter()
            .filter_map(|&point|{
                match self.min_distance(point, to, &mut used) {
                    Some(v) => Some(v + 1),
                    None => None
                }
            })
            .min()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_solution() {
        let mut p = Program::parse(INPUT).expect("Failed to parse program");
        let mut a = Area::new();

        let oxygen = a.find_oxygen(&mut p).unwrap();

        let distance = a.min_distance((0, 0), oxygen, &mut HashSet::new());

        assert_eq!(distance, Some(244));
    }
}
