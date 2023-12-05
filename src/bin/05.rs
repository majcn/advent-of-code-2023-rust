advent_of_code::solution!(5);

type Seed = u64;

struct MapPart {
    min_source: u64,
    max_source: u64,
    min_target: u64,
    max_target: u64,
}

struct Map(Vec<MapPart>);

impl Map {
    pub fn map(&self, s: u64) -> u64 {
        for map_part in &self.0 {
            if s >= map_part.min_source && s <= map_part.max_source {
                return s - map_part.min_source + map_part.min_target;
            }
        }

        s
    }

    pub fn map_reverse(&self, s: u64) -> u64 {
        for map_part in &self.0 {
            if s >= map_part.min_target && s <= map_part.max_target {
                return s - map_part.min_target + map_part.min_source;
            }
        }

        s
    }
}

fn parse_data(input: &str) -> (Vec<Seed>, Vec<Map>) {
    let mut parts = input.split("\n\n");

    let seeds = parts.next().unwrap()[7..]
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut maps = vec![];
    for part in parts {
        let mut part_result = vec![];
        for line in part.lines().skip(1) {
            let tmp = line
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>();

            part_result.push(MapPart {
                min_source: tmp[1],
                max_source: tmp[1] + tmp[2] - 1,
                min_target: tmp[0],
                max_target: tmp[0] + tmp[2] - 1,
            })
        }
        maps.push(Map(part_result));
    }

    (seeds, maps)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = parse_data(input);

    let result = seeds
        .into_iter()
        .map(|location| maps.iter().fold(location, |acc, map| map.map(acc)))
        .min()
        .unwrap();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, maps) = parse_data(input);

    let mut seed_ranges = Vec::with_capacity(seeds.len() / 2);
    for i in (0..seeds.len()).step_by(2) {
        let start = seeds[i];
        let end = seeds[i] + seeds[i + 1];

        seed_ranges.push(start..end)
    }

    let result = (0..u64::MAX)
        .map(|location| {
            (
                location,
                maps.iter()
                    .rev()
                    .fold(location, |acc, map| map.map_reverse(acc)),
            )
        })
        .filter(|(_, seed)| seed_ranges.iter().any(|x| x.contains(seed)))
        .map(|(location, _)| location)
        .next()
        .unwrap();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
