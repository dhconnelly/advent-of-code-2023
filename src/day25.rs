use heapless::{FnvIndexMap, Vec};
use libc_print::std_name::*;

type Connections = Vec<Vec<u16, 16>, 2048>;

fn parse(input: &str) -> Connections {
    let mut keys: FnvIndexMap<&str, u16, 2048> = FnvIndexMap::new();
    for line in input.lines() {
        let (src, dsts) = line.split_once(": ").unwrap();
        for wire in core::iter::once(src).chain(dsts.split_whitespace()) {
            if !keys.contains_key(wire) {
                keys.insert(wire, keys.len() as u16).unwrap();
            }
        }
    }
    let mut conns = Connections::new();
    conns.resize_default(keys.len()).unwrap();
    for line in input.lines() {
        let (src, dsts) = line.split_once(": ").unwrap();
        let src = keys.get(src).unwrap();
        for dst in dsts.split_whitespace() {
            let dst = keys.get(dst).unwrap();
            conns[*src as usize].push(*dst).unwrap();
            conns[*dst as usize].push(*src).unwrap();
        }
    }
    conns
}

pub fn part1(input: &str) -> usize {
    let conns = parse(input);
    println!("digraph G {{");
    for (src, dsts) in conns.iter().enumerate() {
        for dst in dsts {
            println!("{} -> {};", src, dst);
        }
    }
    println!("}}");
    0
}

#[cfg(test)]
mod test {
    use super::*;

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
        let input = include_str!("../inputs/day25.txt");
        assert_eq!(part1(input), 0);
    }
}
