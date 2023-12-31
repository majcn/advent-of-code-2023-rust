advent_of_code::solution!(20);

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, Copy)]
enum Pulse {
    Low,
    High,
}

enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast(Broadcast),
}

impl Module {
    fn id(&self) -> usize {
        match self {
            Module::FlipFlop(m) => m.id,
            Module::Conjunction(m) => m.id,
            Module::Broadcast(m) => m.id,
        }
    }

    fn has_output(&self, id: &usize) -> bool {
        match self {
            Module::FlipFlop(m) => m.outputs.contains(id),
            Module::Conjunction(m) => m.outputs.contains(id),
            Module::Broadcast(m) => m.outputs.contains(id),
        }
    }

    fn trigger(&mut self, input_module: usize, input_pulse: Pulse) -> Vec<(usize, usize, Pulse)> {
        match self {
            Module::FlipFlop(m) => m.trigger(input_module, input_pulse),
            Module::Conjunction(m) => m.trigger(input_module, input_pulse),
            Module::Broadcast(m) => m.trigger(input_module, input_pulse),
        }
    }
}

struct FlipFlop {
    id: usize,
    state: bool,
    outputs: Vec<usize>,
}

impl FlipFlop {
    fn trigger(&mut self, _input_module: usize, input_pulse: Pulse) -> Vec<(usize, usize, Pulse)> {
        match input_pulse {
            Pulse::Low => {
                self.state = !self.state;
                let output_pulse = if self.state { Pulse::High } else { Pulse::Low };
                self.outputs
                    .iter()
                    .map(|&o| (self.id, o, output_pulse))
                    .collect()
            }
            Pulse::High => vec![],
        }
    }
}

struct Conjunction {
    id: usize,
    state: u64,
    outputs: Vec<usize>,
}

impl Conjunction {
    fn trigger(&mut self, input_module: usize, input_pulse: Pulse) -> Vec<(usize, usize, Pulse)> {
        match input_pulse {
            Pulse::Low => self.state &= !(1 << input_module),
            Pulse::High => self.state |= 1 << input_module,
        };

        let output_pulse = match self.state {
            u64::MAX => Pulse::Low,
            _ => Pulse::High,
        };

        self.outputs
            .iter()
            .map(|&o| (self.id, o, output_pulse))
            .collect()
    }
}

struct Broadcast {
    id: usize,
    outputs: Vec<usize>,
}

impl Broadcast {
    fn trigger(&mut self, _input_module: usize, input_pulse: Pulse) -> Vec<(usize, usize, Pulse)> {
        self.outputs
            .iter()
            .map(|&o| (self.id, o, input_pulse))
            .collect()
    }
}

const SPECIAL_OUTPUT_ID: usize = 10000;

fn parse_data(input: &str) -> (Vec<Module>, usize) {
    let mut inputs: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut outputs: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut ids = HashMap::new();

    ids.insert("rx", SPECIAL_OUTPUT_ID);

    for (i, line) in input.lines().enumerate() {
        let (left_str, right_str) = line.split_once(" -> ").unwrap();
        let module_name = match left_str {
            "broadcaster" => "broadcaster",
            _ => &left_str[1..],
        };

        ids.insert(module_name, i);
        for output in right_str.split(", ") {
            match inputs.entry(output) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    o.get_mut().push(module_name);
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(vec![module_name]);
                }
            };

            match outputs.entry(module_name) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    o.get_mut().push(output);
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(vec![output]);
                }
            };
        }
    }

    let modules = input
        .lines()
        .map(|line| {
            let (left_str, _) = line.split_once(" -> ").unwrap();
            let module_name = match left_str {
                "broadcaster" => "broadcaster",
                _ => &left_str[1..],
            };

            let id = ids[module_name];

            let outputs = outputs
                .get(module_name)
                .unwrap()
                .iter()
                .map(|x| ids[x])
                .collect();

            match left_str.as_bytes()[0] {
                b'%' => {
                    let state = false;
                    Module::FlipFlop(FlipFlop { id, state, outputs })
                }
                b'&' => {
                    let state = inputs
                        .get(module_name)
                        .unwrap()
                        .iter()
                        .map(|x| ids[x])
                        .fold(u64::MAX, |acc, x| acc & !(1 << x));
                    Module::Conjunction(Conjunction { id, state, outputs })
                }
                _ => Module::Broadcast(Broadcast { id, outputs }),
            }
        })
        .collect();

    let broadcaster = ids["broadcaster"];

    (modules, broadcaster)
}

fn press_button<F>(modules: &mut [Module], broadcaster: usize, mut visitor: F)
where
    F: FnMut(usize, usize, Pulse),
{
    let mut queue = VecDeque::new();
    queue.push_back((broadcaster, broadcaster, Pulse::Low));
    while let Some((input_module, output_module, input_pulse)) = queue.pop_front() {
        visitor(input_module, output_module, input_pulse);

        if output_module == SPECIAL_OUTPUT_ID {
            continue;
        }

        queue.extend(modules[output_module].trigger(input_module, input_pulse))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut modules, broadcaster) = parse_data(input);

    let mut lows = 0;
    let mut highs = 0;

    for _ in 0..1000 {
        press_button(
            &mut modules,
            broadcaster,
            |_, _, input_pulse| match input_pulse {
                Pulse::Low => lows += 1,
                Pulse::High => highs += 1,
            },
        )
    }

    let result = lows * highs;

    Some(result)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut modules, broadcaster) = parse_data(input);

    let module_with_special_output = modules
        .iter()
        .find(|m| m.has_output(&SPECIAL_OUTPUT_ID))
        .map(|m| m.id())
        .unwrap();

    let targets = modules
        .iter()
        .filter(|m| m.has_output(&module_with_special_output))
        .map(|m| m.id())
        .collect::<Vec<_>>();

    let mut found_results = 0;
    let mut results = vec![0; targets.len()];
    let mut i = 0;
    while found_results < targets.len() {
        press_button(&mut modules, broadcaster, |input_module, _, input_pulse| {
            if matches!(input_pulse, Pulse::High) {
                if let Some(target_id) = targets.iter().position(|&x| x == input_module) {
                    if results[target_id] == 0 {
                        results[target_id] = i + 1;
                        found_results += 1;
                    }
                }
            }
        });

        i += 1;
    }

    let result = results.into_iter().reduce(lcm).unwrap();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
