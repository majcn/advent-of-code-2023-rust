advent_of_code::solution!(7);

use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::parse::*;

struct Hand {
    cards: Vec<char>,
    bid: u32,
}

#[derive(PartialEq, PartialOrd)]
enum HandPower {
    None,
    Pair,
    TwoPairs,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

struct HandState {
    bid: u32,
    power: HandPower,
    values: Vec<u8>,
}

fn parse_data(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let cards = line[..5].chars().collect();
            let bid = (&line[6..]).unsigned();
            Hand { cards, bid }
        })
        .collect()
}

fn calculate_power(hand: &Hand, joker_mode: bool) -> HandPower {
    let mut counter = FastMap::with_capacity(hand.cards.len());

    for x in &hand.cards {
        *counter.entry(x).or_default() += 1;
    }

    let jokers = if joker_mode {
        counter.remove(&'J').unwrap_or(0)
    } else {
        0
    };

    let best_value = counter.values().max().unwrap_or(&0) + jokers;
    match (best_value, counter.len()) {
        (5, _) => HandPower::FiveOfKind,
        (4, _) => HandPower::FourOfKind,
        (3, 2) => HandPower::FullHouse,
        (3, _) => HandPower::ThreeOfKind,
        (2, 3) => HandPower::TwoPairs,
        (2, _) => HandPower::Pair,
        _ => HandPower::None,
    }
}

fn part_x(hands: &[Hand], joker_mode: bool) -> u32 {
    let mut state = hands
        .iter()
        .map(|hand| {
            let bid = hand.bid;
            let power = calculate_power(hand, joker_mode);
            let values = hand
                .cards
                .iter()
                .map(|x| match (x, joker_mode) {
                    ('J', true) => 1,
                    ('2', _) => 2,
                    ('3', _) => 3,
                    ('4', _) => 4,
                    ('5', _) => 5,
                    ('6', _) => 6,
                    ('7', _) => 7,
                    ('8', _) => 8,
                    ('9', _) => 9,
                    ('T', _) => 10,
                    ('J', false) => 11,
                    ('Q', _) => 12,
                    ('K', _) => 13,
                    ('A', _) => 14,
                    _ => unreachable!(),
                })
                .collect();

            HandState { bid, power, values }
        })
        .collect::<Vec<_>>();

    state.sort_unstable_by(|a, b| {
        if a.power == b.power {
            let mut values_iter = a.values.iter().zip(b.values.iter());
            return values_iter
                .find(|(av, bv)| av != bv)
                .map(|(av, bv)| av.cmp(bv))
                .unwrap();
        }

        a.power.partial_cmp(&b.power).unwrap()
    });

    state
        .into_iter()
        .enumerate()
        .map(|(i, s)| (i as u32 + 1) * s.bid)
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let hands = parse_data(input);

    let result = part_x(&hands, false);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands = parse_data(input);

    let result = part_x(&hands, true);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
