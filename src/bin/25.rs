advent_of_code::solution!(25);

use std::collections::HashMap;

fn parse_data(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| (&line[..3], line[5..].split(' ').collect()))
        .collect()
}

struct KargersAlgorithm<'a> {
    nr_edges: usize,
    nr_vertices: usize,
    edges: &'a [(usize, usize)],
}

impl<'a> KargersAlgorithm<'a> {
    pub fn new(edges: &'a [(usize, usize)], nr_vertices: usize) -> Self {
        Self {
            nr_edges: edges.len(),
            nr_vertices,
            edges,
        }
    }

    fn find(&self, parent: &mut [usize], node: usize) -> usize {
        KargersAlgorithm::find_internal(parent, node)
    }

    // find with path compression technique
    fn find_internal(parent: &mut [usize], node: usize) -> usize {
        if parent[node] != node {
            parent[node] = KargersAlgorithm::find_internal(parent, parent[node]);
        }

        parent[node]
    }

    fn union(&self, parent: &mut [usize], rank: &mut [usize], subset_1: usize, subset_2: usize) {
        match rank[subset_1].cmp(&rank[subset_2]) {
            std::cmp::Ordering::Less => parent[subset_1] = subset_2,
            std::cmp::Ordering::Greater => parent[subset_2] = subset_1,
            std::cmp::Ordering::Equal => {
                parent[subset_2] = subset_1;
                rank[subset_1] += 1
            }
        }
    }

    pub fn min_cut(&self) -> (u32, Vec<usize>) {
        let mut vertices = self.nr_vertices;
        let mut parent = (0..vertices).collect::<Vec<_>>();
        let mut rank = vec![0; vertices];

        let mut shuffled_edges_indices = (0..self.nr_edges).collect::<Vec<_>>();
        fastrand::shuffle(&mut shuffled_edges_indices);
        let mut shuffled_edges_indices_iter = shuffled_edges_indices.into_iter();

        while vertices > 2 {
            let i = shuffled_edges_indices_iter.next().unwrap();

            let subset_1 = self.find(&mut parent, self.edges[i].0);
            let subset_2 = self.find(&mut parent, self.edges[i].1);

            if subset_1 == subset_2 {
                continue;
            }

            vertices -= 1;

            self.union(&mut parent, &mut rank, subset_1, subset_2)
        }

        let cut_edges = self
            .edges
            .iter()
            .map(|e| (self.find(&mut parent, e.0), self.find(&mut parent, e.1)))
            .filter(|(subset_1, subset_2)| subset_1 != subset_2)
            .count() as u32;

        (cut_edges, parent)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let edges = data
        .into_iter()
        .flat_map(|(from, to_list)| to_list.into_iter().map(move |to| (from, to)))
        .collect::<Vec<_>>();

    let vertices_map = edges
        .iter()
        .flat_map(|e| [e.0, e.1])
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<HashMap<_, _>>();

    let edges = edges
        .into_iter()
        .map(|e| (vertices_map[e.0], vertices_map[e.1]))
        .collect::<Vec<_>>();

    let kargers_algorithm_subsets = loop {
        let (min_cut, subsets) = KargersAlgorithm::new(&edges, vertices_map.len()).min_cut();
        if min_cut == 3 {
            break subsets;
        }
    };

    let left_group_count = kargers_algorithm_subsets
        .iter()
        .filter(|x| x != &&kargers_algorithm_subsets[0])
        .count();

    let right_group_count = kargers_algorithm_subsets.len() - left_group_count;

    let result = (left_group_count * right_group_count) as u32;

    Some(result)
}

pub fn part_two(_input: &str) -> Option<String> {
    // "Thank you Eric for another wonderful year of AoC!"
    Some(String::from("⭐️⭐️"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("⭐️⭐️")));
    }
}
