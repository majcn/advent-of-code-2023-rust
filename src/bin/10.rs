advent_of_code::solution!(10);

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::point::*;

type Direction = u8;

fn parse_data(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn get_next_position(grid: &Grid<u8>, position: Point, direction: Direction) -> Option<Point> {
    let result = position + Point::from(direction);

    if grid.contains(result) {
        Some(result)
    } else {
        None
    }
}

fn part_x(grid: &Grid<u8>, start_direction: Direction) -> Option<Vec<Point>> {
    let start_position = grid.find(b'S').unwrap();

    let mut result = vec![start_position];

    if let Some(new_position) = get_next_position(grid, start_position, start_direction) {
        result.push(new_position);
    } else {
        return None;
    }

    loop {
        let prev_position = &result[result.len() - 2];
        let position = result[result.len() - 1];

        let new_directions = match grid[position] {
            b'|' => [b'D', b'U'],
            b'-' => [b'L', b'R'],
            b'L' => [b'U', b'R'],
            b'J' => [b'L', b'U'],
            b'7' => [b'L', b'D'],
            b'F' => [b'R', b'D'],
            _ => unreachable!(),
        };

        let new_possible_position = new_directions
            .iter()
            .filter_map(|d| get_next_position(grid, position, *d))
            .find(|next_position| next_position != prev_position);

        if let Some(new_position) = new_possible_position {
            result.push(new_position);

            if new_position == start_position {
                return Some(result);
            }
        } else {
            return None;
        }
    }
}

const STARTING_DIRECTIONS: [Direction; 4] = [b'L', b'R', b'U', b'D'];

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let best_path = STARTING_DIRECTIONS
        .into_iter()
        .filter_map(|d| part_x(&grid, d))
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap();

    let result = best_path.len() as u32 / 2;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let best_path = STARTING_DIRECTIONS
        .into_iter()
        .filter_map(|d| part_x(&grid, d))
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap();

    // Shoelace formula
    let area_twice = best_path
        .windows(2)
        .map(|w| (w[0].y + w[1].y) * (w[0].x - w[1].x))
        .sum::<i32>()
        .unsigned_abs();

    // Pick's theorem
    let boundaries = best_path.len() as u32 - 1;
    let interior_points = (area_twice / 2) - boundaries / 2 + 1;

    let result = interior_points;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }
}
