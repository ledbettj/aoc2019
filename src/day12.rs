const INPUT : &'static str = include_str!("../inputs/day12.txt");

use std::convert::TryFrom;
use regex::Regex;

#[derive(Debug,PartialEq,Clone,Copy,Eq,Hash)]
struct Vec3d {
    x: isize,
    y: isize,
    z: isize
}

#[derive(Debug,PartialEq,Copy,Clone,Eq,Hash)]
struct Moon {
    pos: Vec3d,
    vel: Vec3d
}

impl Moon {
    pub fn new(pos: Vec3d) -> Moon {
        Moon {
            pos: pos,
            vel: Vec3d { x: 0, y: 0, z: 0 }
        }
    }

    pub fn x_only(&self) -> Moon {
        Moon {
            pos: Vec3d { x: self.pos.x, y: 0, z: 0 },
            vel: Vec3d { x: self.vel.x, y: 0, z: 0 }
        }
    }

    pub fn y_only(&self) -> Moon {
        Moon {
            pos: Vec3d { x: 0, y: self.pos.y, z: 0 },
            vel: Vec3d { x: 0, y: self.vel.y, z: 0 }
        }
    }

    pub fn z_only(&self) -> Moon {
        Moon {
            pos: Vec3d { x: 0, y: 0, z: self.pos.z },
            vel: Vec3d { x: 0, y: 0, z: self.vel.z }
        }
    }

    pub fn apply_velocity(&self) -> Moon {
        let mut pos = self.pos;
        pos.x += self.vel.x;
        pos.y += self.vel.y;
        pos.z += self.vel.z;

        Moon {
            pos: pos,
            vel: self.vel
        }
    }

    pub fn apply_gravity(&self, other: &Moon) -> Moon {
        let mut vel = self.vel;

        if other.pos.x > self.pos.x {
            vel.x += 1;
        } else if other.pos.x < self.pos.x {
            vel.x -= 1;
        }

        if other.pos.y > self.pos.y {
            vel.y += 1;
        } else if other.pos.y < self.pos.y {
            vel.y -= 1;
        }

        if other.pos.z > self.pos.z {
            vel.z += 1;
        } else if other.pos.z < self.pos.z {
            vel.z -= 1;
        }

        Moon {
            pos: self.pos,
            vel: vel
        }

    }

    pub fn potential_energy(&self) -> usize {
        (self.pos.x.abs() +
            self.pos.y.abs() +
            self.pos.z.abs()) as usize
    }

    pub fn kinetic_energy(&self) -> usize {
        (self.vel.x.abs() +
            self.vel.y.abs() +
            self.vel.z.abs()) as usize
    }

    pub fn total_energy(&self) -> usize {
        self.potential_energy() * self.kinetic_energy()
    }

    pub fn vec_x_only(moons: &Vec<Moon>) -> Vec<Moon> {
        moons
            .iter()
            .map(|m| m.x_only())
            .collect()
    }

    pub fn vec_y_only(moons: &Vec<Moon>) -> Vec<Moon> {
        moons
            .iter()
            .map(|m| m.y_only())
            .collect()
    }

    pub fn vec_z_only(moons: &Vec<Moon>) -> Vec<Moon> {
        moons
            .iter()
            .map(|m| m.z_only())
            .collect()
    }


    pub fn step(moons: &Vec<Moon>) -> Vec<Moon> {
        let result = moons
            .iter()
            .map(|moon|{
                moons
                    .iter()
                    .filter(|&m| m != moon)
                    .fold(*moon, |acc, m| acc.apply_gravity(m))

            })
            .collect::<Vec<Moon>>();

        // apply velocity
        result
            .iter()
            .map(|moon| moon.apply_velocity())
            .collect::<Vec<Moon>>()
    }
}

#[derive(Debug,PartialEq)]
pub enum Vec3dError {
    ParseError
}

impl TryFrom<&str> for Vec3d {
    type Error = Vec3dError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE : Regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>")
                .unwrap();
        }
        match RE.captures(input) {
            None => Err(Vec3dError::ParseError),
            Some(caps) => {
                Ok(Vec3d{
                    x: caps.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                    y: caps.get(2).unwrap().as_str().parse::<isize>().unwrap(),
                    z: caps.get(3).unwrap().as_str().parse::<isize>().unwrap()
                })
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_parse() {
        let input = "<x=-1, y=0, z=2>";
        let v = Vec3d::try_from(input);

        assert_eq!(v, Ok(Vec3d{x: -1, y: 0, z: 2}));
    }

    #[test]
    fn p1_example() {
        let input = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>";
        let mut moons = input
            .lines()
            .map(|line| Moon::new(Vec3d::try_from(line).unwrap()))
            .collect::<Vec<Moon>>();

        for _ in 0..10 {
            moons = Moon::step(&moons);
        }

        let energy : usize = moons
            .iter()
            .map(|m| m.total_energy())
            .sum();

        assert_eq!(energy, 179);
    }

    #[test]
    fn p1_solution() {
        let mut moons = INPUT
            .lines()
            .map(|line| Moon::new(Vec3d::try_from(line).unwrap()))
            .collect::<Vec<Moon>>();

        for _ in 0..1000 {
            moons = Moon::step(&moons);
        }

        let energy : usize = moons
            .iter()
            .map(|m| m.total_energy())
            .sum();

        assert_eq!(energy, 7758);

    }

    #[test]
    fn p2_solution() {
        let mut moons = INPUT
            .lines()
            .map(|line| Moon::new(Vec3d::try_from(line).unwrap()))
            .collect::<Vec<Moon>>();

        let initial_x = Moon::vec_x_only(&moons);
        let initial_y = Moon::vec_y_only(&moons);
        let initial_z = Moon::vec_z_only(&moons);

        let mut x_count = 0;
        let mut y_count = 0;
        let mut z_count = 0;

        let mut prev = initial_x.clone();
        let mut next;

        loop {
            next = Moon::step(&prev);
            x_count += 1;
            if next == initial_x {
                break;
            }
            prev = next;
        }

        prev = initial_y.clone();

        loop {
            next = Moon::step(&prev);
            y_count += 1;
            if next == initial_y {
                break;
            }
            prev = next;
        }

        prev = initial_z.clone();

        loop {
            next = Moon::step(&prev);
            z_count += 1;
            if next == initial_z {
                break;
            }
            prev = next;
        }

        println!("x {} y {} z {}", x_count, y_count, z_count);
        // now paste these numbers into a LCM calculator and get the result 354_540_398_381_256, 0
    }
}
