advent_of_code::solution!(8);

use std::collections::HashMap;

type MyLocation = [char; 3];

struct State {
    instructions: Vec<char>,
    graph: HashMap<MyLocation, (MyLocation, MyLocation)>,
}

fn parse_data(input: &str) -> State {
    let (instructions_str, graph_str) = input.split_once("\n\n").unwrap();
    let instructions = instructions_str.chars().collect();

    let mut graph = HashMap::new();
    for line in graph_str.lines() {
        let key = line[..3].chars().collect::<Vec<_>>().try_into().unwrap();
        let left = line[7..10].chars().collect::<Vec<_>>().try_into().unwrap();
        let right = line[12..15].chars().collect::<Vec<_>>().try_into().unwrap();

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
    loop {
        for instruction in state.instructions.iter() {
            location = match instruction {
                'L' => state.graph[&location].0,
                _ => state.graph[&location].1,
            };
            steps += 1;

            if end_predicate(&location) {
                return steps;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let state = parse_data(input);

    let location = state
        .graph
        .keys()
        .find(|loc| loc == &&['A', 'A', 'A'])
        .unwrap();

    let result = part_x(&state, location, |loc| loc == &['Z', 'Z', 'Z']);

    Some(result)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

pub fn part_two(input: &str) -> Option<u64> {
    let state = parse_data(input);

    let locations = state
        .graph
        .keys()
        .filter(|loc| loc[2] == 'A')
        .collect::<Vec<_>>();

    let result = locations
        .into_iter()
        .map(|loc| part_x(&state, loc, |loc| loc[2] == 'Z'))
        .fold(1, lcm);

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
