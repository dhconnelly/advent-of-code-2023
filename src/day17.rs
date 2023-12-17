use heapless::{
    binary_heap::{BinaryHeap, Min},
    Vec,
};

type Pt = (u8, u8);
type Grid = Vec<Vec<u8, 256>, 256>;
type MinQueue<T> = BinaryHeap<T, Min, 16384>;
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
    fn advance(self, grid: &Grid) -> Option<(u64, Step)> {
        self.dir.apply(self.pt, grid).map(|pt @ (row, col)| {
            (grid[row as usize][col as usize] as u64, Step { pt, dir: self.dir })
        })
    }
}

fn neighbors(grid: &Grid, mut cur: Step, min: u8, max: u8) -> Vec<(u64, Step), 16> {
    let mut nbrs = Vec::new();
    let mut cost = 0;
    for _ in 0..min {
        if let Some((step_cost, step)) = cur.advance(grid) {
            cost += step_cost;
            cur = step;
        } else {
            return Vec::new();
        };
    }
    for _ in 0..max - min + 1 {
        for dir in cur.dir.turns() {
            nbrs.push((cost, Step { pt: cur.pt, dir })).unwrap();
        }
        if let Some((step_cost, step)) = cur.advance(grid) {
            cost += step_cost;
            cur = step;
        } else {
            break;
        };
    }
    nbrs
}

// too big for the stack :(
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

fn min_path(grid: &Grid, start: Pt, end: Pt, min_steps: u8, max_steps: u8) -> Option<u64> {
    init_costs();
    let mut q = MinQueue::new();
    for dir in [Dir::Right, Dir::Down] {
        let step = Step { pt: start, dir };
        q.push((0, step)).unwrap();
        set_cost(step, 0);
    }
    while let Some((cost, step)) = q.pop() {
        if step.pt == end {
            return Some(cost);
        }
        for (nbr_step_cost, nbr_step) in neighbors(grid, step, min_steps, max_steps) {
            let nbr_cost = cost + nbr_step_cost;
            if nbr_cost < get_cost(&nbr_step) {
                q.push((nbr_cost, nbr_step)).unwrap();
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
    min_path(&grid, start, end, 1, 3).unwrap()
}

pub fn part2(input: &str) -> u64 {
    let grid = parse(input);
    let start = (0, 0);
    let end = (grid.len() as u8 - 1, grid[0].len() as u8 - 1);
    min_path(&grid, start, end, 4, 10).unwrap()
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
        assert_eq!(part2(input), 94);

        // real
        let input = include_str!("../inputs/day17.txt");
        assert_eq!(part1(input), 1263);
        assert_eq!(part2(input), 1411);
    }
}
