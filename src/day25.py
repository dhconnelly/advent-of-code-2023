import networkx as nx
from networkx.algorithms.connectivity.stoerwagner import stoer_wagner

def parse(s):
    G = nx.Graph()
    for line in s.splitlines():
        src, dsts = line.split(': ', 1)
        for dst in dsts.split(' '):
            G.add_edge(src, dst)
    return G

def partition(graph):
    return stoer_wagner(graph)

TEST_INPUT = '''jqt: rhn xhk nvd
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
'''

def solve(s):
    graph = parse(s)
    _, p = partition(graph)
    a, b = p
    return len(a) * len(b)

if __name__ == '__main__':
    import sys
    arg = sys.argv[1]
    if arg == 'test':
        solve(TEST_INPUT)
    else:
        with open(sys.argv[1]) as f:
            s = f.read()
            print(solve(s))

