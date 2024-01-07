advent_of_code::solution!(9);

use advent_of_code::maneatingape::parse::*;

fn parse_data(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| line.iter_signed().collect())
        .collect()
}

fn part_x(line: Vec<i64>) -> Vec<Vec<i64>> {
    let mut state = vec![];
    state.push(line);
    loop {
        let line = state.last().unwrap();
        let mut part_result = Vec::with_capacity(line.len() - 1);
        for w in line.windows(2) {
            part_result.push(w[1] - w[0]);
        }

        if part_result.iter().all(|x| x == &0) {
            state.push(part_result);
            return state;
        } else {
            state.push(part_result);
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let data = parse_data(input);

    let result = data
        .into_iter()
        .flat_map(part_x)
        .map(|mut x| x.pop().unwrap())
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let data = parse_data(input);

    let result = data
        .into_iter()
        .map(part_x)
        .flat_map(|state| state.into_iter().enumerate())
        .map(|(i, x)| (i, x.into_iter().next().unwrap()))
        .fold(0, |acc, (i, x)| if i % 2 == 0 { acc + x } else { acc - x });

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
