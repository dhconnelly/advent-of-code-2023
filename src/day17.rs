use heapless::{
    binary_heap::{BinaryHeap, Min},
    FnvIndexMap, Vec,
};
use libc_print::std_name::*;

type Pt = (u8, u8);
type Grid = Vec<Vec<u8, 256>, 256>;
type MinQueue<T> = BinaryHeap<T, Min, 65536>;
type Costs = Vec<Vec<[u64; 4], 256>, 256>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn apply(self, (row, col): Pt, grid: &Grid) -> Option<Pt> {
        match self {
            Dir::Up if row > 0 => Some((row - 1, col)),
            Dir::Down if row < grid.len() as u8 - 1 => Some((row + 1, col)),
            Dir::Left if col > 0 => Some((row, col - 1)),
            Dir::Right if col < grid[0].len() as u8 - 1 => Some((row, col + 1)),
            _ => None,
        }
    }

    fn turns(self) -> [Dir; 2] {
        match self {
            Dir::Right => [Dir::Up, Dir::Down],
            Dir::Up => [Dir::Left, Dir::Right],
            Dir::Left => [Dir::Up, Dir::Down],
            Dir::Down => [Dir::Left, Dir::Right],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Step {
    pt: Pt,
    dir: Dir,
}

impl Step {
    fn go(self, dir: Dir, grid: &Grid) -> Option<(u64, Step)> {
        dir.apply(self.pt, grid)
            .map(|pt @ (row, col)| (grid[row as usize][col as usize] as u64, Step { pt, dir }))
    }

    fn advance(self, grid: &Grid) -> Option<(u64, Step)> {
        self.go(self.dir, grid)
    }
}

fn neighbors2(grid: &Grid, step: Step) -> Vec<(u64, Step), 6> {
    let mut nbrs = Vec::new();
    for dir in step.dir.turns() {
        if let Some(next) = step.go(dir, grid) {
            nbrs.push(next).unwrap();
        }
    }
    if let Some((one_cost, one_step)) = step.advance(grid) {
        for turn in one_step.dir.turns() {
            if let Some((turn_cost, turn_step)) = one_step.go(turn, grid) {
                nbrs.push((one_cost + turn_cost, turn_step)).unwrap();
            }
        }
        if let Some((two_cost, two_step)) = one_step.advance(grid) {
            for turn in two_step.dir.turns() {
                if let Some((turn_cost, turn_step)) = two_step.go(turn, grid) {
                    nbrs.push((one_cost + two_cost + turn_cost, turn_step)).unwrap();
                }
            }
        }
    }
    nbrs
}

static mut Q: MinQueue<(u64, Step)> = MinQueue::new();
fn init_q() {
    unsafe {
        Q.clear();
    }
}
fn pop_q() -> Option<(u64, Step)> {
    unsafe { Q.pop() }
}
fn push_q(step: Step, cost: u64) {
    unsafe {
        Q.push((cost, step)).unwrap();
    }
}

static mut COSTS: Costs = Costs::new();
fn init_costs() {
    unsafe {
        COSTS.clear();
        for i in 0..COSTS.capacity() {
            COSTS.push(Vec::new()).unwrap();
            for j in 0..COSTS[i].capacity() {
                COSTS[i].push([0; 4]).unwrap();
                for k in 0..4 {
                    COSTS[i][j][k] = u64::MAX;
                }
            }
        }
    }
}
fn get_cost(step: &Step) -> u64 {
    unsafe { COSTS[step.pt.0 as usize][step.pt.1 as usize][step.dir as usize] }
}
fn set_cost(step: Step, cost: u64) {
    unsafe {
        COSTS[step.pt.0 as usize][step.pt.1 as usize][step.dir as usize] = cost;
    }
}

fn min_path2(grid: &Grid, start: Pt, end: Pt) -> Option<u64> {
    init_q();
    init_costs();

    // TODO: simplify wrt all the type casts
    let start = Step { pt: start, dir: Dir::Right };
    for (cost, step) in [start.go(Dir::Right, grid).unwrap(), start.go(Dir::Down, grid).unwrap()] {
        push_q(step, cost);
        set_cost(step, cost);
    }
    while let Some((cost, step)) = pop_q() {
        if step.pt == end {
            return Some(cost);
        }
        for (nbr_step_cost, nbr_step) in neighbors2(grid, step) {
            let nbr_cost = cost + nbr_step_cost;
            if nbr_cost < get_cost(&nbr_step) {
                push_q(nbr_step, nbr_cost);
                set_cost(nbr_step, nbr_cost);
            }
        }
    }
    None
}

fn parse(input: &str) -> Grid {
    input.lines().map(|line| line.bytes().map(|b| b - b'0').collect()).collect()
}

pub fn part1(input: &str) -> u64 {
    let grid = parse(input);
    let start = (0, 0);
    let end = (grid.len() as u8 - 1, grid[0].len() as u8 - 1);
    min_path2(&grid, start, end).unwrap()
}

pub fn part2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        assert_eq!(part1(input), 102);

        // real
        let input = include_str!("../inputs/day17.txt");
        assert_eq!(part1(input), 1263);
    }
}
