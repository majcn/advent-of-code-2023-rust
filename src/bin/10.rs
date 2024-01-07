advent_of_code::solution!(10);

use advent_of_code::maneatingape::point::*;

use advent_of_code::util::list::Array2D;

type Direction = u8;

struct State {
    grid: Array2D<Vec<Direction>>,
    start_position: Point,
}

fn parse_data(input: &str) -> State {
    let len_x = input.lines().next().unwrap().len();

    let start_position = input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            line.chars()
                .position(|v| v == 'S')
                .map(|x| Point::new(x as i32, y as i32))
        })
        .next()
        .unwrap();

    let mut grid = Array2D::default(len_x);

    input
        .lines()
        .map(|line| {
            line.as_bytes().iter().map(|v| match v {
                b'|' => vec![b'D', b'U'],
                b'-' => vec![b'L', b'R'],
                b'L' => vec![b'U', b'R'],
                b'J' => vec![b'L', b'U'],
                b'7' => vec![b'L', b'D'],
                b'F' => vec![b'R', b'D'],
                _ => vec![],
            })
        })
        .for_each(|line| grid.add_line(line));

    State {
        grid,
        start_position,
    }
}

fn get_next_position(
    grid: &Array2D<Vec<Direction>>,
    position: Point,
    direction: Direction,
) -> Option<Point> {
    let result = position + Point::from(direction);

    if grid.contains(&result) {
        Some(result)
    } else {
        None
    }
}

fn part_x(
    grid: &Array2D<Vec<Direction>>,
    start_position: Point,
    start_direction: Direction,
) -> Option<Vec<Point>> {
    let mut result = vec![start_position];

    if let Some(new_position) = get_next_position(grid, start_position, start_direction) {
        result.push(new_position);
    } else {
        return None;
    }

    loop {
        let prev_position = &result[result.len() - 2];
        let position = result[result.len() - 1];

        let new_possible_position = grid[&position]
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
    let state = parse_data(input);

    let best_path = STARTING_DIRECTIONS
        .into_iter()
        .filter_map(|d| part_x(&state.grid, state.start_position, d))
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap();

    let result = best_path.len() as u32 / 2;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let state = parse_data(input);

    let best_path = STARTING_DIRECTIONS
        .into_iter()
        .filter_map(|d| part_x(&state.grid, state.start_position, d))
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
