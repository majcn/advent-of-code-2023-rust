advent_of_code::solution!(10);

use advent_of_code::util::list::Array2D;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Grid {
    data: Array2D<Vec<Direction>>,
    len_x: usize,
    len_y: usize,
}

struct State {
    grid: Grid,
    start_position: (usize, usize),
}

fn parse_data(input: &str) -> State {
    let len_x = input.lines().next().unwrap().len();
    let len_y = input.lines().count();

    let start_position = input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| line.chars().position(|v| v == 'S').map(|x| (x, y)))
        .next()
        .unwrap();

    let mut grid = Array2D::new(len_x);

    input
        .lines()
        .map(|line| {
            line.as_bytes().iter().map(|v| match v {
                b'|' => vec![Direction::Down, Direction::Up],
                b'-' => vec![Direction::Left, Direction::Right],
                b'L' => vec![Direction::Up, Direction::Right],
                b'J' => vec![Direction::Left, Direction::Up],
                b'7' => vec![Direction::Left, Direction::Down],
                b'F' => vec![Direction::Right, Direction::Down],
                _ => vec![],
            })
        })
        .for_each(|line| grid.add_line(line));

    State {
        grid: Grid {
            data: grid,
            len_x,
            len_y,
        },
        start_position,
    }
}

fn get_next_position(
    grid: &Grid,
    position: &(usize, usize),
    direction: &Direction,
) -> Option<(usize, usize)> {
    let result = match direction {
        Direction::Left => (position.0.wrapping_sub(1), position.1),
        Direction::Right => (position.0 + 1, position.1),
        Direction::Up => (position.0, position.1.wrapping_sub(1)),
        Direction::Down => (position.0, position.1 + 1),
    };

    if (0..grid.len_x).contains(&result.0) && (0..grid.len_y).contains(&result.1) {
        Some(result)
    } else {
        None
    }
}

fn part_x(
    grid: &Grid,
    start_position: (usize, usize),
    start_direction: Direction,
) -> Option<Vec<(usize, usize)>> {
    let mut result = vec![start_position];

    if let Some(new_position) = get_next_position(grid, &start_position, &start_direction) {
        result.push(new_position);
    } else {
        return None;
    }

    loop {
        let prev_position = &result[result.len() - 2];
        let position = &result[result.len() - 1];

        let new_possible_position = grid.data[position]
            .iter()
            .filter_map(|d| get_next_position(grid, position, d))
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

const STARTING_DIRECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Right,
    Direction::Up,
    Direction::Down,
];

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
        .map(|w| (w[0].1 as i32 + w[1].1 as i32) * (w[0].0 as i32 - w[1].0 as i32))
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
