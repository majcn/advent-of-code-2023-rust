advent_of_code::solution!(11);

use advent_of_code::maneatingape::hash::*;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

struct Grid {
    data: FastSet<Point>,
    max_x: usize,
    max_y: usize,
}

fn parse_data(input: &str) -> Grid {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut data = FastSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, v) in line.as_bytes().iter().enumerate() {
            if v == &b'#' {
                data.insert(Point { x, y });
                max_x = usize::max(max_x, x);
                max_y = usize::max(max_y, y);
            }
        }
    }

    Grid { data, max_x, max_y }
}

fn part_x<const N: usize>(grid: Grid) -> u64 {
    let expand_x = (0..=grid.max_x)
        .filter(|&x| !(0..=grid.max_y).any(|y| grid.data.contains(&Point { x, y })))
        .collect::<Vec<_>>();

    let expand_y = (0..=grid.max_y)
        .filter(|&y| !(0..=grid.max_x).any(|x| grid.data.contains(&Point { x, y })))
        .collect::<Vec<_>>();

    let grid_data_as_vec = grid
        .data
        .into_iter()
        .map(|p| Point {
            x: p.x + (N - 1) * expand_x.iter().filter(|ex| &p.x > ex).count(),
            y: p.y + (N - 1) * expand_y.iter().filter(|ey| &p.y > ey).count(),
        })
        .collect::<Vec<_>>();

    let mut result = 0;
    for i in 0..grid_data_as_vec.len() {
        for j in (i + 1)..grid_data_as_vec.len() {
            let p1 = &grid_data_as_vec[i];
            let p2 = &grid_data_as_vec[j];
            result += usize::abs_diff(p1.x, p2.x) + usize::abs_diff(p1.y, p2.y)
        }
    }

    result as u64
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
