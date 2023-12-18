advent_of_code::solution!(17);

use advent_of_code::util::list::Array2D;

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

struct Grid {
    data: Array2D<u32>,
    len_x: usize,
    len_y: usize,
}

fn parse_data(input: &str) -> Grid {
    let len_x = input.lines().next().unwrap().len();
    let len_y = input.lines().count();

    let mut data = Array2D::new(len_x);
    for line in input.lines() {
        data.add_line(line.bytes().map(|x| (x - b'0') as u32));
    }

    Grid { data, len_x, len_y }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Element {
    x: usize,
    y: usize,
    direction: Direction,
    direction_count: u8,
}

#[derive(PartialEq, Eq)]
struct QueueElement {
    element: Element,
    cost: u32,
}

impl QueueElement {
    fn new(element: Element, cost: u32) -> Self {
        Self { element, cost }
    }
}

impl PartialOrd for QueueElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn get_neighbors_part_1(grid: &Grid, element: &Element) -> Vec<Element> {
    let Element {
        x,
        y,
        direction,
        direction_count,
    } = *element;

    let mut result = Vec::with_capacity(4);

    if x > 0
        && direction != Direction::Right
        && !(direction_count >= 3 && direction == Direction::Left)
    {
        let direction_count = match direction {
            Direction::Left => direction_count + 1,
            _ => 1,
        };

        result.push(Element {
            x: x - 1,
            y,
            direction: Direction::Left,
            direction_count,
        });
    }

    if x < grid.len_x - 1
        && direction != Direction::Left
        && !(direction_count >= 3 && direction == Direction::Right)
    {
        let direction_count = match direction {
            Direction::Right => direction_count + 1,
            _ => 1,
        };

        result.push(Element {
            x: x + 1,
            y,
            direction: Direction::Right,
            direction_count,
        });
    }

    if y > 0
        && direction != Direction::Down
        && !(direction_count >= 3 && direction == Direction::Up)
    {
        let direction_count = match direction {
            Direction::Up => direction_count + 1,
            _ => 1,
        };

        result.push(Element {
            x,
            y: y - 1,
            direction: Direction::Up,
            direction_count,
        });
    }

    if y < grid.len_y - 1
        && direction != Direction::Up
        && !(direction_count >= 3 && direction == Direction::Down)
    {
        let direction_count = match direction {
            Direction::Down => direction_count + 1,
            _ => 1,
        };

        result.push(Element {
            x,
            y: y + 1,
            direction: Direction::Down,
            direction_count,
        });
    }

    result
}

fn get_neighbors_part_2(grid: &Grid, element: &Element) -> Vec<Element> {
    let Element {
        x,
        y,
        direction,
        direction_count,
    } = *element;

    let mut result = Vec::with_capacity(4);

    if x > 0
        && direction != Direction::Right
        && !(direction_count < 4 && direction != Direction::Left)
        && !(direction_count >= 10 && direction == Direction::Left)
    {
        let direction_count = match direction {
            Direction::Left => direction_count + 1,
            _ => 1,
        };

        result.push(Element {
            x: x - 1,
            y,
            direction: Direction::Left,
            direction_count,
        });
    }

    if x < grid.len_x - 1
        && direction != Direction::Left
        && !(direction_count < 4 && direction != Direction::Right)
        && !(direction_count >= 10 && direction == Direction::Right)
    {
        let direction_count = match direction {
            Direction::Right => direction_count + 1,
            _ => 1,
        };

        result.push(Element {
            x: x + 1,
            y,
            direction: Direction::Right,
            direction_count,
        });
    }

    if y > 0
        && direction != Direction::Down
        && !(direction_count < 4 && direction != Direction::Up)
        && !(direction_count >= 10 && direction == Direction::Up)
    {
        let direction_count = match direction {
            Direction::Up => direction_count + 1,
            _ => 1,
        };

        result.push(Element {
            x,
            y: y - 1,
            direction: Direction::Up,
            direction_count,
        });
    }

    if y < grid.len_y - 1
        && direction != Direction::Up
        && !(direction_count < 4 && direction != Direction::Down)
        && !(direction_count >= 10 && direction == Direction::Down)
    {
        let direction_count = match direction {
            Direction::Down => direction_count + 1,
            _ => 1,
        };

        result.push(Element {
            x,
            y: y + 1,
            direction: Direction::Down,
            direction_count,
        });
    }

    result
}

fn find_path_cost<F>(grid: &Grid, get_neighbors: F) -> u32
where
    F: Fn(&Grid, &Element) -> Vec<Element>,
{
    let root_element_right = Element {
        x: 0,
        y: 0,
        direction: Direction::Right,
        direction_count: 0,
    };

    let root_element_down = Element {
        x: 0,
        y: 0,
        direction: Direction::Down,
        direction_count: 0,
    };

    #[inline]
    fn is_goal(grid: &Grid, element: &Element) -> bool {
        element.x == grid.len_x - 1 && element.y == grid.len_y - 1
    }

    let mut visited = HashSet::new();

    let mut dist = HashMap::new();
    dist.insert(root_element_right, 0);
    dist.insert(root_element_down, 0);

    let mut queue = BinaryHeap::new();
    queue.push(QueueElement::new(root_element_right, 0));
    queue.push(QueueElement::new(root_element_down, 0));

    while let Some(queue_element) = queue.pop() {
        let element = &queue_element.element;

        if is_goal(grid, element) {
            return queue_element.cost;
        }

        if visited.contains(element) {
            continue;
        }

        visited.insert(*element);

        for new_element in get_neighbors(grid, element) {
            let dist_element = dist.get(element).unwrap();
            let dist_new_element = dist.get(&new_element).unwrap_or(&u32::MAX);

            let dist_to_new_element = dist_element + grid.data[(new_element.x, new_element.y)];
            if dist_to_new_element < *dist_new_element {
                dist.insert(new_element, dist_to_new_element);
                queue.push(QueueElement {
                    element: new_element,
                    cost: dist_to_new_element,
                });
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
