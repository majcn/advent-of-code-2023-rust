advent_of_code::solution!(22);

use std::collections::HashMap;
use std::collections::HashSet;

use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;

type BrickId = usize;

struct Point {
    x: u32,
    y: u32,
    z: u32,
}

struct Brick {
    id: BrickId,
    p1: Point,
    p2: Point,
}

fn parse_data(input: &str) -> Vec<Brick> {
    input
        .iter_unsigned::<u32>()
        .chunk::<6>()
        .enumerate()
        .map(|(i, [x1, y1, z1, x2, y2, z2])| Brick {
            id: i,
            p1: Point {
                x: x1,
                y: y1,
                z: z1,
            },
            p2: Point {
                x: x2,
                y: y2,
                z: z2,
            },
        })
        .collect()
}

fn part_x(mut bricks: Vec<Brick>) -> Vec<HashSet<BrickId>> {
    bricks.sort_unstable_by_key(|b| b.p1.z);
    for (i, b) in bricks.iter_mut().enumerate() {
        b.id = i;
    }

    let mut result = Vec::with_capacity(bricks.len());

    let does_intersect_xy = |brick1: &&Brick, brick2: &&Brick| {
        brick1.p1.x <= brick2.p2.x
            && brick1.p2.x >= brick2.p1.x
            && brick1.p1.y <= brick2.p2.y
            && brick1.p2.y >= brick2.p1.y
    };

    let mut max_z = HashMap::new();
    let mut new_bricks: Vec<Brick> = Vec::with_capacity(bricks.len());
    for brick in bricks {
        let mut best_z = 0;
        for x in brick.p1.x..=brick.p2.x {
            for y in brick.p1.y..=brick.p2.y {
                if let Some(v) = max_z.get(&(x, y)) {
                    best_z = u32::max(best_z, *v);
                }
            }
        }

        for x in brick.p1.x..=brick.p2.x {
            for y in brick.p1.y..=brick.p2.y {
                max_z.insert((x, y), brick.p2.z - brick.p1.z + best_z + 1);
            }
        }

        let supporting_bricks = new_bricks
            .iter()
            .filter(|b| b.p2.z == best_z)
            .filter(|b| does_intersect_xy(&&brick, b))
            .map(|b| b.id)
            .collect::<HashSet<_>>();

        result.push(supporting_bricks);

        new_bricks.push(Brick {
            id: brick.id,
            p1: Point {
                x: brick.p1.x,
                y: brick.p1.y,
                z: best_z + 1,
            },
            p2: Point {
                x: brick.p2.x,
                y: brick.p2.y,
                z: brick.p2.z - brick.p1.z + best_z + 1,
            },
        });
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = part_x(parse_data(input));

    let result = (0..graph.len())
        .filter(|i| {
            !graph
                .iter()
                .any(|ss| ss.len() == 1 && ss.iter().next().unwrap() == i)
        })
        .count() as u32;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let graph = part_x(parse_data(input));

    let result = (0..graph.len())
        .map(|start_i| {
            let mut destroyed = HashSet::from([start_i]);
            let mut changes = true;

            while changes {
                changes = false;
                for (i, ss) in graph.iter().enumerate() {
                    if ss.is_empty() {
                        continue;
                    }

                    if destroyed.contains(&i) {
                        continue;
                    }

                    if destroyed.is_superset(ss) {
                        destroyed.insert(i);
                        changes = true;
                    }
                }
            }

            destroyed
        })
        .map(|d| d.len() as u32 - 1)
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
