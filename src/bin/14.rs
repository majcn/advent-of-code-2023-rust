advent_of_code::solution!(14);

use std::collections::BTreeSet;

use advent_of_code::maneatingape::hash::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

type Rocks = FastSet<Point>;

struct Platform {
    rounded_rocks: Rocks,
    cube_rocks: Rocks,
    len_x: usize,
    len_y: usize,
}

fn parse_data(input: &str) -> Platform {
    let mut rounded_rocks = Rocks::new();
    let mut cube_rocks = Rocks::new();

    let len_x = input.lines().next().unwrap().len();
    let len_y = input.lines().count();

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
        len_x,
        len_y,
    }
}

impl Platform {
    fn move_north(&mut self) {
        let mut new_rounded_rocks = Rocks::new();

        for x in 0..self.len_x {
            let mut max_location = 0;
            for y in 0..self.len_y {
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

        for y in 0..self.len_y {
            let mut max_location = 0;
            for x in 0..self.len_x {
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

        for x in 0..self.len_x {
            let mut max_location = self.len_y - 1;
            for y in (0..self.len_y).rev() {
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

        for y in 0..self.len_y {
            let mut max_location = self.len_x - 1;
            for x in (0..self.len_x).rev() {
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

        (self.len_y * self.rounded_rocks.len() - rev_score) as u32
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
