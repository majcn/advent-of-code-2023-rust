advent_of_code::solution!(10);

use advent_of_code::util::point::Point;

use std::collections::HashMap;
use std::collections::HashSet;

type Grid = HashMap<Point, (Point, Point)>;

struct State {
    grid: Grid,
    start_position: Point,
}

fn parse_data(input: &str) -> State {
    let mut start_position = Point::new(-1, -1);

    let mut grid = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, v) in line.as_bytes().iter().enumerate() {
            if v == &b'S' {
                start_position = Point::new(x as i32, y as i32);
                continue;
            }

            let directions = match v {
                b'|' => Some((Point::new(0, -1), Point::new(0, 1))),
                b'-' => Some((Point::new(-1, 0), Point::new(1, 0))),
                b'L' => Some((Point::new(0, -1), Point::new(1, 0))),
                b'J' => Some((Point::new(0, -1), Point::new(-1, 0))),
                b'7' => Some((Point::new(0, 1), Point::new(-1, 0))),
                b'F' => Some((Point::new(0, 1), Point::new(1, 0))),
                _ => None,
            };

            if let Some(directions) = directions {
                grid.insert(Point::new(x as i32, y as i32), directions);
            }
        }
    }

    State {
        grid,
        start_position,
    }
}

const STARTING_DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: -1, y: 0 },
];

fn part_x(start_position: Point, start_direction: Point, grid: &Grid) -> Option<Vec<Point>> {
    let mut prev_position = start_position;
    let mut position = start_position + start_direction;

    let mut result = vec![start_position];

    loop {
        result.push(position);

        if position == start_position {
            return Some(result);
        }

        if !grid.contains_key(&position) {
            return None;
        }

        let directions = grid.get(&position).unwrap();
        let position_1 = position + directions.0;
        let position_2 = position + directions.1;

        if position_1 == prev_position {
            prev_position = position;
            position = position_2;
        } else {
            prev_position = position;
            position = position_1;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let state = parse_data(input);

    let best_path = STARTING_DIRECTIONS
        .into_iter()
        .filter_map(|d| part_x(state.start_position, d, &state.grid))
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap();

    let result = best_path.len() as u32 / 2;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let state = parse_data(input);

    let best_path = STARTING_DIRECTIONS
        .into_iter()
        .filter_map(|d| part_x(state.start_position, d, &state.grid))
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap();

    let max_x = best_path.iter().map(|x| x.x).max().unwrap();
    let max_y = best_path.iter().map(|x| x.y).max().unwrap();

    let all_points_iter = (0..=max_x).flat_map(|x| (0..=max_y).map(move |y| Point { x, y }));

    let best_path_as_set = best_path.iter().collect::<HashSet<_>>();
    let result = all_points_iter
        .filter(|p| !best_path_as_set.contains(p))
        .filter(|p| {
            let counter = best_path
                .windows(2)
                .filter(|w| {
                    if (p.y < w[0].y) == (p.y < w[1].y) {
                        return false;
                    }

                    p.x < w[0].x + ((p.y - w[0].y) / (w[1].y - w[0].y)) * (w[1].x - w[0].x)
                })
                .count();

            counter % 2 == 1
        })
        .count() as u32;

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
