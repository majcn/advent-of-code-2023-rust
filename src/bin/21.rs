advent_of_code::solution!(21);

use advent_of_code::maneatingape::point::*;

use advent_of_code::util::bignumbers::U1024;
use advent_of_code::util::list::Array2D;

fn parse_data(input: &str) -> Array2D<u8> {
    Array2D::new(input)
}

fn run_step(my_positions_bits: &[U1024], rocks_bits: &[U1024]) -> Vec<U1024> {
    let mut new_positions_bits = my_positions_bits.to_vec();

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

fn part_x<const N: usize>(
    snapshots: [usize; N],
    my_positions_bits: Vec<U1024>,
    rocks_bits: Vec<U1024>,
) -> [(i64, i64); N] {
    let mut result = [(0, 0); N];

    let mut my_positions_bits = my_positions_bits;

    let mut i = 0;
    for (snapshot_index, snapshot) in snapshots.into_iter().enumerate() {
        for _ in i..snapshot {
            my_positions_bits = run_step(&my_positions_bits, &rocks_bits);
        }
        i = snapshot;

        result[snapshot_index] = (
            snapshot as i64,
            my_positions_bits
                .iter()
                .map(|v| v.count_ones() as i64)
                .sum(),
        );
    }

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_data(input);
    let len_x = grid.len_x() as usize;
    let len_y = grid.len_y() as usize;

    // convert to bits and add borders
    let mut rocks_bits = vec![U1024::ZERO; len_y + 2];
    *rocks_bits.first_mut().unwrap() = !U1024::ZERO;
    *rocks_bits.last_mut().unwrap() = !U1024::ZERO;
    for y in 0..grid.len_y() {
        for x in 0..grid.len_x() {
            if grid[&Point::new(x, y)] == b'#' {
                rocks_bits[y as usize + 1] |= U1024::ONE << (x as usize + 1)
            }
            rocks_bits[y as usize + 1] |= U1024::ONE;
            rocks_bits[y as usize + 1] |= U1024::ONE << (len_x + 1)
        }
    }
    let mut my_positions_bits = vec![U1024::ZERO; rocks_bits.len()];
    my_positions_bits[(len_y + 2) / 2] |= U1024::ONE << ((len_x + 2) / 2);

    let result = part_x([64], my_positions_bits, rocks_bits)
        .into_iter()
        .next()
        .map(|x| x.1 as u64)
        .unwrap();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_data(input);
    let len_x = grid.len_x() as usize;
    let len_y = grid.len_y() as usize;

    // convert to bits and expand GRID_MULTIPLIER times
    let mut rocks_bits = vec![U1024::ZERO; len_y];
    for y in 0..grid.len_y() {
        for x in 0..grid.len_x() {
            if grid[&Point::new(x, y)] == b'#' {
                rocks_bits[y as usize] |= U1024::ONE << x as usize
            }
        }
    }

    const GRID_MULTIPLIER: usize = 7;
    let new_len = rocks_bits.len() * GRID_MULTIPLIER;

    for i in 1..GRID_MULTIPLIER {
        for r in rocks_bits.iter_mut() {
            *r |= *r << (i * len_x);
        }
    }
    let rocks_bits = rocks_bits
        .into_iter()
        .cycle()
        .take(new_len)
        .collect::<Vec<_>>();

    let mut my_positions_bits = (0..rocks_bits.len())
        .map(|_| U1024::ZERO)
        .collect::<Vec<_>>();
    my_positions_bits[len_y * GRID_MULTIPLIER / 2] |= U1024::ONE << (len_x * GRID_MULTIPLIER / 2);

    let magic_number_1 = len_x / 2;
    let magic_number_2 = magic_number_1 + len_x;
    let magic_number_3 = magic_number_2 + len_x;

    let results = part_x(
        [magic_number_1, magic_number_2, magic_number_3],
        my_positions_bits,
        rocks_bits,
    );

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
