advent_of_code::solution!(8);

use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::math::*;

type MyLocation = [u8; 3];

struct State {
    instructions: Vec<u8>,
    graph: FastMap<MyLocation, (MyLocation, MyLocation)>,
}

fn parse_data(input: &str) -> State {
    let (instructions_str, graph_str) = input.split_once("\n\n").unwrap();
    let instructions = instructions_str.bytes().collect();

    let mut graph = FastMap::new();
    for line in graph_str.lines() {
        let key = line[..3].bytes().collect::<Vec<_>>().try_into().unwrap();
        let left = line[7..10].bytes().collect::<Vec<_>>().try_into().unwrap();
        let right = line[12..15].bytes().collect::<Vec<_>>().try_into().unwrap();

        graph.insert(key, (left, right));
    }

    State {
        instructions,
        graph,
    }
}

fn part_x<F>(state: &State, current_location: &MyLocation, end_predicate: F) -> u64
where
    F: Fn(&MyLocation) -> bool,
{
    let mut steps = 0;
    let mut location = *current_location;

    for instruction in state.instructions.iter().cycle() {
        location = match instruction {
            b'L' => state.graph[&location].0,
            b'R' => state.graph[&location].1,
            _ => unreachable!(),
        };
        steps += 1;

        if end_predicate(&location) {
            return steps;
        }
    }

    unreachable!()
}

pub fn part_one(input: &str) -> Option<u64> {
    let state = parse_data(input);

    let location = state
        .graph
        .keys()
        .find(|loc| loc == &&[b'A', b'A', b'A'])
        .unwrap();

    let result = part_x(&state, location, |loc| loc == &[b'Z', b'Z', b'Z']);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let state = parse_data(input);

    let locations = state
        .graph
        .keys()
        .filter(|loc| loc[2] == b'A')
        .collect::<Vec<_>>();

    let result = locations
        .into_iter()
        .map(|loc| part_x(&state, loc, |loc| loc[2] == b'Z'))
        .fold(1, IntegerMathOps::lcm);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
