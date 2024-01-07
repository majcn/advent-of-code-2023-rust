advent_of_code::solution!(11);

use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::point::*;

struct Grid {
    data: FastSet<Point>,
    len_x: i32,
    len_y: i32,
}

fn parse_data(input: &str) -> Grid {
    let len_x = input.lines().next().unwrap().len() as i32;
    let len_y = input.lines().count() as i32;
    let mut data = FastSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, v) in line.as_bytes().iter().enumerate() {
            if v == &b'#' {
                data.insert(Point::new(x as i32, y as i32));
            }
        }
    }

    Grid { data, len_x, len_y }
}

fn part_x<const N: i32>(grid: Grid) -> u64 {
    let expand_x = (0..grid.len_x)
        .filter(|&x| !(0..grid.len_y).any(|y| grid.data.contains(&Point::new(x, y))))
        .collect::<Vec<_>>();

    let expand_y = (0..grid.len_y)
        .filter(|&y| !(0..grid.len_x).any(|x| grid.data.contains(&Point::new(x, y))))
        .collect::<Vec<_>>();

    let grid_data_as_vec = grid
        .data
        .into_iter()
        .map(|p| {
            Point::new(
                p.x + (N - 1) * expand_x.iter().filter(|ex| &p.x > ex).count() as i32,
                p.y + (N - 1) * expand_y.iter().filter(|ey| &p.y > ey).count() as i32,
            )
        })
        .collect::<Vec<_>>();

    let mut result = 0;
    for i in 0..grid_data_as_vec.len() {
        for j in (i + 1)..grid_data_as_vec.len() {
            let p1 = &grid_data_as_vec[i];
            let p2 = &grid_data_as_vec[j];
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
