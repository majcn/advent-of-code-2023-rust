advent_of_code::solution!(16);

use advent_of_code::util::list::Array2D;

use std::collections::HashSet;

struct Grid {
    len_x: usize,
    len_y: usize,
    data: Array2D<char>,
}

fn parse_data(input: &str) -> Grid {
    let len_x = input.lines().next().unwrap().len();
    let len_y = input.lines().count();

    let mut data = Array2D::new(len_x);
    input.lines().for_each(|line| data.add_line(line.chars()));

    Grid { len_x, len_y, data }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn next_directions(grid: &Grid, location: &(usize, usize), direction: &Direction) -> Vec<Direction> {
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

fn next_position(
    grid: &Grid,
    location: &(usize, usize),
    direction: &Direction,
) -> Option<(usize, usize)> {
    let (next_x, next_y) = match direction {
        Direction::Left => (location.0.wrapping_sub(1), location.1),
        Direction::Right => (location.0 + 1, location.1),
        Direction::Up => (location.0, location.1.wrapping_sub(1)),
        Direction::Down => (location.0, location.1 + 1),
    };

    if (0..grid.len_x).contains(&next_x) && (0..grid.len_y).contains(&next_y) {
        Some((next_x, next_y))
    } else {
        None
    }
}

fn follow_the_light(
    grid: &Grid,
    start_location: (usize, usize),
    start_direction: Direction,
) -> u32 {
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

        let (current_position, current_direction) = &beam;
        if let Some(next_position) = next_position(grid, current_position, current_direction) {
            queue.extend(
                next_directions(grid, &next_position, current_direction)
                    .into_iter()
                    .map(|d| (next_position, d)),
            )
        }
    }

    cache.into_iter().map(|x| x.0).collect::<HashSet<_>>().len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let result = follow_the_light(&grid, (0, 0), Direction::Right);

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
        .map(|(x, y, d)| follow_the_light(&grid, (x, y), d))
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
