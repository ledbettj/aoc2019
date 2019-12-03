use std::collections::{HashSet,HashMap};

const INPUT : &'static str = include_str!("../inputs/day3.txt");

type Point = (isize, isize);


struct Wire {
    points: Vec<Point>,
    costs: HashMap<Point, usize>
}

impl Wire {
    pub fn parse(input: &str) -> Wire {
        let mut points : Vec<Point> = vec![];
        let mut last = (0, 0);

        for instr in input.split(",") {
            let mut chars = instr.chars();
            let dir = chars.next().unwrap();
            let mag = chars.collect::<String>().parse::<usize>().unwrap();
            let more = match dir {
                'U' => Wire::generate_points(last, (0,  1), mag),
                'D' => Wire::generate_points(last, (0, -1), mag),
                'L' => Wire::generate_points(last, (-1, 0), mag),
                'R' => Wire::generate_points(last, (1,  0), mag),
                _ => unreachable!()
            };
            points.extend(more.iter());
            last = *points.last().unwrap();
        }

        let mut cost_map = HashMap::new();
        for (index, &p) in points.iter().enumerate() {
            cost_map.entry(p)
                .or_insert(index + 1);
        }
        Wire { points: points, costs: cost_map }
    }

    fn generate_points(from: Point, dir: Point, count: usize) -> Vec<Point> {
        let mut last = from;
        let mut results = vec![];
        for _ in 0..count {
            let next = (last.0 + dir.0, last.1 + dir.1);
            results.push(next);
            last = next;
        }

        results
    }

    pub fn cost(&self, point: Point) -> usize {
        self.costs[&point]
    }

    pub fn intersections(&self, other: &Wire) -> Vec<Point> {
        let mine  = self.points
            .iter()
            .cloned()
            .collect::<HashSet<Point>>();
        let yours = other.points
            .iter()
            .cloned()
            .collect::<HashSet<Point>>();

        mine.intersection(&yours).cloned().collect::<Vec<Point>>()
    }

    pub fn closest_intersection_distance(&self, other: &Wire) -> isize {
        let intersects = self.intersections(other);

        intersects
            .iter()
            .map(|&(x, y)| x.abs() + y.abs())
            .min()
            .expect("Do not intersect?")
    }

    pub fn lowest_cost_intersection_cost(&self, other: &Wire) -> usize {
        let intersects = self.intersections(other);

        intersects
            .iter()
            .map(|&p| self.cost(p) + other.cost(p))
            .min()
            .expect("Do not intersect?")

    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn part1_solution() {
        let wires = INPUT.lines().map(|line| Wire::parse(line)).collect::<Vec<Wire>>();

        assert_eq!(wires[0].closest_intersection_distance(&wires[1]), 1285);
    }

    #[test]
    fn intersect_works() {
        let w1 = Wire::parse("R8,U5,L5,D3");
        let w2 = Wire::parse("U7,R6,D4,L4");
        assert_eq!(w1.closest_intersection_distance(&w2), 6);
        
        let w1 = Wire::parse("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let w2 = Wire::parse("U62,R66,U55,R34,D71,R55,D58,R83");

        assert_eq!(w1.closest_intersection_distance(&w2), 159);

        let w1 = Wire::parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let w2 = Wire::parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        assert_eq!(w1.closest_intersection_distance(&w2), 135);
    }

    #[test]
    fn cost_works() {
        let w1 = Wire::parse("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let w2 = Wire::parse("U62,R66,U55,R34,D71,R55,D58,R83");

        assert_eq!(w1.lowest_cost_intersection_cost(&w2), 610);

        let w1 = Wire::parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let w2 = Wire::parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        assert_eq!(w1.lowest_cost_intersection_cost(&w2), 410);
    }

    #[test]
    fn part2_solution() {
        let wires = INPUT.lines().map(|line| Wire::parse(line)).collect::<Vec<Wire>>();

        assert_eq!(wires[0].lowest_cost_intersection_cost(&wires[1]), 14228);
    }
}
