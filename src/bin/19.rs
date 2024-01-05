advent_of_code::solution!(19);

use advent_of_code::maneatingape::hash::*;

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
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
    condition: Option<Condition>,
    value: Option<NodeValue>,
}

fn create_node(data: &FastMap<&str, &str>, name: &str) -> TreeNode {
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
        left: Some(Box::new(left_node)),
        right: Some(Box::new(right_node)),
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
        .collect::<FastMap<_, _>>();

    let root = create_node(&workflows, "in");

    let ratings = ratings_str
        .lines()
        .map(|line| &line[1..line.len() - 1])
        .map(|line| {
            line.splitn(4, ',')
                .map(|x| x[2..].parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (root, ratings)
}

fn get_final_node_value(node: &TreeNode, rating: &[u32]) -> NodeValue {
    if let Some(v) = &node.value {
        *v
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
            return get_final_node_value(node.left.as_ref().unwrap(), rating);
        } else {
            return get_final_node_value(node.right.as_ref().unwrap(), rating);
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (root_node, ratings) = parse_data(input);

    let result = ratings
        .into_iter()
        .filter(|rating| matches!(get_final_node_value(&root_node, rating), NodeValue::Accept))
        .map(|x| x.iter().sum::<u32>())
        .sum();

    Some(result)
}

fn calculate_paths(
    path_conditions: &mut Vec<Vec<Condition>>,
    path_values: &mut Vec<NodeValue>,
    node: &TreeNode,
    values: Vec<Condition>,
) {
    if let Some(v) = &node.value {
        path_values.push(*v);
        path_conditions.push(values);
    } else {
        let node_condition = node.condition.unwrap();

        let mut new_values = Vec::with_capacity(values.len() + 1);
        new_values.extend(values.iter().copied());
        new_values.push(node_condition);
        calculate_paths(
            path_conditions,
            path_values,
            node.left.as_ref().unwrap(),
            new_values,
        );

        let mut new_values = Vec::with_capacity(values.len() + 1);
        new_values.extend(values);
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
        calculate_paths(
            path_conditions,
            path_values,
            node.right.as_ref().unwrap(),
            new_values,
        );
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (node, _) = parse_data(input);

    let mut path_conditions = vec![];
    let mut path_values = vec![];

    calculate_paths(&mut path_conditions, &mut path_values, &node, vec![]);

    let accepted_paths = path_conditions
        .iter()
        .enumerate()
        .filter(|(i, _)| matches!(path_values[*i], NodeValue::Accept))
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
