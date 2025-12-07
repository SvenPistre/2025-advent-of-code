use std::{error::Error, fmt::Display};

advent_of_code::solution!(7);

enum Part {
    Start,
    FreeSpace,
    Splitter,
    Beam,
}

impl Part {
    fn parse(c: char) -> Result<Self, Box<dyn Error>> {
        match c.to_string().as_str() {
            "S" => Ok(Part::Start),
            "." => Ok(Part::FreeSpace),
            "^" => Ok(Part::Splitter),
            "|" => Ok(Part::Beam),
            _ => Err(From::from("Invalid input")),
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Start => write!(f, "S"),
            Part::FreeSpace => write!(f, "."),
            Part::Splitter => write!(f, "^"),
            Part::Beam => write!(f, "|"),
        }
    }
}

struct TachyonManifold(Vec<Vec<Part>>);

impl TachyonManifold {
    fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let parsed_input = input
            .lines()
            .map(|line| line.chars().map(Part::parse).collect::<Result<Vec<_>, _>>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(parsed_input))
    }
}

impl Display for TachyonManifold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for parts in &self.0 {
            for part in parts {
                write!(f, "{}", part)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let manifold = TachyonManifold::parse(input).unwrap();
    println!("{}", manifold);
    None
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
