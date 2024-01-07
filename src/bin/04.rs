advent_of_code::solution!(4);

use advent_of_code::maneatingape::parse::*;

struct Card {
    my_numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

fn parse_data(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (winning_numbers_str, my_numbers_str) = line.split_once('|').unwrap();
            let winning_numbers = winning_numbers_str.iter_unsigned().skip(1).collect();
            let my_numbers = my_numbers_str.iter_unsigned().collect();

            Card {
                my_numbers,
                winning_numbers,
            }
        })
        .collect()
}

fn part_x(data: &[Card]) -> Vec<usize> {
    data.iter()
        .map(|card| {
            card.my_numbers
                .iter()
                .filter(|x| card.winning_numbers.contains(x))
                .count()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = part_x(&data)
        .into_iter()
        .filter(|x| x > &0)
        .map(|x| 1 << (x - 1))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let mut result = vec![1; data.len()];
    for (card_id, matching_numbers) in part_x(&data).into_iter().enumerate() {
        for i in 1..=matching_numbers {
            result[card_id + i] += result[card_id];
        }
    }
    let result = result.iter().sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
