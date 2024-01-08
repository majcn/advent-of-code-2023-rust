advent_of_code::solution!(13);

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::point::*;

fn parse_data(input: &str) -> Vec<Grid<u8>> {
    input.split("\n\n").map(Grid::parse).collect()
}

fn is_valid_x<const N_P: usize>(mirror: &Grid<u8>, split_point_x: i32) -> bool {
    let mut problems = 0;

    for y in 0..mirror.height {
        let mut i = 0;
        while split_point_x >= i && split_point_x + i + 1 < mirror.width {
            if mirror[Point::new(split_point_x - i, y)]
                != mirror[Point::new(split_point_x + i + 1, y)]
            {
                if problems == N_P {
                    return false;
                }

                problems += 1;
            }
            i += 1;
        }
    }

    problems == N_P
}

fn find_valid_x<const N_P: usize>(mirror: &Grid<u8>) -> Option<i32> {
    (0..mirror.width - 1).find(|&x| is_valid_x::<N_P>(mirror, x))
}

fn is_valid_y<const N_P: usize>(mirror: &Grid<u8>, split_point_y: i32) -> bool {
    let mut problems = 0;

    for x in 0..mirror.width {
        let mut i = 0;
        while split_point_y >= i && split_point_y + i + 1 < mirror.height {
            if mirror[Point::new(x, split_point_y - i)]
                != mirror[Point::new(x, split_point_y + i + 1)]
            {
                if problems == N_P {
                    return false;
                }

                problems += 1;
            }
            i += 1;
        }
    }

    problems == N_P
}

fn find_valid_y<const N_P: usize>(mirror: &Grid<u8>) -> Option<i32> {
    (0..mirror.height - 1).find(|&y| is_valid_y::<N_P>(mirror, y))
}

fn part_x<const N_P: usize>(data: &[Grid<u8>]) -> u32 {
    data.iter()
        .map(|mirror| {
            let valid_x = find_valid_x::<N_P>(mirror);
            if let Some(v) = valid_x {
                v as u32 + 1
            } else {
                let v = find_valid_y::<N_P>(mirror).unwrap();
                (v as u32 + 1) * 100
            }
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = part_x::<0>(&data);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = part_x::<1>(&data);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
