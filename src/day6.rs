use std::collections::HashMap;

const INPUT : &'static str = include_str!("../inputs/day6.txt");

struct OrbitMap {
    map: HashMap::<String, String>
}

#[derive(Debug)]
enum MapLoadError {
    ParsError
}


impl OrbitMap {
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn parse(input: &str) -> Result<OrbitMap, MapLoadError> {
        let map = input
            .lines()
            .map(|line|{
                let mut parts = line.split(")");
                let orbitee = parts.next().ok_or(MapLoadError::ParsError)?;
                let orbiter = parts.last().ok_or(MapLoadError::ParsError)?;

                Ok((orbiter.to_string(), orbitee.to_string()))
            })
            .collect::<Result<HashMap<String, String>, MapLoadError>>()?;

        Ok(OrbitMap { map: map })
    }

    pub fn total_orbit_count(&self) -> usize {
        let mut counts = HashMap::<&str, usize>::new();

        self.map
            .keys()
            .map(|orbiter|{
                let count = counts
                    .entry(orbiter)
                    .or_insert(self.orbit_count(orbiter));

                *count
            })
            .sum()
    }

    fn orbit_count(&self, orbiter: &str) -> usize {
        let mut count = 0;
        let mut o = orbiter;

        while let Some(orbitee) = &self.map.get(o) {
            o = orbitee;
            count += 1;
        }

        count
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn p1_example() {
        let m = OrbitMap::parse("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L")
            .expect("Failed to parse Orbit Map");

        assert_eq!(m.len(), 11);

        assert_eq!(m.total_orbit_count(), 42);

    }
    #[test]
    fn p1_solution() {
        let m = OrbitMap::parse(INPUT).expect("Failed to parse Orbit Map");

        assert_eq!(m.total_orbit_count(), 160040);

    }
}
