advent_of_code::solution!(14);

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::point::*;

struct Platform {
    grid: Grid<u8>,
    rounded_rocks: Vec<Point>,
}

fn parse_data(input: &str) -> Platform {
    let grid = Grid::parse(input);

    let mut rounded_rocks = vec![];
    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid[Point::new(x, y)] == b'O' {
                rounded_rocks.push(Point::new(x, y));
            }
        }
    }

    Platform {
        grid,
        rounded_rocks,
    }
}

impl Platform {
    fn update_state(&mut self, new_rounded_rocks: Vec<Point>) {
        let mut new_rounded_rocks = new_rounded_rocks;

        std::mem::swap(&mut self.rounded_rocks, &mut new_rounded_rocks);
        let old_rounded_rocks = new_rounded_rocks;

        for old_rounded_rock in old_rounded_rocks {
            self.grid[old_rounded_rock] = b'.';
        }

        for &new_rounded_rock in self.rounded_rocks.iter() {
            self.grid[new_rounded_rock] = b'O';
        }
    }

    fn move_north(&mut self) {
        let mut new_rounded_rocks = vec![];

        for x in 0..self.grid.width {
            let mut max_location = 0;
            for y in 0..self.grid.height {
                match self.grid[Point::new(x, y)] {
                    b'O' => {
                        new_rounded_rocks.push(Point::new(x, max_location));
                        max_location += 1
                    }
                    b'#' => max_location = y + 1,
                    _ => {}
                }
            }
        }

        self.update_state(new_rounded_rocks);
    }

    fn move_west(&mut self) {
        let mut new_rounded_rocks = vec![];

        for y in 0..self.grid.height {
            let mut max_location = 0;
            for x in 0..self.grid.width {
                match self.grid[Point::new(x, y)] {
                    b'O' => {
                        new_rounded_rocks.push(Point::new(max_location, y));
                        max_location += 1
                    }
                    b'#' => max_location = x + 1,
                    _ => {}
                }
            }
        }

        self.update_state(new_rounded_rocks);
    }

    fn move_south(&mut self) {
        let mut new_rounded_rocks = vec![];

        for x in 0..self.grid.width {
            let mut max_location = self.grid.height - 1;
            for y in (0..self.grid.height).rev() {
                match self.grid[Point::new(x, y)] {
                    b'O' => {
                        new_rounded_rocks.push(Point::new(x, max_location));
                        max_location -= 1
                    }
                    b'#' => max_location = y - 1,
                    _ => {}
                }
            }
        }

        self.update_state(new_rounded_rocks);
    }

    fn move_east(&mut self) {
        let mut new_rounded_rocks = vec![];

        for y in 0..self.grid.height {
            let mut max_location = self.grid.width - 1;
            for x in (0..self.grid.width).rev() {
                match self.grid[Point::new(x, y)] {
                    b'O' => {
                        new_rounded_rocks.push(Point::new(max_location, y));
                        max_location -= 1
                    }
                    b'#' => max_location = x - 1,
                    _ => {}
                }
            }
        }

        self.update_state(new_rounded_rocks);
    }

    fn calculate_score(&self) -> u32 {
        let rev_score = self.rounded_rocks.iter().map(|rock| rock.y).sum::<i32>();

        (self.grid.height * self.rounded_rocks.len() as i32 - rev_score) as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut platform = parse_data(input);

    platform.move_north();
    let result = platform.calculate_score();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    const MAX_CYCLES: usize = 1000000000;

    let mut platform = parse_data(input);
    let mut cache = FastMap::new();

    let mut i = 0;
    while i < MAX_CYCLES {
        platform.move_north();
        platform.move_west();
        platform.move_south();
        platform.move_east();

        match cache.entry(platform.rounded_rocks.clone()) {
            std::collections::hash_map::Entry::Occupied(v) => {
                let diff = i - v.get();
                let multiplier = (MAX_CYCLES - i) / diff;
                i += diff * multiplier;
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(i);
            }
        }

        i += 1;
    }

    let result = platform.calculate_score();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
