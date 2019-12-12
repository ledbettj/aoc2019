const INPUT : &'static str = include_str!("../inputs/day10.txt");

use std::collections::HashSet;

type Point = (i32, i32);

pub fn load_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)|{
            row
                .chars()
                .enumerate()
                .filter(|&(index, ch)| ch == '#')
                .map(|(index, _)| (y as i32, index as i32))
                .collect::<Vec<Point>>()

        })
        .collect::<Vec<Point>>()
}

pub fn angle_from(a: &Point, b: &Point) -> f64 {
    ((b.1 - a.1) as f64).atan2((b.0 - a.0) as f64)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_solution() {
        let points = load_points(INPUT);
        let m = points
            .iter()
            .enumerate()
            .map(|(index, point)| {
                let mut set = HashSet::new();
                for i in 0..points.len() {
                    if i == index {
                        continue;
                    }
                    let angle = angle_from(&point, &points[i]);
                    set.insert(angle.to_bits());
                }
                set.len()
            })
            .max();

        assert_eq!(m.unwrap(), 274);
    }
}
