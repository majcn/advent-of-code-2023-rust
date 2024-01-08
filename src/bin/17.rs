advent_of_code::solution!(17);

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::heap::*;
use advent_of_code::maneatingape::point::*;

type Direction = u8;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Element {
    location: Point,
    direction: Direction,
    direction_count: u8,
}

fn parse_data(input: &str) -> Grid<u8> {
    let mut grid = Grid::parse(input);
    for elem in grid.bytes.iter_mut() {
        *elem -= b'0';
    }
    grid
}

fn get_neighbors_part_1(grid: &Grid<u8>, element: Element) -> Vec<Element> {
    let Element {
        location,
        direction,
        direction_count,
    } = element;

    let mut result = Vec::with_capacity(4);

    if location.x > 0 && direction != b'R' && !(direction_count >= 3 && direction == b'L') {
        let direction_count = match direction {
            b'L' => direction_count + 1,
            _ => 1,
        };

        result.push(Element {
            location: location + LEFT,
            direction: b'L',
            direction_count,
        });
    }

    if location.x < grid.width - 1
        && direction != b'L'
        && !(direction_count >= 3 && direction == b'R')
    {
        let direction_count = match direction {
            b'R' => direction_count + 1,
            _ => 1,
        };

        result.push(Element {
            location: location + RIGHT,
            direction: b'R',
            direction_count,
        });
    }

    if location.y > 0 && direction != b'D' && !(direction_count >= 3 && direction == b'U') {
        let direction_count = match direction {
            b'U' => direction_count + 1,
            _ => 1,
        };

        result.push(Element {
            location: location + UP,
            direction: b'U',
            direction_count,
        });
    }

    if location.y < grid.height - 1
        && direction != b'U'
        && !(direction_count >= 3 && direction == b'D')
    {
        let direction_count = match direction {
            b'D' => direction_count + 1,
            _ => 1,
        };

        result.push(Element {
            location: location + DOWN,
            direction: b'D',
            direction_count,
        });
    }

    result
}

fn get_neighbors_part_2(grid: &Grid<u8>, element: Element) -> Vec<Element> {
    let Element {
        location,
        direction,
        direction_count,
    } = element;

    let mut result = Vec::with_capacity(4);

    if location.x > 0
        && direction != b'R'
        && !(direction_count < 4 && direction != b'L')
        && !(direction_count >= 10 && direction == b'L')
    {
        let direction_count = match direction {
            b'L' => direction_count + 1,
            _ => 1,
        };

        result.push(Element {
            location: location + LEFT,
            direction: b'L',
            direction_count,
        });
    }

    if location.x < grid.width - 1
        && direction != b'L'
        && !(direction_count < 4 && direction != b'R')
        && !(direction_count >= 10 && direction == b'R')
    {
        let direction_count = match direction {
            b'R' => direction_count + 1,
            _ => 1,
        };

        result.push(Element {
            location: location + RIGHT,
            direction: b'R',
            direction_count,
        });
    }

    if location.y > 0
        && direction != b'D'
        && !(direction_count < 4 && direction != b'U')
        && !(direction_count >= 10 && direction == b'U')
    {
        let direction_count = match direction {
            b'U' => direction_count + 1,
            _ => 1,
        };

        result.push(Element {
            location: location + UP,
            direction: b'U',
            direction_count,
        });
    }

    if location.y < grid.height - 1
        && direction != b'U'
        && !(direction_count < 4 && direction != b'D')
        && !(direction_count >= 10 && direction == b'D')
    {
        let direction_count = match direction {
            b'D' => direction_count + 1,
            _ => 1,
        };

        result.push(Element {
            location: location + DOWN,
            direction: b'D',
            direction_count,
        });
    }

    result
}

fn find_path_cost<F>(grid: &Grid<u8>, get_neighbors: F) -> u32
where
    F: Fn(&Grid<u8>, Element) -> Vec<Element>,
{
    let root_element_right = Element {
        location: Point::new(0, 0),
        direction: b'R',
        direction_count: 0,
    };

    let root_element_down = Element {
        location: Point::new(0, 0),
        direction: b'D',
        direction_count: 0,
    };

    fn is_goal(grid: &Grid<u8>, element: &Element) -> bool {
        element.location.x == grid.width - 1 && element.location.y == grid.height - 1
    }

    let mut visited = FastSet::new();

    let mut dist = FastMap::new();
    dist.insert(root_element_right, 0);
    dist.insert(root_element_down, 0);

    let mut queue = MinHeap::new();
    queue.push(0, root_element_right);
    queue.push(0, root_element_down);

    while let Some(queue_element) = queue.pop() {
        let element = queue_element.1;

        if is_goal(grid, &element) {
            return queue_element.0;
        }

        if visited.contains(&element) {
            continue;
        }

        visited.insert(element);

        for new_element in get_neighbors(grid, element) {
            let dist_element = dist.get(&element).unwrap();
            let dist_new_element = dist.get(&new_element).copied().unwrap_or(u32::MAX);

            let dist_to_new_element = dist_element + grid[new_element.location] as u32;
            if dist_to_new_element < dist_new_element {
                dist.insert(new_element, dist_to_new_element);
                queue.push(dist_to_new_element, new_element);
            }
        }
    }

    unreachable!()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let result = find_path_cost(&grid, get_neighbors_part_1);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    let result = find_path_cost(&grid, get_neighbors_part_2);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
