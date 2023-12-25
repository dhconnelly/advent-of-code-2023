use heapless::{Deque, FnvIndexMap, FnvIndexSet, Vec};
use libc_print::std_name::*;

type Connections<const N: usize> = FnvIndexSet<u16, N>;
type Graph = Vec<Connections<64>, 2048>;
type ResidualGraph = FnvIndexMap<(u16, u16), i16, 2048>;
type Path = FnvIndexMap<u16, u16, 1024>;
type Keys<'a> = FnvIndexMap<u16, &'a str, 2048>;

fn parse<'a>(input: &'a str, graph: &mut Graph) -> Keys<'a> {
    let mut keys: FnvIndexMap<&str, u16, 2048> = FnvIndexMap::new();
    for line in input.lines() {
        let (src, dsts) = line.split_once(": ").unwrap();
        for wire in core::iter::once(src).chain(dsts.split_whitespace()) {
            if !keys.contains_key(wire) {
                keys.insert(wire, keys.len() as u16).unwrap();
            }
        }
    }
    graph.resize_default(keys.len()).unwrap();
    for line in input.lines() {
        let (src, dsts) = line.split_once(": ").unwrap();
        let src = keys.get(src).unwrap();
        graph[*src as usize].insert(*src).unwrap();
        for dst in dsts.split_whitespace() {
            let dst = keys.get(dst).unwrap();
            graph[*dst as usize].insert(*dst).unwrap();
            graph[*src as usize].insert(*dst).unwrap();
            graph[*dst as usize].insert(*src).unwrap();
        }
    }
    let mut inverted = Keys::new();
    for (label, idx) in keys {
        inverted.insert(idx, label).unwrap();
    }
    inverted
}

fn shortest_path(
    graph: &Graph,
    residuals: &ResidualGraph,
    from: u16,
    to: u16,
    path: &mut Path,
) -> bool {
    let mut q: Deque<u16, 1024> = Deque::new();
    q.push_back(from).unwrap();
    path.clear();
    while let Some(cur) = q.pop_front() {
        if cur == to {
            break;
        }
        for nbr in &graph[cur as usize] {
            if *nbr == from {
                continue;
            }
            if !path.contains_key(nbr) && *residuals.get(&(cur, *nbr)).unwrap() > 0 {
                path.insert(*nbr, cur).unwrap();
                q.push_back(*nbr).unwrap();
            }
        }
    }
    path.get(&to).is_some()
}

fn print_path(path: &Path, keys: &Keys, from: u16, to: u16) {
    let mut cur = to;
    println!("{} to {}: {:?}", from, to, path);
    println!("path:");
    while cur != from {
        println!("{:?}", keys.get(&cur).unwrap_or(&"end"));
        cur = *path.get(&cur).unwrap();
    }
    println!("{:?}", keys.get(&cur).unwrap());
    println!();
}

fn update_residuals(residuals: &mut ResidualGraph, path: &Path, from: u16, to: u16) {
    let mut flow = i16::MAX;
    let mut cur = to;
    while cur != from {
        let pred = path.get(&cur).unwrap();
        flow = flow.min(*residuals.get(&(*pred, cur)).unwrap());
        cur = *pred;
    }
    let mut cur = to;
    while cur != from {
        let pred = path.get(&cur).unwrap();
        let remaining = *residuals.get(&(*pred, cur)).unwrap();
        residuals.insert((*pred, cur), remaining - flow).unwrap().unwrap();
        cur = *pred;
    }
}

fn explore(graph: &Graph, residuals: &ResidualGraph, keys: &Keys, from: u16) -> usize {
    let mut q: Deque<u16, 2048> = Deque::new();
    q.push_back(from).unwrap();
    let mut v: FnvIndexSet<u16, 2048> = FnvIndexSet::new();
    v.insert(from).unwrap();
    while let Some(cur) = q.pop_front() {
        println!("{}:", keys.get(&cur).unwrap());
        for nbr in &graph[cur as usize] {
            if !v.contains(&nbr) && *residuals.get(&(cur, *nbr)).unwrap() > 0 {
                println!("{} {}", keys.get(nbr).unwrap(), *residuals.get(&(cur, *nbr)).unwrap());
                v.insert(*nbr).unwrap();
                q.push_back(*nbr).unwrap();
            }
        }
    }
    v.len()
}

fn partition(graph: &mut Graph, keys: &Keys) -> (usize, usize) {
    // https://cse.iitkgp.ac.in/~pabitra/paper/barna-sdm07.pdf
    // https://en.wikipedia.org/wiki/Edmonds%E2%80%93Karp_algorithm
    // https://www.cs.princeton.edu/courses/archive/spring06/cos226/lectures/maxflow.pdf
    // https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm
    // https://github.com/valiantljk/graph-partition/blob/master/algorithms/connectivity/stoerwagner.py
    // https://dl.acm.org/doi/pdf/10.1145/263867.263872
    // https://www.cs.tau.ac.il/~zwick/grad-algo-08/gmc.pdf
    todo!()
}

fn print_dot(graph: &Graph, keys: &Keys) {
    println!("graph G {{");
    for i in 0..graph.len() - 1 {
        for j in i + 1..graph.len() {
            if graph[i].contains(&(j as u16)) {
                let (src, dst) = (keys.get(&(i as u16)).unwrap(), keys.get(&(j as u16)).unwrap());
                println!("{} -- {};", src, dst);
            }
        }
    }
    println!("}}");
}

static mut GRAPH: Graph = Graph::new();

pub fn part1(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";
        // TODO:
        assert_eq!(part1(input), 0);
        let input = include_str!("../inputs/day25.txt");
        assert_eq!(part1(input), 0);
    }
}
