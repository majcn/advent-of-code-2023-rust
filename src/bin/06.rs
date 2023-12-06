advent_of_code::solution!(6);

fn parse_data(input: &str) -> (Vec<u64>, Vec<u64>) {
    let (time, distance) = input.split_once('\n').unwrap();
    let time = time[5..]
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let distance = distance[9..]
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    (time, distance)
}

fn part_x(time: u64, distance: u64) -> u64 {
    let mut a = 0;
    let mut b = time / 2;
    loop {
        let c = (a + b) / 2;
        if c * (time - c) > distance && (c - 1) * (time - c + 1) <= distance {
            break c;
        }

        if c * (time - c) <= distance {
            a = c;
        } else {
            b = c;
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (time, distance) = parse_data(input);

    let mut result = 1;
    for i in 0..time.len() {
        let best_time = part_x(time[i], distance[i]);
        result *= time[i] - best_time - best_time + 1;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (time, distance) = parse_data(input);
    let time = time
        .into_iter()
        .fold(String::new(), |acc, x| acc + &x.to_string())
        .parse()
        .unwrap();
    let distance = distance
        .into_iter()
        .fold(String::new(), |acc, x| acc + &x.to_string())
        .parse()
        .unwrap();

    let best_time = part_x(time, distance);
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
