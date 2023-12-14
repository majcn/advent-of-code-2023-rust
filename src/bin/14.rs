advent_of_code::solution!(14);

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

type Rocks = HashSet<Point>;

struct Platform {
    rounded_rocks: Rocks,
    cube_rocks: Rocks,
    x_size: usize,
    y_size: usize,
}

fn parse_data(input: &str) -> Platform {
    let mut rounded_rocks = Rocks::new();
    let mut cube_rocks = Rocks::new();

    let x_size = input.lines().next().unwrap().len();
    let y_size = input.lines().count();

    for (y, line) in input.lines().enumerate() {
        for (x, v) in line.as_bytes().iter().enumerate() {
            match v {
                b'O' => {
                    rounded_rocks.insert(Point { x, y });
                }
                b'#' => {
                    cube_rocks.insert(Point { x, y });
                }
                _ => {}
            }
        }
    }

    Platform {
        rounded_rocks,
        cube_rocks,
        x_size,
        y_size,
    }
}

impl Platform {
    fn move_north(&mut self) {
        let mut new_rounded_rocks = Rocks::new();

        for x in 0..self.x_size {
            let mut max_location = 0;
            for y in 0..self.y_size {
                if self.rounded_rocks.contains(&Point { x, y }) {
                    new_rounded_rocks.insert(Point { x, y: max_location });
                    max_location += 1;
                } else if self.cube_rocks.contains(&Point { x, y }) {
                    max_location = y + 1;
                }
            }
        }

        self.rounded_rocks = new_rounded_rocks;
    }

    fn move_west(&mut self) {
        let mut new_rounded_rocks = Rocks::new();

        for y in 0..self.y_size {
            let mut max_location = 0;
            for x in 0..self.x_size {
                if self.rounded_rocks.contains(&Point { x, y }) {
                    new_rounded_rocks.insert(Point { x: max_location, y });
                    max_location += 1;
                } else if self.cube_rocks.contains(&Point { x, y }) {
                    max_location = x + 1;
                }
            }
        }

        self.rounded_rocks = new_rounded_rocks;
    }

    fn move_south(&mut self) {
        let mut new_rounded_rocks = Rocks::new();

        for x in 0..self.x_size {
            let mut max_location = self.y_size - 1;
            for y in (0..self.y_size).rev() {
                if self.rounded_rocks.contains(&Point { x, y }) {
                    new_rounded_rocks.insert(Point { x, y: max_location });
                    if y == 0 {
                        break;
                    }
                    max_location -= 1;
                } else if self.cube_rocks.contains(&Point { x, y }) {
                    if y == 0 {
                        break;
                    }
                    max_location = y - 1;
                }
            }
        }

        self.rounded_rocks = new_rounded_rocks;
    }

    fn move_east(&mut self) {
        let mut new_rounded_rocks = Rocks::new();

        for y in 0..self.y_size {
            let mut max_location = self.x_size - 1;
            for x in (0..self.x_size).rev() {
                if self.rounded_rocks.contains(&Point { x, y }) {
                    new_rounded_rocks.insert(Point { x: max_location, y });
                    if x == 0 {
                        break;
                    }
                    max_location -= 1;
                } else if self.cube_rocks.contains(&Point { x, y }) {
                    if x == 0 {
                        break;
                    }
                    max_location = x - 1;
                }
            }
        }

        self.rounded_rocks = new_rounded_rocks;
    }

    fn calculate_score(&self) -> u32 {
        let rev_score = self.rounded_rocks.iter().map(|rock| rock.y).sum::<usize>();

        (self.y_size * self.rounded_rocks.len() - rev_score) as u32
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
    let mut cache = HashMap::new();

    let mut i = 0;
    while i < MAX_CYCLES {
        platform.move_north();
        platform.move_west();
        platform.move_south();
        platform.move_east();

        match cache.entry(BTreeSet::from_iter(platform.rounded_rocks.iter().copied())) {
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
