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

#[derive(Debug, PartialEq)]
enum MathMode {
    Human,
    Cephalopod,
}

fn parse_input(input: &str, math_mode: MathMode) -> Result<Vec<Problem>, Box<dyn Error>> {
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
    if math_mode == MathMode::Human {
        return Ok(problems);
    }
    let height_of_problem = problems[0].numbers.len();
    let widths_of_problems: Vec<usize> = problems
        .iter()
        .flat_map(|problem| problem.numbers.iter().max())
        .map(|num| num.to_string().len())
        .collect();
    assert_eq!(
        widths_of_problems.len(),
        number_of_problems,
        "Widths of problems do not match number of problems"
    );

    let mut problem_matrices = (0..number_of_problems)
        .map(|idx| vec![vec![' '; widths_of_problems[idx]]; height_of_problem])
        .collect::<Vec<_>>();
    for (row, line) in input.lines().enumerate() {
        if row == height_of_problem {
            continue;
        }
        let mut line_chars = line.chars();
        for problem_idx in 0..number_of_problems {
            problem_matrices[problem_idx][row] = line_chars
                .by_ref()
                .take(widths_of_problems[problem_idx])
                .collect();
            line_chars.next();
        }
    }
    for (idx, matrix) in problem_matrices.iter().enumerate() {
        let rows = matrix.len();
        let columns = matrix[0].len();

        let mut numbers: Vec<u64> = Vec::new();
        for column in (0..columns).rev() {
            let mut number = 0_u64;
            let mut number_of_digits = rows;
            for row in (0..rows).rev() {
                if matrix[row][column].is_whitespace() {
                    number_of_digits -= 1;
                    continue;
                }
                let digit = matrix[row][column].to_string().parse::<u64>().unwrap_or(0);
                let factor = 10_u64.pow(
                    (number_of_digits - row - 1)
                        .try_into()
                        .expect("Should fit into u32"),
                );
                number += digit * factor;
            }
            numbers.push(number);
        }
        problems[idx].numbers = numbers;
    }
    Ok(problems)
}

pub fn part_one(input: &str) -> Option<u64> {
    let problems = parse_input(input, MathMode::Human).unwrap();
    let results = problems.iter().map(|problem| match problem.operator {
        Operator::Add => problem.numbers.iter().sum::<u64>(),
        Operator::Multiply => problem.numbers.iter().product::<u64>(),
    });
    Some(results.sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let problems = parse_input(input, MathMode::Cephalopod).unwrap();
    let results = problems.iter().map(|problem| match problem.operator {
        Operator::Add => problem.numbers.iter().sum::<u64>(),
        Operator::Multiply => problem.numbers.iter().product::<u64>(),
    });
    Some(results.sum())
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
        assert_eq!(result, Some(3263827));
    }
}
