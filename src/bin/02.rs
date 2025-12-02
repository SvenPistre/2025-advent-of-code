use std::error::Error;

advent_of_code::solution!(2);

#[derive(Debug)]
struct Range(u64, u64);

impl Range {
    fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let parts: Vec<u64> = input.trim().split('-').flat_map(|n| n.parse()).collect();
        if parts.len() != 2 {
            return Err(From::from("too many parts"));
        }
        if parts[0] >= parts[1] {
            return Err(From::from("order is wrong"));
        }
        Ok(Range(parts[0], parts[1]))
    }
}

mod part_one {
    use super::*;

    pub fn find_invalid_ids(range: &Range) -> Vec<u64> {
        let mut invalid_ids = vec![];
        for id in range.0..=range.1 {
            if is_invalid_id(id) {
                invalid_ids.push(id);
            }
        }
        invalid_ids
    }

    fn is_invalid_id(num: u64) -> bool {
        if num < 10 {
            return false;
        }
        let num_str = num.to_string();
        let divisor = 2;
        let (first, _) = num_str.split_at(num_str.len() / divisor);
        first.repeat(divisor).parse::<u64>().unwrap() == num
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<Range> = input
        .split(',')
        .map(Range::parse)
        .map(|res| res.expect("input should be valid"))
        .collect();
    let invalid_ids: Vec<u64> = ranges.iter().flat_map(part_one::find_invalid_ids).collect();
    Some(invalid_ids.iter().sum::<u64>())
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_one_find_invalid_ids_works() {
        let ranges: Vec<(Range, Vec<u64>)> = vec![
            (Range(11, 22), vec![11, 22]),
            (Range(95, 115), vec![99]),
            (Range(998, 1012), vec![1010]),
            (Range(1188511880, 1188511890), vec![1188511885]),
            (Range(222220, 222224), vec![222222]),
            (Range(1698522, 1698528), vec![]),
            (Range(446443, 446449), vec![446446]),
            (Range(38593856, 38593862), vec![38593859]),
        ];
        ranges.iter().for_each(|(range, expected)| {
            let actual = part_one::find_invalid_ids(range);
            assert_eq!(actual, *expected);
        });
    }
}
