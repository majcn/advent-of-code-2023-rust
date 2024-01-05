advent_of_code::solution!(1);

use advent_of_code::maneatingape::hash::*;

fn parse_data(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part_x(data: Vec<&str>, digit_map: FastMap<&str, u8>) -> u32 {
    let mut result = 0;
    for line in data.into_iter() {
        let mut tmp_result = vec![];

        for i in 0..line.len() {
            let digit = digit_map
                .iter()
                .find(|(key, _)| line[i..].starts_with(*key));

            if let Some((_, value)) = digit {
                tmp_result.push(value)
            }
        }

        let mut tmp_result_iter = tmp_result.into_iter();
        let first = *tmp_result_iter.next().unwrap();
        let last = *tmp_result_iter.next_back().unwrap_or(&first);
        result += first as u32 * 10 + last as u32;
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let digit_map = FastMap::build([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let result = part_x(data, digit_map);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let digit_map = FastMap::build([
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ]);

    let result = part_x(data, digit_map);

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
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
