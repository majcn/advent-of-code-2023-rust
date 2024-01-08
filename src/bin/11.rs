advent_of_code::solution!(11);

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::point::*;

fn parse_data(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn part_x<const N: i32>(grid: Grid<u8>) -> u64 {
    let expand_x = (0..grid.width)
        .filter(|&x| !(0..grid.height).any(|y| grid[Point::new(x, y)] == b'#'))
        .collect::<Vec<_>>();

    let expand_y = (0..grid.height)
        .filter(|&y| !(0..grid.width).any(|x| grid[Point::new(x, y)] == b'#'))
        .collect::<Vec<_>>();

    let mut grid_data_as_vec = Vec::with_capacity(grid.bytes.len());
    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid[Point::new(x, y)] == b'#' {
                grid_data_as_vec.push(Point::new(
                    x + (N - 1) * expand_x.iter().filter(|ex| &x > ex).count() as i32,
                    y + (N - 1) * expand_y.iter().filter(|ey| &y > ey).count() as i32,
                ));
            }
        }
    }

    let mut result = 0;
    for i in 0..grid_data_as_vec.len() {
        for j in (i + 1)..grid_data_as_vec.len() {
            let p1 = grid_data_as_vec[i];
            let p2 = grid_data_as_vec[j];
            result += i32::abs_diff(p1.x, p2.x) as u64 + i32::abs_diff(p1.y, p2.y) as u64
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_data(input);

    let result = part_x::<2>(grid);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_data(input);

    let result = part_x::<1000000>(grid);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
