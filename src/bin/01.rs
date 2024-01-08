advent_of_code::solution!(1);

fn parse_data(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part_x<F>(data: Vec<&str>, digit_map: F) -> u32
where
    F: Fn(&[u8]) -> Option<u32>,
{
    let mut result = 0;
    for line in data.into_iter() {
        let mut first = 0;
        let mut last = 0;

        for i in 0..line.len() {
            if let Some(digit) = digit_map(line[i..].as_bytes()) {
                if first == 0 {
                    first = digit;
                    last = digit;
                } else {
                    last = digit;
                }
            }
        }

        result += first * 10 + last;
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let digit_map = |s: &[u8]| match s[0] {
        b'1' => Some(1),
        b'2' => Some(2),
        b'3' => Some(3),
        b'4' => Some(4),
        b'5' => Some(5),
        b'6' => Some(6),
        b'7' => Some(7),
        b'8' => Some(8),
        b'9' => Some(9),
        _ => None,
    };

    let result = part_x(data, digit_map);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let digit_map = |s: &[u8]| match s {
        [b'1', ..] | [b'o', b'n', b'e', ..] => Some(1),
        [b'2', ..] | [b't', b'w', b'o', ..] => Some(2),
        [b'3', ..] | [b't', b'h', b'r', b'e', b'e', ..] => Some(3),
        [b'4', ..] | [b'f', b'o', b'u', b'r', ..] => Some(4),
        [b'5', ..] | [b'f', b'i', b'v', b'e', ..] => Some(5),
        [b'6', ..] | [b's', b'i', b'x', ..] => Some(6),
        [b'7', ..] | [b's', b'e', b'v', b'e', b'n', ..] => Some(7),
        [b'8', ..] | [b'e', b'i', b'g', b'h', b't', ..] => Some(8),
        [b'9', ..] | [b'n', b'i', b'n', b'e', ..] => Some(9),
        _ => None,
    };

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
