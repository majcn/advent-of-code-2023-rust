advent_of_code::solution!(21);

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::point::*;

type UBig = advent_of_code::majcn::bignumbers::UX64<13>;

fn parse_data(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn run_step(my_positions_bits: &[UBig], rocks_bits: &[UBig]) -> Vec<UBig> {
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
    my_positions_bits: Vec<UBig>,
    rocks_bits: Vec<UBig>,
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
    let len_x = grid.width as usize;
    let len_y = grid.height as usize;

    // convert to bits and add borders
    let mut rocks_bits = vec![UBig::ZERO; len_y + 2];
    *rocks_bits.first_mut().unwrap() = !UBig::ZERO;
    *rocks_bits.last_mut().unwrap() = !UBig::ZERO;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid[Point::new(x, y)] == b'#' {
                rocks_bits[y as usize + 1] |= UBig::ONE << (x as usize + 1)
            }
            rocks_bits[y as usize + 1] |= UBig::ONE;
            rocks_bits[y as usize + 1] |= UBig::ONE << (len_x + 1)
        }
    }
    let mut my_positions_bits = vec![UBig::ZERO; rocks_bits.len()];
    my_positions_bits[(len_y + 2) / 2] |= UBig::ONE << ((len_x + 2) / 2);

    let result = part_x([64], my_positions_bits, rocks_bits)
        .into_iter()
        .next()
        .map(|x| x.1 as u64)
        .unwrap();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_data(input);
    let len_x = grid.width as usize;
    let len_y = grid.height as usize;

    // convert to bits and expand GRID_MULTIPLIER times
    let mut rocks_bits = vec![UBig::ZERO; len_y];
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid[Point::new(x, y)] == b'#' {
                rocks_bits[y as usize] |= UBig::ONE << x as usize
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
        .map(|_| UBig::ZERO)
        .collect::<Vec<_>>();
    my_positions_bits[len_y * GRID_MULTIPLIER / 2] |= UBig::ONE << (len_x * GRID_MULTIPLIER / 2);

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
    let x = NUMBER_OF_STEPS;
    let [(x1, y1), (x2, y2), (x3, y3)] = results;
    let result = (x - x2) / (x1 - x2) * (x - x3) / (x1 - x3) * y1
        + (x - x1) / (x2 - x1) * (x - x3) / (x2 - x3) * y2
        + (x - x1) / (x3 - x1) * (x - x2) / (x3 - x2) * y3;

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
        assert_eq!(result, Some(528192758996204));
    }
}
