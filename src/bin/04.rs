use std::error::Error;
use std::fmt::Display;
advent_of_code::solution!(4);

#[derive(Clone, PartialEq)]
enum TileState {
    Free,
    Occupied,
    Accessible,
}

#[derive(Clone)]
struct Tile {
    state: TileState,
}

impl Tile {
    fn parse(c: char) -> Result<Self, Box<dyn Error>> {
        match c {
            '.' => Ok(TileState::Free),
            '@' => Ok(TileState::Occupied),
            _ => Err(From::from("Invalid tile character")),
        }
        .map(|state| Tile { state })
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.state {
            TileState::Free => write!(f, "."),
            TileState::Occupied => write!(f, "@"),
            TileState::Accessible => write!(f, "x"),
        }
    }
}

#[derive(Clone)]
struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let grid_height = input.lines().count();
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(Tile::parse)
                    .collect::<Result<Vec<Tile>, _>>()
                    .and_then(|row| {
                        if row.len() == grid_height {
                            Ok(row)
                        } else {
                            Err(From::from("Row length mismatch"))
                        }
                    })
            })
            .collect::<Result<Vec<Vec<Tile>>, _>>()
            .map(Grid)
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<&Tile> {
        let mut neighbors = Vec::new();
        let rows = self.height() as i32;
        let cols = if rows > 0 { self.width() } else { 0 } as i32;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;

                if new_x >= 0 && new_x < rows && new_y >= 0 && new_y < cols {
                    let neighbour = &self.0[new_x as usize][new_y as usize];
                    neighbors.push(neighbour);
                }
            }
        }
        neighbors
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            for tile in line {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let original_grid = Grid::parse(input).unwrap();
    let mut updated_grid = original_grid.clone();
    let max_neighbours = 4;
    let mut accessible_tiles = 0;

    for x in 0..original_grid.width() {
        for y in 0..original_grid.height() {
            match original_grid.0[x][y].state {
                TileState::Occupied => (),
                TileState::Free | TileState::Accessible => continue,
            }
            let amount_of_neighbours = original_grid
                .get_neighbours(x, y)
                .into_iter()
                .filter(|&tile| tile.state == TileState::Occupied)
                .count();
            if amount_of_neighbours < max_neighbours {
                accessible_tiles += 1;
                updated_grid.0[x][y].state = TileState::Accessible;
            }
        }
    }
    Some(accessible_tiles)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
