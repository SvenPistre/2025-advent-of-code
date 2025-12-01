advent_of_code::solution!(1);

#[derive(Debug)]
enum Rotation {
    Left { turns: i32 },
    Right { turns: i32 },
}

impl Rotation {
    fn parse(input: &str) -> Option<Rotation> {
        let input = input.trim();
        if input.is_empty() {
            return None;
        }

        let direction = input.chars().next()?;
        let turns_str = &input[1..];
        let turns = turns_str.parse().ok()?;

        match direction {
            'L' => Some(Rotation::Left { turns }),
            'R' => Some(Rotation::Right { turns }),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Dial(i32);

impl Default for Dial {
    fn default() -> Self {
        Self(50)
    }
}

impl Dial {
    fn rotate_by(&mut self, rotation: &Rotation) -> Self {
        match rotation {
            Rotation::Left { turns } => self.0 = (self.0 - turns).rem_euclid(100),
            Rotation::Right { turns } => self.0 = (self.0 + turns).rem_euclid(100),
        };
        Self(self.0)
    }
}

fn count_zero_positions(initial_position: Dial, rotations: Vec<Rotation>) -> u64 {
    let mut current_position = initial_position;
    let mut zero_position_counter = 0;
    for rotation in rotations {
        current_position.rotate_by(&rotation);
        if current_position.0 == 0 {
            zero_position_counter += 1;
        }
    }
    zero_position_counter
}

pub fn part_one(input: &str) -> Option<u64> {
    let rotations: Vec<Rotation> = input.lines().filter_map(Rotation::parse).collect();
    let zero_position_counter = count_zero_positions(Dial::default(), rotations);
    Some(zero_position_counter)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u64> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
