use std::collections::HashMap;

const INPUT : &'static str = include_str!("../inputs/day6.txt");

#[derive(Debug)]
struct OrbitMap {
    map: HashMap::<String, String>
}

// An interator which yields the ancestors of the given object.
// For example, given A -- B -- C -- D, ancestors(D) will yield C, B, A.
struct OrbitMapAncestorIterator<'a> {
    current:   &'a str,
    orbit_map: &'a OrbitMap
}

impl<'a> Iterator for OrbitMapAncestorIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.orbit_map
            .get(self.current)
            .map(|parent|{
                self.current = parent;
                parent.as_str()
            })
    }
}

#[derive(Debug,PartialEq)]
enum MapLoadError {
    ParseError(String)
}

impl OrbitMap {
    // Given a string containing a single specification "A)B" per line,
    // Return an OrbitMap representing the relationship between all the
    // contents.
    // Returns a MapLoadError if something goes wrong.
    pub fn parse(input: &str) -> Result<OrbitMap, MapLoadError> {
        // a list of (key, value) tuples can be collected into a HashMap.
        let map = input
            .lines()
            .map(OrbitMap::parse_line)
            .collect::<Result<HashMap<String, String>, MapLoadError>>()?;

        Ok(OrbitMap { map: map })
    }

    // Given a line like "YOU)SAN" return a tuple of ("YOU", "SAN").
    // Returns an Error if the line has too many or not enough parts.
    fn parse_line(line: &str) -> Result<(String, String), MapLoadError> {
        let mut parts = line.split(")");
        // ensure that the line is exactly two items separated by a )
        match (parts.next(), parts.next(), parts.next()) {
            (Some(a), Some(b), None) => Ok((b.to_string(), a.to_string())),
            _ => Err(MapLoadError::ParseError(line.to_string()))
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    // Given the name of an object, like 'SAN', return the parent object,
    // That is the object that the given key orbits.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    // Returns an interator that yields all the parents of the given object.
    pub fn ancestors<'a>(&'a self, object: &'a str) -> OrbitMapAncestorIterator<'a> {
        OrbitMapAncestorIterator { current: object, orbit_map: self }
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

    // Returns how many nodes are above this node in the tree.
    fn orbit_count(&self, orbiter: &str) -> usize {
        self
            .ancestors(orbiter)
            .count()
    }

    // Return the distance from the parent node to the target node.
    // Returns None if the ancestor is not actually an ancestor of target.
    pub fn distance_from_parent(&self, parent: &str, target: &str) -> Option<usize> {
        self
            .ancestors(target)
            .position(|ancestor| parent == ancestor)
    }

    // Return the distance between two nodes, through their common parent node.
    // If they do not have a common parent (they totally should?) returns None.
    pub fn transfer_distance(&self, first: &str, second: &str) -> Option<usize> {
        // Goal is to find the lowest common ancenstor of `from` and `to`, and then sum
        // distance from the ancestor to each node.
        let result = self
            .ancestors(second)
            .map(|ancestor| (ancestor, self.distance_from_parent(ancestor, first)) )
            .find(|(_, dist)| dist.is_some());


        result.and_then(|(parent, distance)|{
            distance.and_then(|value| Some(value + self.distance_from_parent(parent, second).unwrap()))
        })
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


    #[test]
    fn p2_example() {
        let m = OrbitMap::parse("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN")
            .expect("Failed to parse Orbit Map");

        assert_eq!(m.transfer_distance("YOU", "SAN"), Some(4));
    }

    #[test]
    fn p2_solution() {
        let m = OrbitMap::parse(INPUT).expect("Failed to parse Orbit Map");

        assert_eq!(m.transfer_distance("YOU", "SAN"), Some(373));

    }

    #[test]
    fn parse_error_works() {
        let m = OrbitMap::parse("COM)B)C");
        assert_eq!(m.is_err(), true);
        assert_eq!(m.unwrap_err(), MapLoadError::ParseError("COM)B)C".to_string()));

        let m = OrbitMap::parse("A)B\nCOM");
        assert_eq!(m.is_err(), true);
        assert_eq!(m.unwrap_err(), MapLoadError::ParseError("COM".to_string()));
    }
}
