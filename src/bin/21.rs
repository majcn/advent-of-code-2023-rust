advent_of_code::solution!(21);

use advent_of_code::util::bignumbers::U1024;

use std::collections::HashSet;

struct Data {
    rock_locations: HashSet<(usize, usize)>,
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
                rock_locations.insert((x, y));
            }
        }
    }

    Data {
        rock_locations,
        len_x,
        len_y,
    }
}

fn run_step(my_positions_bits: &[U1024], rocks_bits: &[U1024]) -> Vec<U1024> {
    let mut new_positions_bits = my_positions_bits.iter().copied().collect::<Vec<_>>();

    for y in 1..my_positions_bits.len() - 1 {
        let left = my_positions_bits[y] << 1;
        let right = my_positions_bits[y] >> 1;
        let up = my_positions_bits[y - 1];
        let down = my_positions_bits[y + 1];

        new_positions_bits[y] = (new_positions_bits[y] | left | right | up | down)
            & !my_positions_bits[y]
            & !rocks_bits[y]
    }

    new_positions_bits
}

pub fn part_one(input: &str) -> Option<u64> {
    let Data {
        rock_locations,
        len_x,
        len_y,
    } = parse_data(input);

    let mut rocks_bits = vec![!U1024::ZERO];
    for y in 0..len_y {
        let mut rocks_bits_line = U1024::ZERO;
        for x in 0..len_x {
            if rock_locations.contains(&(x, y)) {
                rocks_bits_line |= U1024::ONE << (x + 1)
            }
            rocks_bits_line |= U1024::ONE;
            rocks_bits_line |= U1024::ONE << (len_x + 1)
        }
        rocks_bits.push(rocks_bits_line);
    }
    rocks_bits.push(!U1024::ZERO);

    let mut my_positions_bits = (0..rocks_bits.len())
        .map(|_| U1024::ZERO)
        .collect::<Vec<_>>();
    my_positions_bits[(len_y + 2) / 2] |= U1024::ONE << ((len_x + 2) / 2);

    let my_positions_bits = (0..64).fold(my_positions_bits, |acc, _| run_step(&acc, &rocks_bits));

    let result = my_positions_bits
        .iter()
        .map(|x| x.count_ones() as u64)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let Data {
        rock_locations,
        len_x,
        len_y,
    } = parse_data(input);

    let mut rocks_bits = vec![];
    for y in 0..len_y {
        let mut rocks_bits_line = U1024::ZERO;
        for x in 0..len_x {
            if rock_locations.contains(&(x, y)) {
                rocks_bits_line |= U1024::ONE << x
            }
        }
        rocks_bits.push(rocks_bits_line);
    }

    const GRID_MULTIPLIER: usize = 7;

    let original_rocks_bits = rocks_bits;
    let mut rocks_bits = Vec::with_capacity(original_rocks_bits.len() * GRID_MULTIPLIER);
    for _ in 0..GRID_MULTIPLIER {
        rocks_bits.extend(original_rocks_bits.iter().copied());
    }

    for l_x_m in 1..GRID_MULTIPLIER {
        for r in rocks_bits.iter_mut() {
            *r |= *r << (l_x_m * len_x);
        }
    }

    let mut my_positions_bits = (0..rocks_bits.len())
        .map(|_| U1024::ZERO)
        .collect::<Vec<_>>();
    my_positions_bits[len_y * GRID_MULTIPLIER / 2] |= U1024::ONE << (len_x * GRID_MULTIPLIER / 2);

    let mut results = vec![];

    let magic_number_1 = len_x as i64 / 2;
    let magic_number_2 = magic_number_1 + len_x as i64;
    let magic_number_3 = magic_number_2 + len_x as i64;

    let mut i = 0;
    while results.len() < 3 {
        my_positions_bits = run_step(&my_positions_bits, &rocks_bits);

        i += 1;

        if i == magic_number_1 || i == magic_number_2 || i == magic_number_3 {
            let i_result = my_positions_bits
                .iter()
                .map(|x| x.count_ones() as i64)
                .sum::<i64>();

            results.push((i, i_result));
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
