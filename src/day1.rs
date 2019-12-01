
const INPUT : &'static str  = include_str!("../inputs/day1.txt");

fn module_masses() -> Vec<usize> {
    INPUT
        .lines()
        .map(|line| line.parse::<usize>().expect("Invalid integer value"))
        .collect()
}

pub fn module_fuel_required(module_mass: usize) -> usize {
    (module_mass / 3) - 2
}

pub fn craft_fuel_required(modules: &Vec<usize>) -> usize {
    modules
        .iter()
        .map(|&mass| module_fuel_required(mass))
        .fold(0, |a, b| a + b)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_fuel_required_works() {
        assert_eq!(module_fuel_required(12), 2);
        assert_eq!(module_fuel_required(14), 2);
        assert_eq!(module_fuel_required(1969), 654);
        assert_eq!(module_fuel_required(100756), 33583);
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(craft_fuel_required(&module_masses()), 3412496);
    }
}
