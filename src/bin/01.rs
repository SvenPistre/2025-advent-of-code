advent_of_code::solution!(1);

#[derive(Debug)]
enum Rotation {
    Left { turns: i64 },
    Right { turns: i64 },
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
struct Dial {
    position: i64,
    zero_crossings: u64,
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            position: 50,
            zero_crossings: 0,
        }
    }
}

impl Dial {
    fn rotate_by(self, rotation: &Rotation) -> Self {
        let zero_crossings = match rotation {
            Rotation::Right { turns } => (self.position + turns).div_euclid(100),
            Rotation::Left { turns } => {
                if self.position == 0 {
                    turns.div_euclid(100)
                } else {
                    -(self.position - turns - 1).div_euclid(100)
                }
            }
        };

        let new_position = match rotation {
            Rotation::Right { turns } => (self.position + turns).rem_euclid(100),
            Rotation::Left { turns } => (self.position - turns).rem_euclid(100),
        };

        Self {
            position: new_position,
            zero_crossings: self.zero_crossings + zero_crossings.unsigned_abs(),
        }
    }
}

fn count_zero_positions(initial_position: Dial, rotations: Vec<Rotation>) -> u64 {
    let mut zero_counter = 0;
    rotations.iter().fold(initial_position, |pos, rot| {
        let new_pos = pos.rotate_by(rot);
        if new_pos.position == 0 {
            zero_counter += 1;
        }
        new_pos
    });
    zero_counter
}

pub fn part_one(input: &str) -> Option<u64> {
    let rotations: Vec<Rotation> = input.lines().filter_map(Rotation::parse).collect();
    let zero_position_counter = count_zero_positions(Dial::default(), rotations);
    Some(zero_position_counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rotations: Vec<Rotation> = input.lines().filter_map(Rotation::parse).collect();
    let final_position = rotations
        .iter()
        .fold(Dial::default(), |pos, rot| pos.rotate_by(rot));
    Some(final_position.zero_crossings)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_correctly_double_counting_when_landing_on_zero_after_left_rotation() {
        let data = ["L50", "L50", "L100", "L150"].join("\n");
        let result = part_two(&data);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_correctly_double_counting_when_landing_on_zero_after_right_rotation() {
        let data = ["R50", "R50", "R100", "R150"].join("\n");
        let result = part_two(&data);
        assert_eq!(result, Some(4));
    }
}
