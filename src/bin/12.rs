advent_of_code::solution!(12);

use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::parse::*;

struct State {
    data: Vec<u8>,
    instructions: Vec<u32>,
}

fn parse_data(input: &str) -> Vec<State> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();

            let data = left.bytes().collect();
            let instructions = right.iter_unsigned().collect();

            State { data, instructions }
        })
        .collect()
}

#[derive(Default)]
struct MemoPartX<'a> {
    cache: FastMap<(&'a [u8], &'a [u32]), u64>,
}

impl<'a> MemoPartX<'a> {
    fn handle_next_operational(&mut self, data: &'a [u8], instructions: &'a [u32]) -> u64 {
        self.part_x(&data[1..], instructions)
    }

    fn handle_next_damaged(
        &mut self,
        data: &'a [u8],
        instructions: &'a [u32],
        group_size: usize,
    ) -> u64 {
        if data.len() < group_size {
            return 0;
        }

        let group_data = &data[..group_size];
        if group_data.iter().any(|x| x == &b'.') {
            return 0;
        }

        if data.len() == group_size {
            return if instructions.len() == 1 { 1 } else { 0 };
        }

        match data[group_size] {
            b'.' | b'?' => self.part_x(&data[(group_size + 1)..], &instructions[1..]),
            b'#' => 0,
            _ => unreachable!(),
        }
    }

    pub fn part_x(&mut self, data: &'a [u8], instructions: &'a [u32]) -> u64 {
        let cache_key = (data, instructions);
        if let Some(cached_value) = self.cache.get(&cache_key) {
            return *cached_value;
        }

        if instructions.is_empty() {
            return if data.iter().any(|x| x == &b'#') {
                0
            } else {
                1
            };
        }

        if data.is_empty() {
            return 0;
        }

        let group_size = instructions[0] as usize;

        let result = match &data[0] {
            b'#' => self.handle_next_damaged(data, instructions, group_size),
            b'.' => self.handle_next_operational(data, instructions),
            b'?' => {
                let damaged = self.handle_next_damaged(data, instructions, group_size);
                let operational = self.handle_next_operational(data, instructions);
                damaged + operational
            }
            _ => unreachable!(),
        };

        self.cache.insert(cache_key, result);

        result
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let states = parse_data(input);

    let result = states
        .into_iter()
        .map(|s| MemoPartX::default().part_x(&s.data, &s.instructions))
        .sum();

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
            instructions: new_state_instructions,
        };

        new_states.push(new_state);
    }

    let result = new_states
        .into_iter()
        .map(|s| MemoPartX::default().part_x(&s.data, &s.instructions))
        .sum();

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
