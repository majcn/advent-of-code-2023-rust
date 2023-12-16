advent_of_code::solution!(16);

use advent_of_code::util::point::{Point, DOWN, LEFT, RIGHT, UP};

use std::collections::{HashMap, HashSet};

struct Grid {
    len_x: i32,
    len_y: i32,
    data: HashMap<Point, char>,
}

fn parse_data(input: &str) -> Grid {
    let len_x = input.lines().next().unwrap().len() as i32;
    let len_y = input.lines().count() as i32;

    let mut data = HashMap::with_capacity(input.len());
    for (y, line) in input.lines().enumerate() {
        for (x, v) in line.chars().enumerate() {
            data.insert(Point::new(x as i32, y as i32), v);
        }
    }

    Grid { len_x, len_y, data }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn next_directions(grid: &Grid, location: &Point, direction: &Direction) -> Vec<Direction> {
    match (grid.data[location], direction) {
        ('\\', Direction::Left) => vec![Direction::Up],
        ('\\', Direction::Right) => vec![Direction::Down],
        ('\\', Direction::Up) => vec![Direction::Left],
        ('\\', Direction::Down) => vec![Direction::Right],

        ('/', Direction::Left) => vec![Direction::Down],
        ('/', Direction::Right) => vec![Direction::Up],
        ('/', Direction::Up) => vec![Direction::Right],
        ('/', Direction::Down) => vec![Direction::Left],

        ('|', Direction::Left) => vec![Direction::Up, Direction::Down],
        ('|', Direction::Right) => vec![Direction::Up, Direction::Down],

        ('-', Direction::Up) => vec![Direction::Left, Direction::Right],
        ('-', Direction::Down) => vec![Direction::Left, Direction::Right],

        _ => vec![*direction],
    }
}

fn next_position(grid: &Grid, location: &Point, direction: &Direction) -> Option<Point> {
    let direction_point = match direction {
        Direction::Left => LEFT,
        Direction::Right => RIGHT,
        Direction::Up => DOWN,
        Direction::Down => UP,
    };

    let next_location = *location + direction_point;
    if (0..grid.len_x).contains(&next_location.x) && (0..grid.len_y).contains(&next_location.y) {
        Some(next_location)
    } else {
        None
    }
}

fn follow_the_light(grid: &Grid, start_location: Point, start_direction: Direction) -> u32 {
    let mut cache = HashSet::new();

    let mut queue = next_directions(grid, &start_location, &start_direction)
        .into_iter()
        .map(|d| (start_location, d))
        .collect::<Vec<_>>();

    while let Some(beam) = queue.pop() {
        if cache.contains(&beam) {
            continue;
        }
        cache.insert(beam);

        let (current_position, current_direction) = beam;
        if let Some(next_position) = next_position(grid, &current_position, &current_direction) {
            queue.extend(
                next_directions(grid, &next_position, &current_direction)
                    .into_iter()
                    .map(|d| (next_position, d)),
            )
        }
    }

    cache.into_iter().map(|x| x.0).collect::<HashSet<_>>().len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let result = follow_the_light(&grid, Point::new(0, 0), Direction::Right);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let m1 = (0..grid.len_y).map(|y| (0, y, Direction::Right));
    let m2 = (0..grid.len_y).map(|y| (grid.len_x - 1, y, Direction::Left));
    let m3 = (0..grid.len_x).map(|x| (x, 0, Direction::Down));
    let m4 = (0..grid.len_x).map(|x| (x, grid.len_y - 1, Direction::Up));

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
