advent_of_code::solution!(19);

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Copy)]
enum ConditionExpression {
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

#[derive(Clone, Copy)]
struct Condition {
    name: char,
    expression: ConditionExpression,
    value: u32,
}

#[derive(Clone, Copy)]
enum NodeValue {
    Accept,
    Reject,
}

struct TreeNode {
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
    condition: Option<Condition>,
    value: Option<NodeValue>,
}

fn create_node(data: &HashMap<&str, &str>, name: &str) -> TreeNode {
    let name = data.get(name).unwrap_or(&name);

    if name == &"A" {
        return TreeNode {
            condition: None,
            left: None,
            right: None,
            value: Some(NodeValue::Accept),
        };
    }

    if name == &"R" {
        return TreeNode {
            condition: None,
            left: None,
            right: None,
            value: Some(NodeValue::Reject),
        };
    }

    let (part, other) = name.split_once(',').unwrap();
    let (condition, left_name) = part.split_once(':').unwrap();

    let mut condition_chars = condition.chars();
    let condition_char = condition_chars.next().unwrap();
    let condition_op = condition_chars.next().unwrap();
    let condition_value = condition_chars.collect::<String>().parse().unwrap();
    let condition = Condition {
        name: condition_char,
        expression: match condition_op {
            '>' => ConditionExpression::Greater,
            '<' => ConditionExpression::Less,
            _ => unreachable!(),
        },
        value: condition_value,
    };

    let left_node = create_node(data, left_name);
    let right_node = create_node(data, other);

    TreeNode {
        left: Some(Rc::new(RefCell::new(left_node))),
        right: Some(Rc::new(RefCell::new(right_node))),
        condition: Some(condition),
        value: None,
    }
}

fn parse_data(input: &str) -> (TreeNode, Vec<Vec<u32>>) {
    let (workflows_str, ratings_str) = input.split_once("\n\n").unwrap();
    let workflows = workflows_str
        .lines()
        .map(|line| &line[..line.len() - 1])
        .map(|line| line.split_once('{').unwrap())
        .collect::<HashMap<_, _>>();

    let root = create_node(&workflows, "in");

    let ratings = ratings_str
        .lines()
        .map(|line| &line[1..line.len() - 1])
        .map(|line| {
            line.splitn(4, ",")
                .map(|x| x[2..].parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (root, ratings)
}

struct TreeTraversePath {}

impl TreeTraversePath {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_value(&self, node_ref: &Rc<RefCell<TreeNode>>, rating: &[u32]) -> NodeValue {
        self.traverse_tree(node_ref, rating)
    }

    fn traverse_tree(&self, node_ref: &Rc<RefCell<TreeNode>>, rating: &[u32]) -> NodeValue {
        let node = node_ref.borrow();

        if let Some(v) = &node.value {
            return *v;
        } else {
            let node_condition = node.condition.unwrap();

            let i = match node_condition.name {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => unreachable!(),
            };

            let should_go_left = match node_condition.expression {
                ConditionExpression::Less => rating[i] < node_condition.value,
                ConditionExpression::Greater => rating[i] > node_condition.value,
                _ => unreachable!(),
            };

            if should_go_left {
                return self.traverse_tree(node.left.as_ref().unwrap(), rating);
            } else {
                return self.traverse_tree(node.right.as_ref().unwrap(), rating);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (root_node, ratings) = parse_data(input);

    let root_node_ref = Rc::new(RefCell::new(root_node));

    let result = ratings
        .into_iter()
        .filter(|rating| {
            matches!(
                TreeTraversePath::new().get_value(&root_node_ref, &rating),
                NodeValue::Accept
            )
        })
        .map(|x| x.iter().sum::<u32>())
        .sum();

    Some(result)
}

struct TreeTraversePaths {
    path_conditions: Vec<Vec<Condition>>,
    path_values: Vec<NodeValue>,
}

impl TreeTraversePaths {
    pub fn new() -> Self {
        Self {
            path_conditions: vec![],
            path_values: vec![],
        }
    }

    pub fn calculate_paths(&mut self, node_ref: &Rc<RefCell<TreeNode>>) {
        self.traverse_tree(node_ref, vec![])
    }

    fn traverse_tree(&mut self, node_ref: &Rc<RefCell<TreeNode>>, values: Vec<Condition>) {
        let node = node_ref.borrow();

        if let Some(v) = &node.value {
            self.path_values.push(*v);
            self.path_conditions.push(values);
        } else {
            let node_condition = node.condition.unwrap();

            let mut new_values = Vec::with_capacity(values.len() + 1);
            new_values.extend(values.iter().copied());
            new_values.push(node_condition);
            self.traverse_tree(node.left.as_ref().unwrap(), new_values);

            let mut new_values = Vec::with_capacity(values.len() + 1);
            new_values.extend(values.iter().copied());
            new_values.push(Condition {
                name: node_condition.name,
                expression: match node_condition.expression {
                    ConditionExpression::Less => ConditionExpression::GreaterOrEqual,
                    ConditionExpression::LessOrEqual => ConditionExpression::Greater,
                    ConditionExpression::Greater => ConditionExpression::LessOrEqual,
                    ConditionExpression::GreaterOrEqual => ConditionExpression::Less,
                },
                value: node_condition.value,
            });
            self.traverse_tree(node.right.as_ref().unwrap(), new_values);
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (node, _) = parse_data(input);

    let mut paths = TreeTraversePaths::new();
    paths.calculate_paths(&Rc::new(RefCell::new(node)));

    let accepted_paths = paths
        .path_conditions
        .iter()
        .enumerate()
        .filter(|(i, _)| matches!(paths.path_values[*i], NodeValue::Accept))
        .map(|(_, v)| v)
        .collect::<Vec<_>>();

    let result = accepted_paths
        .into_iter()
        .map(|path| {
            let mut values = [[1, 4000]; 4];

            for c in path {
                let i = match c.name {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    's' => 3,
                    _ => unreachable!(),
                };

                match c.expression {
                    ConditionExpression::Less => values[i][1] = values[i][1].min(c.value - 1),
                    ConditionExpression::LessOrEqual => values[i][1] = values[i][1].min(c.value),
                    ConditionExpression::Greater => values[i][0] = values[i][0].max(c.value + 1),
                    ConditionExpression::GreaterOrEqual => values[i][0] = values[i][0].max(c.value),
                }
            }

            values
                .iter()
                .map(|x| (x[1] - x[0] + 1) as u64)
                .product::<u64>()
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
