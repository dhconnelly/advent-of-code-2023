use heapless::{
    binary_heap::{BinaryHeap, Min},
    FnvIndexMap, Vec,
};
use libc_print::std_name::*;

type Pt = (u8, u8);
type Grid = Vec<Vec<u8, 256>, 256>;
type MinQueue<T> = BinaryHeap<T, Min, 32768>;
type Map<K, V> = FnvIndexMap<K, V, 32768>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    None,
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

    fn next(self) -> impl Iterator<Item = Dir> {
        match self {
            Dir::None => [Dir::Up, Dir::Right, Dir::Down].into_iter(),
            Dir::Right => [Dir::Up, Dir::Right, Dir::Down].into_iter(),
            Dir::Up => [Dir::Left, Dir::Up, Dir::Right].into_iter(),
            Dir::Left => [Dir::Up, Dir::Left, Dir::Down].into_iter(),
            Dir::Down => [Dir::Left, Dir::Down, Dir::Right].into_iter(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Step {
    pt: Pt,
    dir: Dir,
    steps: u8,
}

impl Step {
    fn go(&self, dir: Dir, grid: &Grid) -> Option<Step> {
        let nbr = dir.apply(self.pt, grid)?;
        if dir == self.dir && self.steps < 3 {
            Some(Step { pt: nbr, dir, steps: self.steps + 1 })
        } else if dir != self.dir {
            Some(Step { pt: nbr, dir, steps: 1 })
        } else {
            None
        }
    }
}

fn neighbors(grid: &Grid, from: &Step) -> Vec<Step, 4> {
    from.dir.next().flat_map(|dir| from.go(dir, grid)).collect()
}

static mut COSTS: Map<Step, u64> = Map::new();
fn init_costs() {
    unsafe {
        COSTS.clear();
    }
}
fn set_cost(step: Step, cost: u64) {
    unsafe {
        COSTS.insert(step, cost).unwrap();
    }
}
fn get_cost(step: &Step) -> u64 {
    unsafe { *COSTS.get(step).unwrap_or(&u64::MAX) }
}

static mut Q: MinQueue<(u64, Step)> = MinQueue::new();
fn init_q() {
    unsafe {
        Q.clear();
    }
}
fn push_q(step: Step, cost: u64) {
    unsafe {
        Q.push((cost, step)).unwrap();
    }
}
fn pop_q() -> Option<(u64, Step)> {
    unsafe { Q.pop() }
}

fn min_path(grid: &Grid, start: Pt, end: Pt) -> Option<u64> {
    init_q();
    init_costs();
    let start = Step { pt: start, dir: Dir::None, steps: 0 };
    push_q(start, 0);
    while let Some((cost, step @ Step { pt, .. })) = pop_q() {
        if pt == end {
            return Some(cost);
        }
        for nbr_step in neighbors(grid, &step) {
            let nbr_cost = cost + grid[nbr_step.pt.0 as usize][nbr_step.pt.1 as usize] as u64;
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
    min_path(&grid, start, end).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
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
    }

    #[test]
    #[ignore]
    fn test_real() {
        let input = include_str!("../inputs/day17.txt");
        assert_eq!(part1(input), 0);
    }
}
