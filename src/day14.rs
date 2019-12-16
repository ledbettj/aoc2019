const INPUT : &'static str = include_str!("../inputs/day14.txt");

use std::fmt;

use std::collections::{HashMap,VecDeque};

use nom::{
    IResult,
    character::complete::{alpha1, digit1, space1, newline},
    bytes::complete::tag,
    combinator::map_res,
    multi::separated_list
};

#[derive(Debug,Eq,PartialEq,Clone,Hash)]
struct Reagent {
    count: usize,
    chemical: String
}

impl fmt::Display for Reagent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.count, self.chemical)
    }
}

#[derive(Debug,Eq,PartialEq,Clone,Hash)]
struct Reaction {
    output: Reagent,
    input:  Vec<Reagent>
}

impl fmt::Display for Reaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for input in &self.input {
            write!(f, "{} ", input)?;
        }
        write!(f, "=> {}", self.output)
    }
}


#[derive(Debug,Eq,PartialEq,Clone)]
struct ReactionMap {
    map: HashMap<String, Reaction>
}

impl Reaction {
    fn parse(input: &str) -> IResult<&str, Reaction> {
        let (input, ingredients) = Reagent::parse_list(input)?;
        let (input, _) = tag(" => ")(input)?;
        let (input, outcome) = Reagent::parse(input)?;

        let reaction = Reaction {
            input: ingredients,
            output: outcome
        };

        Ok((input, reaction))
    }
}

impl ReactionMap {

    // How many units of `units` does it cost to produce result
    pub fn cost(&self, result: &Reagent, units: &str) -> usize {

        let mut required : HashMap<String, usize> = HashMap::new();
        let mut ore = 0;

        required.insert(result.chemical.clone(), result.count);

        while !required.is_empty() {
            for (chemical, count) in required.iter_mut() {
                

            }
        }

        ore
    }

    fn parse(input: &str) -> Result<ReactionMap, nom::Err<(&str, nom::error::ErrorKind)>> {
        let (_, items) = separated_list(newline, Reaction::parse)(input)?;

        let map = items
            .iter()
            .map(|reaction| (reaction.output.chemical.to_string(), reaction.clone()))
            .collect::<HashMap<String, Reaction>>();

        Ok(ReactionMap { map: map })
    }
}


impl Reagent {
    fn parse(input: &str) -> IResult<&str, Reagent> {
        let (input, cnt) = map_res(
            digit1,
            |s: &str| s.parse::<usize>()
        )(input)?;

        let (input, _) = space1(input)?;
        let (input, name) = alpha1(input)?;

        let reagent = Reagent::new(name, cnt);
        Ok((input, reagent))
    }

    fn parse_list(input: &str) -> IResult<&str, Vec<Reagent>> {
        separated_list(tag(", "), Reagent::parse)(input)
    }

    pub fn new<S: Into<String>>(name: S, count: usize) -> Reagent {
        Reagent { count: count, chemical: name.into() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_reagent() {
        assert_eq!(Reagent::parse("5 BORK"), Ok(("", Reagent::new("BORK", 5))));
    }

    #[test]
    fn parse_reagent_list() {
        assert_eq!(Reagent::parse_list("5 BORK"), Ok(("", vec![Reagent::new("BORK", 5)])));
        assert_eq!(
            Reagent::parse_list("5 BORK, 7 FOO"),
            Ok(("", vec![Reagent::new("BORK", 5), Reagent::new("FOO", 7)]))
        );
    }

    #[test]
    fn p1_example() {
        let input = "9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL";
        let map = ReactionMap::parse(input).expect("Failed to parse map!");

        assert_eq!(
            map.cost(&Reagent::new("FUEL", 1), "ORE"),
            165
        );
    }

    #[test]
    fn p1_solution() {
        let map = ReactionMap::parse(INPUT).expect("Failed to parse map!");
    }

}
