advent_of_code::solution!(23);

use advent_of_code::maneatingape::grid::*;
use advent_of_code::maneatingape::hash::*;
use advent_of_code::maneatingape::point::*;

type NodeId = usize;

fn parse_data(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn generate_graph<F>(
    grid: &Grid<u8>,
    start_location: Point,
    get_neighbors: F,
) -> FastMap<(Point, Point), u32>
where
    F: Fn(&Grid<u8>, Point, Point) -> Vec<Point>,
{
    let mut graph = FastMap::new();

    let mut queue = vec![(start_location, start_location, 0)];
    while let Some((cross_loc, path_loc, init_cost)) = queue.pop() {
        let mut prev_loc = cross_loc;
        let mut loc = path_loc;
        let mut cost = init_cost;
        let mut neighbors;
        loop {
            neighbors = get_neighbors(grid, prev_loc, loc);

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
    graph: FastMap<(Point, Point), u32>,
    start_location: &Point,
    end_location: &Point,
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

fn part_x<F>(grid: &Grid<u8>, get_neighbors: F) -> u32
where
    F: Fn(&Grid<u8>, Point, Point) -> Vec<Point>,
{
    let start_location = Point::new(1, 0);
    let end_location = Point::new(grid.width - 2, grid.height - 1);

    let graph = generate_graph(grid, start_location, get_neighbors);
    let (graph, start, end) = simplify_graph(graph, &start_location, &end_location);

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

    fn get_neighbors(grid: &Grid<u8>, prev_loc: Point, loc: Point) -> Vec<Point> {
        let options_iter = match grid[loc] {
            b'.' => Vec::from(ORTHOGONAL.map(|x| loc + x)),
            b'^' => vec![loc + UP],
            b'v' => vec![loc + DOWN],
            b'<' => vec![loc + LEFT],
            b'>' => vec![loc + RIGHT],
            _ => unreachable!(),
        };

        options_iter
            .into_iter()
            .filter(|&o| o != prev_loc)
            .filter(|&o| grid.contains(o))
            .filter(|&o| grid[o] != b'#')
            .collect::<Vec<_>>()
    }

    let result = part_x(&grid, get_neighbors);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_data(input);

    fn get_neighbors(grid: &Grid<u8>, prev_loc: Point, loc: Point) -> Vec<Point> {
        ORTHOGONAL
            .into_iter()
            .map(|x| loc + x)
            .filter(|&o| o != prev_loc)
            .filter(|&o| grid.contains(o))
            .filter(|&o| grid[o] != b'#')
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
