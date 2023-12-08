advent_of_code::solution!(3);

use advent_of_code::util::point::Point;

use regex::Regex;

use std::collections::HashMap;
use std::ops::Add;

#[derive(Default)]
struct Number {
    value: u32,
    locations: Vec<Point>,
}

type Symbols = HashMap<Point, char>;

fn parse_data(input: &str) -> (Vec<Number>, Symbols) {
    let mut numbers = vec![];
    let mut symbols = Symbols::new();

    let re_numbers = Regex::new(r"\d+").unwrap();

    for (y, line) in input.lines().enumerate() {
        for m in re_numbers.find_iter(line) {
            let value = m.as_str().parse().unwrap();
            let locations = (m.start()..m.end())
                .map(|x| Point::new(x as i32, y as i32))
                .collect();

            numbers.push(Number { value, locations });
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
    const NEIGHBORS: [Point; 8] = [
        Point::new(-1, -1),
        Point::new(-1, 0),
        Point::new(-1, 1),
        Point::new(0, -1),
        Point::new(0, 1),
        Point::new(1, -1),
        Point::new(1, 0),
        Point::new(1, 1),
    ];

    for number_location in number.locations.iter() {
        for n in NEIGHBORS {
            let new_location = number_location.add(n);

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

    let mut symbol_values: HashMap<Point, Vec<u32>> = HashMap::new();

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
