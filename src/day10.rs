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
                .filter(|&(_, ch)| ch == '#')
                .map(|(index, _)| (index as i32, y as i32))
                .collect::<Vec<Point>>()

        })
        .collect::<Vec<Point>>()
}

pub fn angle_from(a: &Point, b: &Point) -> f64 {
    ((b.1 - a.1) as f64).atan2((b.0 - a.0) as f64)
}

pub fn distance_squared(a: &Point, b: &Point) -> i32 {
    ((b.0 - a.0).pow(2) + (b.1 - a.1).pow(2))
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

    #[test]
    fn p2_solution() {
        let origin = (19, 14); // answer from p1 above
        let mut points =
            load_points(INPUT)
            .iter()
            .filter(|&p| p != &origin)
            .cloned()
            .collect::<Vec<Point>>();

        // points are now sorted by distance from origin.
        // next group them by angle from origin
        points.sort_by_key(|p|{
            let mut angle = angle_from(&origin, &p).to_degrees();
            angle = if angle < -90.0 {
                180.0 + angle.abs()
            } else {
                angle
            };
            (
                (angle * 10_000.0) as i32,
                distance_squared(&origin, p)
            )
        });

        let mut last_angle = -100.0;

        for i in 0..200 {
            let index = points
                .iter()
                .position(|p|{
                    let mut angle = angle_from(&origin, &p).to_degrees();
                    angle = if angle < -90.0 {
                        180.0 + angle.abs()
                    } else {
                        angle
                    };
//                    println!("item {:?} angle {:?}", p, angle);

                    let ok = angle > last_angle;
                    if ok {
                        last_angle = angle;
                    }
                    ok
                })
                .unwrap();
            println!("{} removed {:?} last={:?}", i, points.remove(index), last_angle);
        }
        // for (i, p) in points.iter().enumerate() {
        //     println!("{}\tp={:?}, a={:?} d={:?}", i, p, angle_from(&origin, &p).to_degrees(), distance_squared(&origin, &p));
        // }
    }
}
