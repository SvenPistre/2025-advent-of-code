use std::error::Error;

advent_of_code::solution!(6);

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Clone, Debug)]
struct Problem {
    numbers: Vec<u64>,
    operator: Operator,
}

impl Default for Problem {
    fn default() -> Self {
        Self {
            numbers: Vec::new(),
            operator: Operator::Add,
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Problem>, Box<dyn Error>> {
    let number_of_problems = input.lines().collect::<Vec<_>>()[0]
        .split_whitespace()
        .count();
    let mut problems: Vec<Problem> = vec![Problem::default(); number_of_problems];

    for line in input.lines() {
        for (column, word) in line.split_whitespace().enumerate() {
            let parsed_number = word.parse::<u64>();
            if let Ok(number) = parsed_number {
                problems[column].numbers.push(number);
            } else {
                match word {
                    "+" => problems[column].operator = Operator::Add,
                    "*" => problems[column].operator = Operator::Multiply,
                    _ => return Err(From::from("Failed to parse input")),
                }
            }
        }
    }
    Ok(problems)
}

pub fn part_one(input: &str) -> Option<u64> {
    let problems = parse_input(input).unwrap();
    let results = problems.iter().map(|problem| match problem.operator {
        Operator::Add => problem.numbers.iter().sum::<u64>(),
        Operator::Multiply => problem.numbers.iter().product::<u64>(),
    });
    Some(results.sum())
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
