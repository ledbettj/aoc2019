const INPUT : &'static str = include_str!("../inputs/day17.txt");

use std::collections::HashMap;
use crate::intcode::{Computer,Program,InvalidInstruction,IOEvent};


type Point = (usize, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_solution() {
        let mut p = Program::parse(INPUT).expect("Unable to load program");

        let output = p.execute(&vec![]).expect("Error running program");

        let mut grid = HashMap::new();

        let mut x = 0;
        let mut y = 0;

        for value in &output {
            let p = (x, y);
            let ch = *value as u8 as char;
            print!("{}", ch);
            match ch {
                '\n' => {
                    x = 0;
                    y += 1;
                },
                _ => {
                    grid.insert(p, ch);
                    x += 1;
                }

            }
        }

        let neighbors = vec![
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0)
        ];

        let v = grid
            .iter()
            .filter(|&((px, py), &ch)|{
                ch == '#' &&
                    neighbors
                    .iter()
                    .all(|(dx, dy)|{
                        grid
                            .get(&(px + dx, py + dy))
                            .unwrap_or(&' ') == &'#'
                    })

            })
            .map(|((px, py), _)| px * py)
            .sum::<i32>();


        assert_eq!(v, 6520);
    }
}
