advent_of_code::solution!(18);

use advent_of_code::maneatingape::point::*;

type Direction = u8;

struct Dig {
    n: i32,
    direction: Direction,
    color: u32,
}

fn parse_data(input: &str) -> Vec<Dig> {
    input
        .lines()
        .map(|line| {
            let mut fields = line.split_ascii_whitespace();

            let direction = fields.next().unwrap().as_bytes()[0];

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
    L: Fn(&Dig) -> i32,
{
    let mut trench = 0;
    let mut polygon = vec![Point::new(0, 0)];
    for dig in data {
        let n = get_length(dig);
        let next_location = polygon[polygon.len() - 1] + Point::from(get_direction(dig)) * n;
        trench += n as u64;
        polygon.push(next_location);
    }

    // Shoelace formula
    let area_twice = polygon
        .windows(2)
        .map(|w| (w[0].y + w[1].y) as i64 * (w[0].x - w[1].x) as i64)
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
            0 => b'R',
            1 => b'D',
            2 => b'L',
            3 => b'U',
            _ => unreachable!(),
        },
        |dig| (dig.color >> 4) as i32,
    );

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
