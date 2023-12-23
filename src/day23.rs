use heapless::{Deque, FnvIndexMap, FnvIndexSet, Vec};
use libc_print::std_name::*;

type Pt = (i16, i16);
type Tile = u8;
type WeightedGraph = Vec<Vec<(u16, i16), 4>, 4096>;
type Set<T> = FnvIndexSet<T, 4096>;
type Map<K, V> = FnvIndexMap<K, V, 4096>;

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
}

const DIRS: [Pt; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn go(grid: &Grid, (r, c): Pt, (dr, dc): Pt) -> Option<Pt> {
    let nbr @ (r2, c2) = (r + dr, c + dc);
    if r2 >= 0 && r2 < grid.height as i16 && c2 >= 0 && c2 < grid.height as i16 {
        Some(nbr)
    } else {
        None
    }
}

fn adjacent(grid: &Grid, pt: Pt) -> Vec<Pt, 4> {
    DIRS.iter().flat_map(|dir| go(grid, pt, *dir)).collect()
}

fn orthog(grid: &Grid, pt: Pt, (dr, dc): Pt) -> Vec<Pt, 2> {
    let dirs = if dr == 0 { [(-1, 0), (1, 0)] } else { [(0, -1), (0, 1)] };
    dirs.into_iter().flat_map(|dir| go(grid, pt, dir)).collect()
}

fn start(grid: &Grid) -> Pt {
    let start = (0, 1);
    assert!(grid.get(start) == b'.');
    start
}

fn end(grid: &Grid) -> Pt {
    let end = (grid.height as i16 - 1, grid.width as i16 - 2);
    assert!(grid.get(end) == b'.');
    end
}

fn can_pass(tile: u8) -> bool {
    matches!(tile, b'.' | b'^' | b'v' | b'<' | b'>')
}

fn nbrs_slopes(grid: &Grid, pt @ (r, c): Pt) -> Vec<Pt, 4> {
    match grid.get(pt) {
        b'.' => adjacent(grid, pt).into_iter().filter(|pt| can_pass(grid.get(*pt))).collect(),
        b'^' => Vec::from_slice(&[(r - 1, c)]).unwrap(),
        b'v' => Vec::from_slice(&[(r + 1, c)]).unwrap(),
        b'<' => Vec::from_slice(&[(r, c - 1)]).unwrap(),
        b'>' => Vec::from_slice(&[(r, c + 1)]).unwrap(),
        _ => Vec::new(),
    }
}

fn nbrs(grid: &Grid, pt: Pt) -> Vec<Pt, 4> {
    match grid.get(pt) {
        b'#' => Vec::new(),
        _ => adjacent(grid, pt).into_iter().filter(|pt| can_pass(grid.get(*pt))).collect(),
    }
}

fn is_intersection(grid: &Grid, pt: Pt) -> bool {
    nbrs(grid, pt).len() != 2
}

fn find_neighbors(grid: &Grid, start: Pt) -> Vec<(Pt, i16), 4> {
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
            if is_intersection(grid, nbr) {
                edges.push((nbr, dist + 1)).unwrap();
                continue;
            }
            v.insert(nbr).unwrap();
            q.push_back((nbr, dist + 1)).unwrap();
        }
    }
    edges
}

// won't fit on the stack :(
static mut GRAPH: WeightedGraph = WeightedGraph::new();
static mut VPT: Set<Pt> = Set::new();
static mut VID: Set<u16> = Set::new();

fn init() {
    unsafe {
        GRAPH.clear();
        VPT.clear();
        VID.clear();
    }
}

fn build_graph(grid: &Grid, graph: &mut WeightedGraph, v: &mut Set<Pt>) -> (u16, u16) {
    // to find neighbors, go in each direction until reaching an intersection
    fn get_id(pt: Pt, indices: &mut Map<Pt, u16>) -> u16 {
        if let Some(id) = indices.get(&pt) {
            *id
        } else {
            let id = indices.len() as u16;
            indices.insert(pt, id).unwrap();
            id
        }
    }
    let start = start(grid);
    let end = end(grid);
    let mut indices = Map::new();
    let mut q: Vec<Pt, 2048> = Vec::new();
    q.push(start).unwrap();
    while let Some(cur) = q.pop() {
        let mut edges = Vec::new();
        for (nbr, dist) in find_neighbors(grid, cur) {
            edges.push((get_id(nbr, &mut indices), dist)).unwrap();
            if v.contains(&nbr) {
                continue;
            }
            v.insert(nbr).unwrap();
            q.push(nbr).unwrap();
        }
        let id = get_id(cur, &mut indices);
        if graph.len() as u16 <= id {
            graph.resize_default(id as usize + 1).unwrap();
        }
        graph[id as usize] = edges;
    }
    (get_id(start, &mut indices), get_id(end, &mut indices))
}

fn graph_longest_path(
    graph: &WeightedGraph,
    cur: u16,
    end: u16,
    v: &mut Set<u16>,
) -> Option<usize> {
    let nbrs = &graph[cur as usize];
    let dists = nbrs.iter().flat_map(|(nbr, dist_to_nbr)| {
        if *nbr == end {
            Some(*dist_to_nbr as usize)
        } else if v.contains(nbr) {
            None
        } else {
            v.insert(*nbr).unwrap();
            let dist_from_nbr = graph_longest_path(graph, *nbr, end, v);
            v.remove(nbr);
            dist_from_nbr.map(|d| *dist_to_nbr as usize + d)
        }
    });
    dists.max()
}

fn longest_path(grid: &Grid) -> usize {
    let graph = unsafe { &mut GRAPH };
    let (start, end) = build_graph(&grid, graph, unsafe { &mut VPT });
    graph_longest_path(graph, start, end, unsafe { &mut VID }).unwrap()
}

fn parse(input: &str) -> Grid {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    Grid { tiles: input.as_bytes(), width, height }
}

pub fn part1(input: &str) -> usize {
    init();
    0
}

pub fn part2(input: &str) -> usize {
    init();
    let grid = parse(input);
    longest_path(&grid)
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
    fn test() {
        //assert_eq!(part1(TEST_INPUT), 94);
        assert_eq!(part2(TEST_INPUT), 154);

        //assert_eq!(part1(REAL_INPUT), 2042);
        assert_eq!(part2(REAL_INPUT), 0);
    }
}
