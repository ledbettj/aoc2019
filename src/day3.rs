
const INPUT : &'static str = include_str!("../inputs/day3.txt");

#[derive(Debug,PartialEq)]
struct Line {
    p1: (isize, isize),
    p2: (isize, isize)
}

struct Wire {
    lines: Vec<Line>
}

impl Line {
    pub fn contains_point(&self, pt: (isize, isize)) -> bool {
        (pt.0 >= self.p1.0 && pt.0 <= self.p2.0 && pt.1 >= self.p1.1 && pt.1 <= self.p2.1) ||
        (pt.0 >= self.p2.0 && pt.0 <= self.p1.0 && pt.1 >= self.p2.1 && pt.1 <= self.p1.1)
    }

    pub fn points(&self) -> Vec<(isize, isize)> {
        let x_diff = self.p2.0 - self.p1.0;
        let y_diff = self.p2.1 - self.p1.1;

        let x_off = if x_diff > 0 {
            1
        } else if x_diff < 0 {
            -1
        } else {
            0
        };


        let y_off = if y_diff > 0 {
            1
        } else if y_diff < 0 {
            -1
        } else {
            0
        };

        let mut results = vec![];
        let mut point = self.p1;

        while point != self.p2 {
            results.push(point);
            point = (point.0 + x_off, point.1 + y_off)
        };
        results.push(self.p2);
        results
    }

    pub fn intersect_at(&self, other: &Line) -> Vec<(isize, isize)> {
        self
            .points()
            .into_iter()
            .filter(|&p| other.contains_point(p))
//            .inspect(|&p| println!("{:?} intersects {:?} at {:?}", self, other, p))
            .collect()
    }
}


impl Wire {
    pub fn parse(input: &str) -> Wire {
        let mut lines : Vec<Line> = vec![];
        let mut last_point = (0, 0);

        for instr in input.split(",") {
            let mut chars = instr.chars();
            let dir = chars.next().unwrap();
            let mag = chars.collect::<String>().parse::<isize>().unwrap();
            let next_point = match dir {
                'U' => (last_point.0, last_point.1 + mag),
                'D' => (last_point.0, last_point.1 - mag),
                'L' => (last_point.0 - mag, last_point.1),
                'R' => (last_point.0 + mag, last_point.1),
                _ => unreachable!()
            };
            lines.push(Line { p1: last_point, p2: next_point });
            last_point = next_point;
        }

        Wire { lines: lines }
    }

    pub fn closest_intersect_distance(&self, other: &Wire) -> isize {
        let mut intersects : Vec<(isize, isize)> = vec![];

        for l1 in &self.lines {
            for l2 in &other.lines {
                intersects.extend(l1.intersect_at(&l2).into_iter());
            }
        };
        println!("intersects: {:?}", intersects);

        intersects
            .iter()
            .filter(|&p| p != &(0, 0))
            .map(|&(x, y)| x.abs() + y.abs())
            .min()
            .expect("Does not intersect??")
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn part1_solution() {
        let wires = INPUT.lines().map(|line| Wire::parse(line)).collect::<Vec<Wire>>();

        assert_eq!(wires[0].closest_intersect_distance(&wires[1]), 0);
    }

    #[test]
    fn intersect_works() {
        let w1 = Wire::parse("R8,U5,L5,D3");
        let w2 = Wire::parse("U7,R6,D4,L4");
        assert_eq!(w1.closest_intersect_distance(&w2), 6);
        
        let w1 = Wire::parse("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let w2 = Wire::parse("U62,R66,U55,R34,D71,R55,D58,R83");

        assert_eq!(w1.closest_intersect_distance(&w2), 159);

        let w1 = Wire::parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let w2 = Wire::parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        assert_eq!(w1.closest_intersect_distance(&w2), 135);
    }
}
