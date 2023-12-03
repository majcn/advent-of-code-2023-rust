advent_of_code::solution!(3);

use std::collections::HashMap;
use std::ops::Add;

use advent_of_code::util::point::Point;

#[derive(Debug, Default)]
struct Number {
    value: u32,
    locations: Vec<Point>,
}

type Symbol = char;
type Symbols = HashMap<Point, Symbol>;

fn parse_data(input: &str) -> (Vec<Number>, Symbols) {
    let mut numbers = vec![];
    let mut symbols = Symbols::new();

    for (y, line) in input.lines().enumerate() {
        let mut current_number = Number::default();
        for (x, v) in line.chars().enumerate() {
            if v.is_ascii_digit() {
                current_number.value = current_number.value * 10 + v.to_digit(10).unwrap();
                current_number
                    .locations
                    .push(Point::new(x as i32, y as i32))
            } else if current_number.value > 0 {
                numbers.push(current_number);
                current_number = Number::default();
            }
        }
        if current_number.value > 0 {
            numbers.push(current_number);
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

pub fn part_one(input: &str) -> Option<u32> {
    let (numbers, symbols) = parse_data(input);

    let neighbors = [
        Point::new(-1, -1),
        Point::new(-1, 0),
        Point::new(-1, 1),
        Point::new(0, -1),
        Point::new(0, 1),
        Point::new(1, -1),
        Point::new(1, 0),
        Point::new(1, 1),
    ];

    let is_valid = |number: &Number| {
        for number_location in number.locations.iter() {
            for n in neighbors {
                let new_location = number_location.add(n);

                if symbols.contains_key(&new_location) {
                    return true;
                }
            }
        }

        false
    };

    let result = numbers.into_iter().filter(is_valid).map(|x| x.value).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (numbers, symbols) = parse_data(input);

    let neighbors = [
        Point::new(-1, -1),
        Point::new(-1, 0),
        Point::new(-1, 1),
        Point::new(0, -1),
        Point::new(0, 1),
        Point::new(1, -1),
        Point::new(1, 0),
        Point::new(1, 1),
    ];

    let find_symbol = |number: &Number| {
        for number_location in number.locations.iter() {
            for n in neighbors {
                let new_location = number_location.add(n);

                if let Some(x) = symbols.get(&new_location) {
                    if x == &'*' {
                        return Some(new_location);
                    }
                }
            }
        }

        None
    };

    let mut symbol_values: HashMap<Point, Vec<u32>> = HashMap::new();

    for number in numbers {
        if let Some(symbol) = find_symbol(&number) {
            symbol_values.entry(symbol).or_default().push(number.value);
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
