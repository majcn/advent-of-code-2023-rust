advent_of_code::solution!(21);

use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    x: usize,
    y: usize,
}

struct Data {
    rock_locations: HashSet<Location>,
    len_x: usize,
    len_y: usize,
}

fn parse_data(input: &str) -> Data {
    let len_x = input.lines().next().unwrap().len();
    let len_y = input.lines().count();

    let mut rock_locations = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, v) in line.chars().enumerate() {
            if v == '#' {
                rock_locations.insert(Location { x, y });
            }
        }
    }

    Data {
        rock_locations,
        len_x,
        len_y,
    }
}

// def do_magic(my_positions_bits):
//     new_position_bits = [x for x in my_positions_bits]
//     for y in range(1, len_y - 1):
//         left = my_positions_bits[y] << 1
//         right = my_positions_bits[y] >> 1
//         up = my_positions_bits[y-1]
//         down = my_positions_bits[y+1]

//         new_position_bits[y] = (new_position_bits[y] | left | right | up | down) & ~my_positions_bits[y] & ~rocks_bits[y]

//     return new_position_bits

fn run_step(
    my_locations: HashSet<Location>,
    rock_locations: &HashSet<Location>,
    len_x: usize,
    len_y: usize,
) -> HashSet<Location> {
    let mut new_locations = HashSet::with_capacity(my_locations.len() * 4);

    for loc in my_locations.into_iter() {
        let left = Location {
            x: loc.x.wrapping_sub(1),
            y: loc.y,
        };

        let right = Location {
            x: loc.x + 1,
            y: loc.y,
        };

        let up = Location {
            x: loc.x,
            y: loc.y.wrapping_sub(1),
        };

        let down = Location {
            x: loc.x,
            y: loc.y + 1,
        };

        new_locations.extend([left, right, up, down].iter().filter(|l| {
            !rock_locations.contains(&Location {
                x: l.x % len_x,
                y: l.y % len_y,
            })
        }));
    }

    new_locations
}

pub fn part_one(input: &str) -> Option<u32> {
    let Data {
        rock_locations,
        len_x,
        len_y,
    } = parse_data(input);

    let start_location = Location {
        x: len_x / 2,
        y: len_y / 2,
    };

    let my_locations = (0..64).fold(HashSet::from([start_location]), |acc, _| {
        run_step(acc, &rock_locations, len_x, len_y)
            .into_iter()
            .filter(|l| (0..len_x).contains(&l.x) && (0..len_y).contains(&l.y))
            .collect()
    });

    let result = my_locations.len() as u32;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let Data {
        rock_locations,
        len_x,
        len_y,
    } = parse_data(input);

    let start_location = Location {
        x: len_x * 2 + len_x / 2,
        y: len_y * 2 + len_y / 2,
    };

    let mut results = vec![];
    let mut my_locations = HashSet::from([start_location]);

    let magic_number_1 = len_x / 2;
    let magic_number_2 = magic_number_1 + len_x;
    let magic_number_3 = magic_number_2 + len_x;

    let mut i = 0;
    while results.len() < 3 {
        my_locations = run_step(my_locations, &rock_locations, len_x, len_y);

        i += 1;

        if i == magic_number_1 || i == magic_number_2 || i == magic_number_3 {
            results.push((i as i64, my_locations.len() as i64));
        }
    }

    const NUMBER_OF_STEPS: i64 = 26501365;

    // Lagrange Interpolation (magic_numbers as points)
    let mut result = 0;
    for (mx, my) in results.iter() {
        let mut part = *my;
        for (ox, oy) in results.iter() {
            if my != oy {
                part = part * (NUMBER_OF_STEPS - ox) / (mx - ox);
            }
        }
        result += part;
    }

    let result = result as u64;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(528192879457051));
    }
}
