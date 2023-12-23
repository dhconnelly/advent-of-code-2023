use heapless::{Deque, FnvIndexMap, FnvIndexSet, Vec};

type Tile = u8;
type Pt = (i16, i16);
type Set = FnvIndexSet<Pt, 256>;
type WeightedGraph = FnvIndexMap<Pt, Vec<(Pt, i16), 4>, 256>;
type Neighbors = Vec<Pt, 4>;

#[derive(Debug)]
struct Grid<'a> {
    tiles: &'a [u8],
    width: usize,
    height: usize,
}

impl Grid<'_> {
    fn get(&self, (r, c): Pt) -> Tile {
        self.tiles[r as usize * (self.width + 1) + c as usize]
    }

    fn in_range(&self, (r, c): Pt) -> bool {
        r >= 0 && (r as usize) < self.height && c >= 0 && (c as usize) < self.width
    }
}

impl<'a> From<&'a str> for Grid<'a> {
    fn from(input: &'a str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        Grid { tiles: input.as_bytes(), width, height }
    }
}

fn find_neighbors<F: Fn(&Grid, Pt) -> Neighbors>(
    grid: &Grid,
    start: Pt,
    nbrs: &F,
) -> Vec<(Pt, i16), 4> {
    let mut edges: Vec<(Pt, i16), 4> = Vec::new();
    let mut v: FnvIndexSet<Pt, 1024> = FnvIndexSet::new();
    v.insert(start).unwrap();
    let mut q: Deque<(Pt, i16), 1024> = Deque::new();
    q.push_back((start, 0)).unwrap();
    while let Some((cur, dist)) = q.pop_back() {
        for nbr in nbrs(grid, cur) {
            if v.contains(&nbr) {
                continue;
            }
            // the nodes are the intersections
            if nbrs(grid, nbr).len() != 2 {
                edges.push((nbr, dist + 1)).unwrap();
                continue;
            }
            v.insert(nbr).unwrap();
            q.push_back((nbr, dist + 1)).unwrap();
        }
    }
    edges
}

fn build_graph<F: Fn(&Grid, Pt) -> Neighbors>(
    grid: &Grid,
    cur: Pt,
    graph: &mut WeightedGraph,
    v: &mut Set,
    nbrs: &F,
) {
    let mut stack: Vec<Pt, 1024> = Vec::new();
    stack.push(cur).unwrap();
    while let Some(cur) = stack.pop() {
        let mut edges = Vec::new();
        for (nbr, dist) in find_neighbors(grid, cur, nbrs) {
            edges.push((nbr, dist)).unwrap();
            if !v.contains(&nbr) {
                v.insert(nbr).unwrap();
                stack.push(nbr).unwrap();
            }
        }
        graph.insert(cur, edges).unwrap();
    }
}

fn longest_path_in_graph(graph: &WeightedGraph, cur: Pt, end: Pt, v: &mut Set) -> Option<usize> {
    let nbrs = graph.get(&cur).unwrap();
    let dists = nbrs.iter().flat_map(|(nbr, dist_to_nbr)| {
        if *nbr == end {
            Some(*dist_to_nbr as usize)
        } else if v.contains(nbr) {
            None
        } else {
            v.insert(*nbr).unwrap();
            let dist_from_nbr = longest_path_in_graph(graph, *nbr, end, v);
            v.remove(nbr);
            dist_from_nbr.map(|d| *dist_to_nbr as usize + d)
        }
    });
    dists.max()
}

fn longest_path<F: Fn(&Grid, Pt) -> Neighbors + Copy>(grid: &Grid, nbrs: F) -> usize {
    let start = (0, 1);
    let end = (grid.height as i16 - 1, grid.width as i16 - 2);
    let mut graph = WeightedGraph::new();
    build_graph(&grid, start, &mut graph, &mut Set::new(), &nbrs);
    longest_path_in_graph(&graph, start, end, &mut Set::new()).unwrap()
}

fn passable_adjacents(grid: &Grid, (r, c): Pt) -> Neighbors {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(|(dr, dc)| (r + dr, c + dc))
        .filter(|pt| grid.in_range(*pt))
        .filter(|pt| matches!(grid.get(*pt), b'.' | b'^' | b'v' | b'<' | b'>'))
        .collect()
}

pub fn part1(input: &str) -> usize {
    longest_path(&Grid::from(input), |grid, pt @ (r, c)| match grid.get(pt) {
        b'.' => passable_adjacents(grid, pt),
        b'^' => Vec::from_slice(&[(r - 1, c)]).unwrap(),
        b'v' => Vec::from_slice(&[(r + 1, c)]).unwrap(),
        b'<' => Vec::from_slice(&[(r, c - 1)]).unwrap(),
        b'>' => Vec::from_slice(&[(r, c + 1)]).unwrap(),
        _ => Vec::new(),
    })
}

pub fn part2(input: &str) -> usize {
    longest_path(&Grid::from(input), |grid, pt| match grid.get(pt) {
        b'#' => Vec::new(),
        _ => passable_adjacents(grid, pt),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

    const REAL_INPUT: &str = include_str!("../inputs/day23.txt");

    #[test]
    fn test_example() {
        assert_eq!(part1(TEST_INPUT), 94);
        assert_eq!(part2(TEST_INPUT), 154);
    }

    #[test]
    fn test_real() {
        assert_eq!(part1(REAL_INPUT), 2042);
        assert_eq!(part2(REAL_INPUT), 6466);
    }
}
