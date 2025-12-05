use std::{error::Error, ops::RangeInclusive};

use rangetools::Rangetools;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Range(RangeInclusive<u64>);

impl Range {
    fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let bounds: Vec<u64> = input
            .split('-')
            .map(|n| {
                n.parse::<u64>()
                    .map_err(|_| From::from("Failed to parse input"))
            })
            .collect::<Result<Vec<u64>, Box<dyn Error>>>()?;
        if bounds.iter().len() == 2 && bounds[0] <= bounds[1] {
            Ok(Range(bounds[0]..=bounds[1]))
        } else {
            Err(From::from("Invalid range"))
        }
    }
}

#[derive(Debug)]
struct Ingredient {
    id: u64,
}

fn parse_input(input: &str) -> Result<(Vec<Range>, Vec<Ingredient>), Box<dyn Error>> {
    let mut parsing_ranges_finished = false;
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            parsing_ranges_finished = true;
            continue;
        }
        if parsing_ranges_finished {
            let ingredient = Ingredient {
                id: line
                    .parse()
                    .map_err(|_| -> Box<dyn Error> { From::from("Wrong input for ingredient") })?,
            };
            ingredients.push(ingredient);
        } else {
            let range = Range::parse(line)?;
            ranges.push(range);
        }
    }
    Ok((ranges, ingredients))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ingredients) = parse_input(input).unwrap();
    let combined_range = ranges
        .iter()
        .fold(ranges[0].0.clone().to_set(), |acc, range| {
            acc.union(range.0.clone())
        });
    let mut fresh_ingredients = 0;
    for ingredient in &ingredients {
        if combined_range.contains(ingredient.id) {
            fresh_ingredients += 1
        }
    }
    Some(fresh_ingredients)
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
