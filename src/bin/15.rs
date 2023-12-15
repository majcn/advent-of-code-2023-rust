advent_of_code::solution!(15);

fn parse_data(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

fn calc_hash(data: &str) -> u32 {
    data.bytes().fold(0, |acc, x| (acc + x as u32) * 17 % 256)
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let result = data.into_iter().map(calc_hash).sum();

    Some(result)
}

enum Operation {
    Set(u32),
    Remove,
}

struct Holder<'a> {
    label: &'a str,
    label_loc: usize,
    operation: Operation,
}

impl<'a> Holder<'a> {
    pub fn new(label: &'a str, operation: Operation) -> Self {
        Holder {
            label,
            label_loc: calc_hash(label) as usize,
            operation,
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_data(input)
        .into_iter()
        .map(|x| {
            if let Some(label) = x.strip_suffix('-') {
                Holder::new(label, Operation::Remove)
            } else {
                let (label, value) = x.split_once('=').unwrap();
                Holder::new(label, Operation::Set(value.parse().unwrap()))
            }
        })
        .collect::<Vec<_>>();

    let mut my_boxes: Vec<Vec<(&str, u32)>> =
        std::iter::repeat(Vec::new()).take(256).collect::<Vec<_>>();

    for d in data {
        let my_box = &mut my_boxes[d.label_loc];

        match d.operation {
            Operation::Set(v) => match my_box.iter().position(|x| x.0 == d.label) {
                None => {
                    my_box.push((d.label, v));
                }
                Some(i) => {
                    my_box.push((d.label, v));
                    my_box.swap_remove(i);
                }
            },
            Operation::Remove => {
                if let Some(i) = my_box.iter().position(|x| x.0 == d.label) {
                    my_box.remove(i);
                }
            }
        }
    }

    let mut result = 0;
    for (box_i, my_box) in my_boxes.into_iter().enumerate() {
        for (slot_i, (_, v)) in my_box.into_iter().enumerate() {
            result += (box_i as u32 + 1) * (slot_i as u32 + 1) * v;
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
