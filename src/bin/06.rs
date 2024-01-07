advent_of_code::solution!(6);

use advent_of_code::maneatingape::parse::*;

struct Race {
    time: u64,
    distance: u64,
}

fn parse_data(input: &str) -> Vec<Race> {
    let (time, distance) = input.split_once('\n').unwrap();

    time.iter_unsigned()
        .zip(distance.iter_unsigned())
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn part_x(race: Race) -> u64 {
    let mut a = 0;
    let mut b = race.time / 2;
    loop {
        let c = (a + b) / 2;
        if c * (race.time - c) > race.distance && (c - 1) * (race.time - c + 1) <= race.distance {
            break c;
        }

        if c * (race.time - c) <= race.distance {
            a = c;
        } else {
            b = c;
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = parse_data(input);

    let result = races
        .into_iter()
        .map(|race| (race.time, part_x(race)))
        .map(|(time, best_time)| time - best_time - best_time + 1)
        .product::<u64>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let races = parse_data(input);

    let time = races
        .iter()
        .map(|race| race.time)
        .fold(String::new(), |acc, x| acc + &x.to_string())
        .as_str()
        .unsigned();

    let distance = races
        .iter()
        .map(|race| race.distance)
        .fold(String::new(), |acc, x| acc + &x.to_string())
        .as_str()
        .unsigned();

    let race = Race { time, distance };

    let best_time = part_x(race);
    let result = time - best_time - best_time + 1;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
