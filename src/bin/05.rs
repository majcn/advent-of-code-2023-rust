advent_of_code::solution!(5);

use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;

type Seed = u64;

#[derive(Clone, Copy)]
struct MapPart {
    min_source: u64,
    max_source: u64,
    min_target: u64,
    max_target: u64,
}

impl MapPart {
    pub fn split_at_target(&self, target_n: u64) -> (Self, Self) {
        let n_left = target_n - self.min_target - 1;
        let n_right = self.max_target - target_n;

        let left = Self {
            min_source: self.min_source,
            max_source: self.min_source + n_left,
            min_target: self.min_target,
            max_target: self.min_target + n_left,
        };

        let right = Self {
            min_source: self.min_source + n_left + 1,
            max_source: self.min_source + n_left + 1 + n_right,
            min_target: self.min_target + n_left + 1,
            max_target: self.min_target + n_left + 1 + n_right,
        };

        (left, right)
    }
}

fn parse_data(input: &str) -> (Vec<Seed>, Vec<Vec<MapPart>>) {
    let mut parts = input.split("\n\n");

    let seeds = parts.next().unwrap().iter_unsigned().collect();

    let maps = parts
        .map(|part| {
            part.iter_unsigned()
                .chunk::<3>()
                .map(|[min_target, min_source, n]| MapPart {
                    min_source,
                    max_source: min_source + n - 1,
                    min_target,
                    max_target: min_target + n - 1,
                })
                .collect()
        })
        .collect();

    (seeds, maps)
}

fn init_big_mapper(first_map: &[MapPart]) -> Vec<MapPart> {
    let mut big_mapper = Vec::with_capacity(first_map.len() * 2);

    let mut first_map = first_map.iter().collect::<Vec<_>>();
    first_map.sort_unstable_by_key(|x| x.min_source);

    let first_el = first_map.first().unwrap();
    if first_el.min_source > 0 {
        big_mapper.push(MapPart {
            min_source: 0,
            max_source: first_el.min_source - 1,
            min_target: 0,
            max_target: first_el.min_source - 1,
        });
    }
    big_mapper.push(**first_el);

    for w in first_map.windows(2) {
        let prev = &w[0];
        let current = &w[1];

        let space_between = current.min_source - prev.max_source;
        if space_between > 1 {
            big_mapper.push(MapPart {
                min_source: prev.max_source + 1,
                max_source: current.min_source - 1,
                min_target: prev.max_source + 1,
                max_target: current.min_source - 1,
            });
        }
        big_mapper.push(**current)
    }

    let last_el = first_map.last().unwrap();
    big_mapper.push(MapPart {
        min_source: last_el.max_source + 1,
        max_source: u64::MAX,
        min_target: last_el.max_source + 1,
        max_target: u64::MAX,
    });

    big_mapper
}

fn apply_mapper(big_mapper: &mut Vec<MapPart>, mapper: &[MapPart]) {
    for mm in mapper.iter() {
        let split_point = mm.min_source;
        let i = big_mapper
            .iter()
            .position(|r| r.min_target <= split_point && split_point <= r.max_target)
            .unwrap();

        if split_point > big_mapper[i].min_target {
            let (left, right) = big_mapper[i].split_at_target(split_point);
            big_mapper.push(left);
            big_mapper.push(right);
            big_mapper.swap_remove(i);
        }

        let split_point = mm.max_source + 1;
        let i = big_mapper
            .iter()
            .position(|r| r.min_target <= split_point && split_point <= r.max_target)
            .unwrap();

        if split_point > big_mapper[i].min_target {
            let (left, right) = big_mapper[i].split_at_target(split_point);
            big_mapper.push(left);
            big_mapper.push(right);
            big_mapper.swap_remove(i);
        }
    }

    for r in big_mapper.iter_mut() {
        let my_map = mapper
            .iter()
            .find(|mm| r.min_target >= mm.min_source && r.max_target <= mm.max_source);

        if let Some(my_map) = my_map {
            r.min_target = r.min_target + my_map.min_target - my_map.min_source;
            r.max_target = r.max_target + my_map.max_target - my_map.max_source;
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = parse_data(input);

    let mut big_mapper = init_big_mapper(maps.first().unwrap());
    maps.iter()
        .skip(1)
        .for_each(|m| apply_mapper(&mut big_mapper, m));

    let result = seeds
        .into_iter()
        .map(|x| {
            big_mapper
                .iter()
                .find(|r| r.min_source <= x && x <= r.max_source)
                .map(|r| r.min_target + x - r.min_source)
                .unwrap()
        })
        .min()
        .unwrap();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, maps) = parse_data(input);

    let mut big_mapper = init_big_mapper(maps.first().unwrap());
    maps.iter()
        .skip(1)
        .for_each(|m| apply_mapper(&mut big_mapper, m));

    big_mapper.sort_unstable_by_key(|r| r.min_target);

    let seed_ranges = seeds
        .chunks(2)
        .map(|c| (c[0], c[0] + c[1]))
        .collect::<Vec<_>>();

    let result = big_mapper
        .into_iter()
        .filter_map(|r| {
            seed_ranges
                .iter()
                .filter(|seed_range| r.max_source >= seed_range.0)
                .filter(|seed_range| r.max_source <= seed_range.1)
                .map(|seed_range| u64::max(r.min_source, seed_range.0))
                .map(|x| x + r.min_target - r.min_source)
                .next()
        })
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
