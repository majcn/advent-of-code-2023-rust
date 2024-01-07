advent_of_code::solution!(3);

use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::parse::*;
use advent_of_code::maneatingape::point::*;

#[derive(Default)]
struct Number {
    value: u32,
    locations: Vec<Point>,
}

type Symbols = FastMap<Point, char>;

fn parse_data(input: &str) -> (Vec<Number>, Symbols) {
    let mut numbers = vec![];
    let mut symbols = Symbols::new();

    for (y, line) in input.lines().enumerate() {
        let mut i = 0;
        while i < line.len() {
            if line[i..].as_bytes()[0].is_ascii_digit() {
                let value = (&line[i..]).unsigned::<u32>();
                let value_len = value.ilog10() as usize + 1;

                let locations = (0..value_len)
                    .map(|x| Point::new((i + x) as i32, y as i32))
                    .collect();

                numbers.push(Number { value, locations });

                i += value_len;
            } else {
                i += 1;
            }
        }
    }

    for (y, line) in input.lines().enumerate() {
        for (x, v) in line.chars().enumerate() {
            if !v.is_ascii_digit() && v != '.' {
                symbols.insert(Point::new(x as i32, y as i32), v);
            }
        }
    }

    (numbers, symbols)
}

fn find_symbol_location(number: &Number, symbols: &Symbols) -> Option<Point> {
    for &number_location in number.locations.iter() {
        for n in DIAGONAL {
            let new_location = number_location + n;

            if symbols.contains_key(&new_location) {
                return Some(new_location);
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let (numbers, symbols) = parse_data(input);

    let result = numbers
        .into_iter()
        .filter(|x| find_symbol_location(x, &symbols).is_some())
        .map(|x| x.value)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (numbers, symbols) = parse_data(input);

    let mut symbol_values: FastMap<Point, Vec<u32>> = FastMap::new();

    for number in numbers {
        if let Some(symbol_location) = find_symbol_location(&number, &symbols) {
            if symbols.get(&symbol_location).unwrap() == &'*' {
                symbol_values
                    .entry(symbol_location)
                    .or_default()
                    .push(number.value);
            }
        }
    }

    let result = symbol_values
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
