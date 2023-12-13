advent_of_code::solution!(12);

use regex::Regex;

#[derive(Debug)]
struct State {
    data: Vec<u8>,
    unknown_data: Vec<usize>,
    instructions: Vec<u32>,
}

fn parse_data(input: &str) -> Vec<State> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();

            let data = left.bytes().collect();
            let unknown_data = left
                .as_bytes()
                .iter()
                .enumerate()
                .filter(|(_, v)| v == &&b'?')
                .map(|(i, _)| i)
                .collect();
            let instructions = right.split(',').map(|x| x.parse().unwrap()).collect();

            State {
                data,
                unknown_data,
                instructions,
            }
        })
        .collect()
}

fn recursion(
    data: &mut Vec<u8>,
    regex: &Regex,
    unknown_data: &[usize],
    unknown_data_i: usize,
) -> u64 {
    let mut result = 0;

    if unknown_data_i == unknown_data.len() {
        return 1;
    }

    let i = unknown_data[unknown_data_i];

    if data[i] == b'?' {
        data[i] = b'.';
        if regex.is_match(std::str::from_utf8(data).unwrap()) {
            result += recursion(data, regex, unknown_data, unknown_data_i + 1)
        }

        data[i] = b'#';
        if regex.is_match(std::str::from_utf8(data).unwrap()) {
            result += recursion(data, regex, unknown_data, unknown_data_i + 1)
        }

        data[i] = b'?';
    }

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let states = parse_data(input);

    let mut result = 0;
    for state in states {
        let mut data = state.data;
        let unknown_data = state.unknown_data;
        let instructions = state.instructions;

        let inner_regex = instructions
            .into_iter()
            .map(|i| (0..i).map(|_| "[#|\\?]").collect::<Vec<_>>().join(""))
            .collect::<Vec<_>>()
            .join(r"[\.|\?]+");
        let regex = format!(r"^[\.|\?]*{inner_regex}[\.|\?]*$");
        let regex = Regex::new(&regex).unwrap();

        let options = recursion(&mut data, &regex, &unknown_data, 0);

        result += options;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let states = parse_data(input);

    let mut new_states = vec![];
    for state in states {
        let new_state_data = Vec::from_iter(
            state
                .data
                .iter()
                .chain(std::iter::once(&b'?'))
                .copied()
                .cycle()
                .take(state.data.len() * 5 + 4),
        );

        let new_state_unknown_data = new_state_data
            .iter()
            .enumerate()
            .filter(|(_, v)| v == &&b'?')
            .map(|(i, _)| i)
            .collect();

        let new_state_instructions = Vec::from_iter(
            state
                .instructions
                .iter()
                .copied()
                .cycle()
                .take(state.instructions.len() * 5),
        );

        let new_state = State {
            data: new_state_data,
            unknown_data: new_state_unknown_data,
            instructions: new_state_instructions,
        };

        new_states.push(new_state);
    }

    let states = new_states;

    let mut result = 0;
    for state in states {
        let mut data = state.data;
        let unknown_data = state.unknown_data;
        let instructions = state.instructions;

        let inner_regex = instructions
            .into_iter()
            .map(|i| (0..i).map(|_| "[#|\\?]").collect::<Vec<_>>().join(""))
            .collect::<Vec<_>>()
            .join(r"[\.|\?]+");
        let regex = format!(r"^[\.|\?]*{inner_regex}[\.|\?]*$");
        let regex = Regex::new(&regex).unwrap();

        let options = recursion(&mut data, &regex, &unknown_data, 0);

        result += options;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
