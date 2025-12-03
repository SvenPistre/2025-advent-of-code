use std::error::Error;

use itertools::Itertools;

advent_of_code::solution!(3);

#[derive(Debug)]
struct BatteryBank(Vec<u64>);

impl BatteryBank {
    fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        input
            .trim()
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(u64::from)
                    .ok_or_else(|| "Invalid digit".into())
            })
            .collect::<Result<Vec<u64>, _>>()
            .map(BatteryBank)
    }

    fn max_joltage(&self, number_of_needed_batteries: usize) -> u64 {
        self.0
            .iter()
            .combinations(number_of_needed_batteries)
            .map(|combination| to_decimal(combination))
            .max()
            .expect("BatteryBank should have at least two batteries")
    }
}

fn to_decimal(digits: Vec<&u64>) -> u64 {
    digits.iter().enumerate().fold(0, |acc, (i, &x)| {
        acc + x * 10_u64.pow((digits.len() - i - 1) as u32)
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    let battery_banks: Vec<BatteryBank> = input
        .lines()
        .map(BatteryBank::parse)
        .map(Result::unwrap)
        .collect();
    Some(battery_banks.iter().map(|bank| bank.max_joltage(2)).sum())
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_max_joltage_for_two_batteries_at_start() {
        let battery_bank = BatteryBank::parse("8912345").unwrap();
        assert_eq!(battery_bank.max_joltage(2), 95);
    }

    #[test]
    fn test_max_joltage_for_two_batteries_in_middle() {
        let battery_bank = BatteryBank::parse("123458912345").unwrap();
        assert_eq!(battery_bank.max_joltage(2), 95);
    }

    #[test]
    fn test_max_joltage_for_two_batteries_at_end() {
        let battery_bank = BatteryBank::parse("1234589").unwrap();
        assert_eq!(battery_bank.max_joltage(2), 89);
    }

    #[test]
    fn test_max_joltage_for_two_batteries_split_up() {
        let battery_bank = BatteryBank::parse("12845123495").unwrap();
        assert_eq!(battery_bank.max_joltage(2), 95);
    }

    #[test]
    fn test_max_joltage_for_two_batteries_duplicate_max_digits_at_start() {
        let battery_bank = BatteryBank::parse("991291").unwrap();
        assert_eq!(battery_bank.max_joltage(2), 99);
    }

    #[test]
    fn test_max_joltage_for_two_batteries_duplicate_max_digits_in_mid() {
        let battery_bank = BatteryBank::parse("19291").unwrap();
        assert_eq!(battery_bank.max_joltage(2), 99);
    }

    #[test]
    fn test_max_joltage_for_two_batteries_duplicate_max_digits_at_end() {
        let battery_bank = BatteryBank::parse("129199").unwrap();
        assert_eq!(battery_bank.max_joltage(2), 99);
    }
    }
}
