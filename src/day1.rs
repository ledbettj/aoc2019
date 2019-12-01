
const INPUT : &'static str  = include_str!("../inputs/day1.txt");

/// The problem input for Day 1 is a list of module masses.
/// We use a signed integer here because for part 2 we need to check if the result
/// of the fuel required calculation is less than zero.
fn module_masses() -> Vec<isize> {
    INPUT
        .lines()
        .map(|line| line.parse::<isize>().expect("Invalid integer value"))
        .collect()
}

/// Part 1: Calculate the fuel required for a given module based on it's mass.
pub fn module_fuel_required(module_mass: isize) -> isize {
    ((module_mass / 3) - 2).max(0)
}

/// Part 1: Calculate the fuel required for a given space craft,
/// based on all it's component's masses.
pub fn craft_fuel_required(modules: &Vec<isize>) -> isize {
    modules
        .iter()
        .map(|&mass| module_fuel_required(mass))
        .fold(0, |a, b| a + b)
}

/// Part 2: Calculate the fuel required for a given module,
/// based on it's masse, making sure to include fuel for the added fuel.
pub fn module_fuel_required_including_fuel(module_mass: isize) -> isize {
    let mut fuel_mass = ((module_mass / 3) - 2).max(0);
    let mut total_fuel = 0;

    while fuel_mass > 0 {
        total_fuel += fuel_mass;
        fuel_mass = (fuel_mass / 3) - 2;
    }

    total_fuel
}

/// Part 2: Calculate the fuel required for a given space craft,
/// based on all it's component's masses, making sure to include fuel for
/// the added fuel.
pub fn craft_fuel_required_including_fuel(modules: &Vec<isize>) -> isize {
    modules
        .iter()
        .map(|&mass| module_fuel_required_including_fuel(mass) )
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


    #[test]
    fn module_fuel_required_including_fuel_works() {
        assert_eq!(module_fuel_required_including_fuel(14), 2);
        assert_eq!(module_fuel_required_including_fuel(1969), 966);
        assert_eq!(module_fuel_required_including_fuel(100756), 50346);
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(craft_fuel_required_including_fuel(&module_masses()), 5115845);
    }

}
