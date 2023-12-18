advent_of_code::solution!(18);

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Dig {
    n: u32,
    direction: Direction,
    color: u32,
}

fn parse_data(input: &str) -> Vec<Dig> {
    input
        .lines()
        .map(|line| {
            let mut fields = line.split_ascii_whitespace();

            let direction = match fields.next().unwrap().as_bytes()[0] {
                b'L' => Direction::Left,
                b'R' => Direction::Right,
                b'U' => Direction::Up,
                b'D' => Direction::Down,
                _ => unreachable!(),
            };

            let n = fields.next().unwrap().parse().unwrap();

            let color = u32::from_str_radix(&fields.next().unwrap()[2..8], 16).unwrap();

            Dig {
                n,
                direction,
                color,
            }
        })
        .collect()
}

fn part_x<D, L>(data: &[Dig], get_direction: D, get_length: L) -> u64
where
    D: Fn(&Dig) -> Direction,
    L: Fn(&Dig) -> u32,
{
    let mut trench = 0;
    let mut polygon = vec![(0, 0)];
    for dig in data {
        let x = polygon[polygon.len() - 1].0;
        let y = polygon[polygon.len() - 1].1;

        let n = get_length(dig) as i64;
        let next_location = match get_direction(dig) {
            Direction::Left => (x - n, y),
            Direction::Right => (x + n, y),
            Direction::Up => (x, y - n),
            Direction::Down => (x, y + n),
        };

        trench += n as u64;
        polygon.push(next_location);
    }

    // Shoelace formula
    let area_twice = polygon
        .windows(2)
        .map(|w| (w[0].1 + w[1].1) * (w[0].0 - w[1].0))
        .sum::<i64>()
        .unsigned_abs();

    // Pick's theorem
    let boundaries = trench;
    let interior_points = (area_twice / 2) - boundaries / 2 + 1;

    interior_points + boundaries
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = part_x(&data, |dig| dig.direction, |dig| dig.n);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let result = part_x(
        &data,
        |dig| match dig.color & 0xF {
            2 => Direction::Left,
            0 => Direction::Right,
            3 => Direction::Up,
            1 => Direction::Down,
            _ => unreachable!(),
        },
        |dig| dig.color >> 4,
    );

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
