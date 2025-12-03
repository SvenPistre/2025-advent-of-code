use std::error::Error;

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

    fn max_joltage(&self, total_batteries: usize) -> u64 {
        let mut max_joltage = 0;
        let mut start = 0;
        let length = self.0.len();

        for battery_pos in 0..total_batteries {
            let end = length - (total_batteries - battery_pos) + 1;
            let (max_digit_index, max_digit) = self.0[start..end]
                .iter()
                .enumerate()
                .reduce(|current_max, item| {
                    if item.1 > current_max.1 {
                        item
                    } else {
                        current_max
                    }
                })
                .unwrap();
            start += max_digit_index + 1;
            max_joltage = max_joltage * 10 + max_digit;
        }
        max_joltage
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let battery_banks: Vec<BatteryBank> = input
        .lines()
        .map(BatteryBank::parse)
        .map(Result::unwrap)
        .collect();
    Some(battery_banks.iter().map(|bank| bank.max_joltage(2)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let battery_banks: Vec<BatteryBank> = input
        .lines()
        .map(BatteryBank::parse)
        .map(Result::unwrap)
        .collect();
    Some(battery_banks.iter().map(|bank| bank.max_joltage(12)).sum())
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
        assert_eq!(result, Some(3121910778619));
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

    #[test]
    fn test_max_joltage_for_two_batteries() {
        let battery_banks: Vec<(BatteryBank, u64)> = vec![
            (BatteryBank::parse("987654321111111").unwrap(), 98),
            (BatteryBank::parse("811111111111119").unwrap(), 89),
            (BatteryBank::parse("234234234234278").unwrap(), 78),
            (BatteryBank::parse("818181911112111").unwrap(), 92),
        ];

        battery_banks.iter().for_each(|(bank, expected)| {
            let actual = bank.max_joltage(2);
            assert_eq!(actual, *expected);
        });
    }

    #[test]
    fn test_max_joltage_for_twelve_batteries() {
        let battery_banks: Vec<(BatteryBank, u64)> = vec![
            (BatteryBank::parse("987654321111111").unwrap(), 987654321111),
            (BatteryBank::parse("811111111111119").unwrap(), 811111111119),
            (BatteryBank::parse("234234234234278").unwrap(), 434234234278),
            (BatteryBank::parse("818181911112111").unwrap(), 888911112111),
        ];

        battery_banks.iter().for_each(|(bank, expected)| {
            let actual = bank.max_joltage(12);
            assert_eq!(actual, *expected);
        });
    }
}
