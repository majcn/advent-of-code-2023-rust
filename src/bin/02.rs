advent_of_code::solution!(2);

use advent_of_code::maneatingape::parse::*;

struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<&str> for Set {
    fn from(item: &str) -> Self {
        let mut result = Set::default();

        item.split(", ")
            .filter_map(|x| x.split_once(' '))
            .for_each(|(n, color)| match color {
                "red" => result.red = n.unsigned(),
                "green" => result.green = n.unsigned(),
                "blue" => result.blue = n.unsigned(),
                _ => unreachable!(),
            });

        result
    }
}

impl From<&str> for Game {
    fn from(item: &str) -> Self {
        let (game_id, game_data) = item.split_once(": ").unwrap();

        let id = game_id.unsigned();
        let sets = game_data.split("; ").map(Set::from).collect();

        Game { id, sets }
    }
}

fn parse_data(input: &str) -> Vec<Game> {
    input.lines().map(Game::from).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data
        .into_iter()
        .filter(|game| {
            !game
                .sets
                .iter()
                .any(|set| set.red > 12 || set.green > 13 || set.blue > 14)
        })
        .map(|game| game.id)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data
        .into_iter()
        .map(|game| Set {
            red: game.sets.iter().map(|x| x.red).max().unwrap(),
            green: game.sets.iter().map(|x| x.green).max().unwrap(),
            blue: game.sets.iter().map(|x| x.blue).max().unwrap(),
        })
        .map(|set| set.red * set.green * set.blue)
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
