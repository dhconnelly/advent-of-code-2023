use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node<'a>(BTreeSet<&'a str>);

impl<'a> Node<'a> {
    fn new(key: &'a str) -> Node {
        Self(BTreeSet::from_iter(std::iter::once(key)))
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn merge_with(&self, other: &Node<'a>) -> Node<'a> {
        let mut merged = self.clone();
        merged.0.extend(other.0.iter());
        merged
    }
}

type WeightedEdges<'a> = BTreeMap<Node<'a>, i64>;

#[derive(Clone)]
struct WeightedGraph<'a>(BTreeMap<Node<'a>, WeightedEdges<'a>>);

impl<'a> WeightedGraph<'a> {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn add_edge(&mut self, node1: Node<'a>, node2: Node<'a>, weight: i64) {
        *self.0.entry(node2.clone()).or_default().entry(node1.clone()).or_default() += weight;
        *self.0.entry(node1).or_default().entry(node2).or_default() += weight;
    }

    fn get<'b, 'c: 'b>(&'c self, node: &'b Node<'a>) -> Option<&'c WeightedEdges<'a>> {
        self.0.get(node)
    }

    fn remove(&mut self, node: &Node<'a>) {
        let nbrs = self.0.get(&node).unwrap().keys().cloned().collect::<Vec<_>>();
        for dst in nbrs {
            self.0.get_mut(&dst).unwrap().remove(&node);
        }
        self.0.remove(&node);
    }

    fn keys(&self) -> impl Iterator<Item = &Node<'a>> {
        self.0.keys()
    }
}

fn print_graph(graph: &WeightedGraph) {
    for (src, dsts) in graph.0.iter() {
        println!("{:?} -> {:?}", src, dsts);
    }
}

type Nodes<'a, 'b> = BTreeSet<&'b Node<'a>>;

fn weight(from: &Node, to: &Nodes, graph: &WeightedGraph) -> i64 {
    to.iter().map(|to| graph.get(from).unwrap().get(to).unwrap_or(&0)).sum()
}

fn most_tightly_connected<'c, 'b: 'c, 'a: 'b>(
    graph: &'b WeightedGraph<'a>,
    not_in: &'c Nodes<'a, 'b>,
) -> &'b Node<'a> {
    graph
        .keys()
        .filter(|node| !not_in.contains(node))
        .max_by_key(|node| weight(node, not_in, graph))
        .unwrap()
}

fn merge<'a>(graph: &mut WeightedGraph<'a>, node1: &Node<'a>, node2: &Node<'a>) {
    let merged = node1.merge_with(&node2);
    for (dst, weight) in graph.get(&node1).unwrap().clone() {
        if &dst != node2 {
            graph.add_edge(merged.clone(), dst.clone(), weight);
        }
    }
    for (dst, weight) in graph.get(&node2).unwrap().clone() {
        if &dst != node1 {
            graph.add_edge(merged.clone(), dst.clone(), weight);
        }
    }
    graph.remove(node1);
    graph.remove(node2);
}

type NodeList<'a> = Vec<Node<'a>>;
type Cut<'a> = (i64, NodeList<'a>);

// stoer-wagner algorithm: https://dl.acm.org/doi/pdf/10.1145/263867.263872
fn min_cut_phase<'b, 'a: 'b>(graph: &'b mut WeightedGraph<'a>, src: &Node<'a>) -> Cut<'a> {
    let mut a = Nodes::from_iter(std::iter::once(src));
    let (mut prev2, mut prev1) = (None, src);
    while a.len() < graph.len() {
        let next = most_tightly_connected(graph, &a);
        a.insert(next);
        (prev2, prev1) = (Some(prev1), next);
    }
    let cut = weight(prev1, &a, graph);
    let (prev2, prev1) = (prev2.unwrap().clone(), prev1.clone());
    let a = a.into_iter().cloned().collect();
    merge(graph, &prev2, &prev1);
    (cut, a)
}

fn partition<'a>(mut graph: WeightedGraph<'a>) -> Node<'a> {
    print_graph(&graph);
    let src = graph.keys().next().unwrap().clone();
    let mut min_cut = min_cut_phase(&mut graph, &src);
    while graph.len() > 2 {
        println!("tick");
        let cut = min_cut_phase(&mut graph, &src);
        if cut.0 < min_cut.0 {
            min_cut = cut;
        }
        print_graph(&graph);
    }
    // extract the cluster
    min_cut.1.into_iter().filter(|node| node.len() > 1).next().unwrap()
}

fn parse<'a>(input: &'a str) -> WeightedGraph<'a> {
    let mut graph = WeightedGraph::new();
    for line in input.lines() {
        let (src, dsts) = line.split_once(": ").unwrap();
        for dst in dsts.split_whitespace() {
            graph.add_edge(Node::new(src), Node::new(dst), 1);
        }
    }
    graph
}

pub fn part1(input: &str) -> usize {
    let graph = parse(input);
    let cluster = partition(graph.clone());
    let n = cluster.len();
    n * (graph.len() - n)
}

fn main() {
    let input = include_str!("../../inputs/day25.txt");
    println!("{}", part1(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_paper() {
        let mut graph = WeightedGraph::new();
        assert_eq!(Node::new("1"), Node::new("1"));
        graph.add_edge(Node::new("1"), Node::new("2"), 2);
        graph.add_edge(Node::new("1"), Node::new("5"), 3);
        graph.add_edge(Node::new("5"), Node::new("2"), 2);
        graph.add_edge(Node::new("2"), Node::new("6"), 2);
        graph.add_edge(Node::new("5"), Node::new("6"), 3);
        graph.add_edge(Node::new("2"), Node::new("3"), 3);
        graph.add_edge(Node::new("3"), Node::new("7"), 2);
        graph.add_edge(Node::new("6"), Node::new("7"), 1);
        graph.add_edge(Node::new("3"), Node::new("4"), 4);
        graph.add_edge(Node::new("7"), Node::new("8"), 3);
        graph.add_edge(Node::new("7"), Node::new("4"), 2);
        graph.add_edge(Node::new("4"), Node::new("8"), 2);
        print_graph(&graph);

        let a = Node::new("2");
        let cut = min_cut_phase(&mut graph, &a).0;
        print_graph(&graph);
        assert_eq!(cut, 5);

        let cut = min_cut_phase(&mut graph, &a).0;
        print_graph(&graph);
        assert_eq!(cut, 5);

        let cut = min_cut_phase(&mut graph, &a).0;
        print_graph(&graph);
        assert_eq!(cut, 7);

        let cut = min_cut_phase(&mut graph, &a).0;
        print_graph(&graph);
        assert_eq!(cut, 7);

        let cut = min_cut_phase(&mut graph, &a).0;
        print_graph(&graph);
        assert_eq!(cut, 4);

        let cut = min_cut_phase(&mut graph, &a).0;
        print_graph(&graph);
        assert_eq!(cut, 7);
    }

    #[test]
    fn test_example() {
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
        assert_eq!(part1(input), 54);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../../inputs/day25.txt");
        assert_eq!(part1(input), 0);
    }
}
