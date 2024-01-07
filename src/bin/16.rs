advent_of_code::solution!(16);

use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::point::*;

use advent_of_code::util::list::Array2D;

type Direction = u8;

fn parse_data(input: &str) -> Array2D<u8> {
    Array2D::new(input)
}

fn next_directions(grid: &Array2D<u8>, location: &Point, direction: Direction) -> Vec<Direction> {
    match (grid[location], direction) {
        (b'\\', b'L') => vec![b'U'],
        (b'\\', b'R') => vec![b'D'],
        (b'\\', b'U') => vec![b'L'],
        (b'\\', b'D') => vec![b'R'],

        (b'/', b'L') => vec![b'D'],
        (b'/', b'R') => vec![b'U'],
        (b'/', b'U') => vec![b'R'],
        (b'/', b'D') => vec![b'L'],

        (b'|', b'L') => vec![b'U', b'D'],
        (b'|', b'R') => vec![b'U', b'D'],

        (b'-', b'U') => vec![b'L', b'R'],
        (b'-', b'D') => vec![b'L', b'R'],

        _ => vec![direction],
    }
}

fn next_position(grid: &Array2D<u8>, location: Point, direction: Direction) -> Option<Point> {
    let result = location + Point::from(direction);

    if grid.contains(&result) {
        Some(result)
    } else {
        None
    }
}

fn follow_the_light(grid: &Array2D<u8>, start_location: Point, start_direction: Direction) -> u32 {
    let mut cache = FastSet::new();

    let mut queue = next_directions(grid, &start_location, start_direction)
        .into_iter()
        .map(|d| (start_location, d))
        .collect::<Vec<_>>();

    while let Some(beam) = queue.pop() {
        if cache.contains(&beam) {
            continue;
        }
        cache.insert(beam);

        let (current_position, current_direction) = beam;
        if let Some(next_position) = next_position(grid, current_position, current_direction) {
            queue.extend(
                next_directions(grid, &next_position, current_direction)
                    .into_iter()
                    .map(|d| (next_position, d)),
            )
        }
    }

    cache.into_iter().map(|x| x.0).collect::<FastSet<_>>().len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let result = follow_the_light(&grid, Point::new(0, 0), b'R');

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let m1 = (0..grid.len_y()).map(|y| (0, y, b'R'));
    let m2 = (0..grid.len_y()).map(|y| (grid.len_x() - 1, y, b'L'));
    let m3 = (0..grid.len_x()).map(|x| (x, 0, b'D'));
    let m4 = (0..grid.len_x()).map(|x| (x, grid.len_y() - 1, b'U'));

    let result = m1
        .chain(m2)
        .chain(m3)
        .chain(m4)
        .map(|(x, y, d)| follow_the_light(&grid, Point::new(x, y), d))
        .max()
        .unwrap();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
