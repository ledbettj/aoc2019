use std::collections::{HashSet,HashMap};

const INPUT : &'static str = include_str!("../inputs/day3.txt");


type Point = (isize, isize);

/// We store the list of points the wire contains, along with the "cost",
/// Which is the count of how many steps it takes to get to the given point as
/// they are generated.  If a point is included twice, it's cost is the lower cost.
struct Wire {
    costs: HashMap<Point, usize>
}

impl Wire {
    pub fn parse(input: &str) -> Wire {
        let mut last = (0, 0);
        let mut cost_map = HashMap::new();
        let mut cost = 1;

        for instr in input.split(",") {
            let mut chars = instr.chars();
            let dir = chars.next().unwrap();

            let mag = chars
                .collect::<String>()
                .parse::<usize>()
                .expect("Failed to parse magnitude of line!");

            let vec = match dir {
                'U' => (0,  1),
                'D' => (0, -1),
                'L' => (-1, 0),
                'R' => (1,  0),
                c => panic!("Unexpected direction indicator: {:?}", c)
            };

            Wire::generate_points(last, vec, mag, |point| {
                cost_map.entry(point).or_insert(cost);
                cost += 1;
                last = point;
            });
        }

        Wire { costs: cost_map }
    }

    /// Given a starting point, a direction unit vector, and a count,
    /// Call the given function with all the points on this line.
    fn generate_points<F>(from: Point, dir: Point, count: usize, mut f: F)
    where F: FnMut(Point) {
        let mut last = from;
        for _ in 0..count {
            let next = (last.0 + dir.0, last.1 + dir.1);
            f(next);
            last = next;
        };
    }

    pub fn cost(&self, point: Point) -> usize {
        self.costs[&point]
    }

    /// Find the collection of points that intersection between two Wires.
    pub fn intersections(&self, other: &Wire) -> Vec<Point> {
        let mine  = self.costs.keys()
            .cloned()
            .collect::<HashSet<Point>>();
        let yours = other.costs.keys()
            .cloned()
            .collect::<HashSet<Point>>();

        mine.intersection(&yours).cloned().collect::<Vec<Point>>()
    }

    /// Find the closest intersection distance (by manhattan distance) of two Wires.
    pub fn closest_intersection_distance(&self, other: &Wire) -> isize {
        let intersects = self.intersections(other);

        intersects
            .iter()
            .map(|&(x, y)| x.abs() + y.abs())
            .min()
            .expect("Do not intersect?")
    }

    /// Find the closest intersection cost (by sum of point cost) of two Wires.
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
