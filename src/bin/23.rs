advent_of_code::solution!(23);

use advent_of_code::util::list::Array2D;

use advent_of_code::maneatingape::hash::*;

enum GridValue {
    Path,
    Forest,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight,
}

struct Grid {
    data: Array2D<GridValue>,
    len_x: usize,
    len_y: usize,
    start_location: (usize, usize),
    end_location: (usize, usize),
}

type Location = (usize, usize);
type NodeId = usize;

fn parse_data(input: &str) -> Grid {
    let len_x = input.lines().next().unwrap().len();
    let len_y = input.lines().count();
    let start_location = (1, 0);
    let end_location = (len_x - 2, len_y - 1);

    let mut data = Array2D::new(len_x);
    for line in input.lines() {
        data.add_line(line.as_bytes().iter().map(|x| match x {
            b'.' => GridValue::Path,
            b'#' => GridValue::Forest,
            b'^' => GridValue::SlopeUp,
            b'v' => GridValue::SlopeDown,
            b'<' => GridValue::SlopeLeft,
            b'>' => GridValue::SlopeRight,
            _ => unreachable!(),
        }));
    }

    Grid {
        data,
        len_x,
        len_y,
        start_location,
        end_location,
    }
}

fn generate_graph<F>(grid: &Grid, get_neighbors: F) -> FastMap<(Location, Location), u32>
where
    F: Fn(&Grid, &Location, &Location) -> Vec<Location>,
{
    let mut graph = FastMap::new();

    let mut queue = vec![(grid.start_location, grid.start_location, 0)];
    while let Some((cross_loc, path_loc, init_cost)) = queue.pop() {
        let mut prev_loc = cross_loc;
        let mut loc = path_loc;
        let mut cost = init_cost;
        let mut neighbors;
        loop {
            neighbors = get_neighbors(grid, &prev_loc, &loc);

            if neighbors.len() != 1 {
                break;
            }

            prev_loc = loc;
            loc = neighbors[0];
            cost += 1;
        }

        if !graph.contains_key(&(cross_loc, loc)) && cost > 1 {
            graph.insert((cross_loc, loc), cost);
            queue.extend(neighbors.into_iter().map(|n| (loc, n, 1)));
        }
    }

    graph
}

fn simplify_graph(
    graph: FastMap<(Location, Location), u32>,
    start_location: &Location,
    end_location: &Location,
) -> (Vec<Vec<(NodeId, u32)>>, NodeId, NodeId) {
    let idx_mapper = graph
        .keys()
        .flat_map(|(from, to)| [from, to])
        .collect::<FastSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<FastMap<_, _>>();

    let mut graph_as_vec = vec![vec![]; idx_mapper.len()];
    for ((from, to), cost) in &graph {
        graph_as_vec[idx_mapper[from]].push((idx_mapper[to], *cost));
    }

    (
        graph_as_vec,
        idx_mapper[start_location],
        idx_mapper[end_location],
    )
}

fn find_all_paths<F>(
    graph: &[Vec<(NodeId, u32)>],
    node: NodeId,
    cost: u32,
    dfs_visitor: &mut F,
    visited: &mut [bool],
) where
    F: FnMut(NodeId, u32),
{
    visited[node] = true;

    dfs_visitor(node, cost);

    for &(new_node, new_cost) in &graph[node] {
        if !visited[new_node] {
            find_all_paths(graph, new_node, cost + new_cost, dfs_visitor, visited);
        }
    }

    visited[node] = false;
}

fn part_x<F>(grid: &Grid, get_neighbors: F) -> u32
where
    F: Fn(&Grid, &Location, &Location) -> Vec<Location>,
{
    let graph = generate_graph(grid, get_neighbors);
    let (graph, start, end) = simplify_graph(graph, &grid.start_location, &grid.end_location);

    let mut result = 0;
    let mut dfs_visitor = |node, cost| {
        if node == end {
            result = u32::max(result, cost)
        }
    };

    find_all_paths(
        &graph,
        start,
        0,
        &mut dfs_visitor,
        &mut vec![false; graph.len()],
    );

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    fn get_neighbors(grid: &Grid, prev_loc: &Location, loc: &Location) -> Vec<Location> {
        let options = match grid.data[loc] {
            GridValue::Path => vec![
                (loc.0.wrapping_sub(1), loc.1),
                (loc.0 + 1, loc.1),
                (loc.0, loc.1.wrapping_sub(1)),
                (loc.0, loc.1 + 1),
            ],
            GridValue::SlopeUp => vec![(loc.0, loc.1.wrapping_sub(1))],
            GridValue::SlopeDown => vec![(loc.0, loc.1 + 1)],
            GridValue::SlopeLeft => vec![(loc.0.wrapping_sub(1), loc.1)],
            GridValue::SlopeRight => vec![(loc.0 + 1, loc.1)],
            GridValue::Forest => unreachable!(),
        };

        options
            .into_iter()
            .filter(|o| o != prev_loc)
            .filter(|o| (0..grid.len_x).contains(&o.0) && (0..grid.len_y).contains(&o.1))
            .filter(|o| !matches!(grid.data[o], GridValue::Forest))
            .collect::<Vec<_>>()
    }

    let result = part_x(&grid, get_neighbors);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    fn get_neighbors(grid: &Grid, prev_loc: &Location, loc: &Location) -> Vec<Location> {
        let options = vec![
            (loc.0.wrapping_sub(1), loc.1),
            (loc.0 + 1, loc.1),
            (loc.0, loc.1.wrapping_sub(1)),
            (loc.0, loc.1 + 1),
        ];

        options
            .into_iter()
            .filter(|o| o != prev_loc)
            .filter(|o| (0..grid.len_x).contains(&o.0) && (0..grid.len_y).contains(&o.1))
            .filter(|o| !matches!(grid.data[o], GridValue::Forest))
            .collect::<Vec<_>>()
    }

    let result = part_x(&grid, get_neighbors);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
