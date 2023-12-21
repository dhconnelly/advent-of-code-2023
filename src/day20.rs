use heapless::{Deque, FnvIndexMap, Vec};

type Connections<'a> = FnvIndexMap<&'a str, Vec<&'a str, 8>, 64>;
type System<'a> = FnvIndexMap<&'a str, Machine<'a>, 64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

impl From<bool> for Pulse {
    fn from(value: bool) -> Self {
        match value {
            true => Pulse::High,
            false => Pulse::Low,
        }
    }
}

#[derive(Debug, Clone)]
enum Machine<'a> {
    FlipFlop { on: bool },
    Conjunction { recent: FnvIndexMap<&'a str, Pulse, 16> },
}

#[derive(Debug)]
struct Message<'a> {
    from: &'a str,
    to: &'a str,
    pulse: Pulse,
}

fn run<'a>(sys: &mut System<'a>, conns: &Connections<'a>, mut f: impl FnMut(Message)) {
    let mut q: Deque<Message, 64> = Deque::new();
    q.push_back(Message { from: "", to: "broadcaster", pulse: Pulse::Low }).unwrap();
    while let Some(msg @ Message { from, to, pulse: input }) = q.pop_front() {
        f(msg);
        let output = match sys.get_mut(to) {
            Some(Machine::FlipFlop { on }) if input == Pulse::Low => {
                *on = !*on;
                Some(Pulse::from(*on))
            }
            Some(Machine::Conjunction { recent }) => {
                recent.insert(from, input).unwrap();
                Some(Pulse::from(recent.values().any(|pulse| *pulse != Pulse::High)))
            }
            None => Some(input),
            _ => None,
        };
        if let Some((pulse, dests)) = output.zip(conns.get(to)) {
            for dest in dests {
                q.push_back(Message { from: to, to: dest, pulse }).unwrap();
            }
        }
    }
}

fn count_pulses<'a>(sys: &mut System<'a>, conns: &Connections<'a>) -> (usize, usize) {
    let (mut low, mut high) = (0, 0);
    run(sys, conns, |Message { pulse, .. }| match pulse {
        Pulse::Low => low += 1,
        Pulse::High => high += 1,
    });
    (low, high)
}

fn run_until<'a>(
    sys: &mut System<'a>,
    conns: &Connections<'a>,
    dest: &'a str,
    want: Pulse,
) -> usize {
    let mut count = None;
    for presses in 1.. {
        run(sys, conns, |Message { to, pulse, .. }| {
            if to == dest && pulse == want {
                count = Some(presses);
            }
        });
        if count.is_some() {
            break;
        }
    }
    count.unwrap()
}

fn parse(input: &str) -> (System, Connections) {
    let mut sys = System::new();
    let mut conns = Connections::new();
    for line in input.lines() {
        let (mut label, out) = line.split_once(" -> ").unwrap();
        let out = out.split(", ").collect();
        if label != "broadcaster" {
            let typ = label.as_bytes()[0];
            label = &label[1..];
            sys.insert(
                label,
                match typ {
                    b'&' => Machine::Conjunction { recent: FnvIndexMap::new() },
                    b'%' => Machine::FlipFlop { on: false },
                    _ => unreachable!(),
                },
            )
            .unwrap();
        }
        conns.insert(label, out).unwrap();
    }
    for (dst, machine) in sys.iter_mut() {
        if let Machine::Conjunction { recent } = machine {
            for (src, _) in conns.iter().filter(|(_, dsts)| dsts.contains(dst)) {
                recent.insert(src, Pulse::Low).unwrap();
            }
        }
    }
    (sys, conns)
}

pub fn part1(input: &str) -> usize {
    let (mut sys, conns) = parse(input);
    let (mut low, mut high) = (0, 0);
    for _ in 0..1000 {
        let (x, y) = count_pulses(&mut sys, &conns);
        (low, high) = (low + x, high + y);
    }
    low * high
}

fn find_source<'a, 'b>(
    conns: &'b Connections<'a>,
    of: &'a str,
) -> impl Iterator<Item = &'a str> + 'b {
    conns.iter().filter(move |(_, dsts)| dsts.contains(&of)).map(|(src, _)| *src)
}

pub fn part2(input: &str) -> usize {
    let (sys, conns) = parse(input);
    // this is basically day 8
    // based on manual inspection of the input file: https://bit.ly/3RSUAbq
    let sink = find_source(&conns, "rx").next().unwrap();
    let sources: Vec<&str, 4> = find_source(&conns, sink).collect();
    assert!(matches!(sys.get(sink), Some(Machine::Conjunction { .. })));
    assert!(sources.iter().all(|src| matches!(sys.get(src), Some(Machine::Conjunction { .. }))));
    let cycles = sources.iter().map(|src| run_until(&mut sys.clone(), &conns, src, Pulse::Low));
    cycles.product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";
        assert_eq!(part1(input), 32000000);
    }

    #[test]
    fn test_example2() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";
        assert_eq!(part1(input), 11687500);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day20.txt");
        assert_eq!(part1(input), 832957356);
        assert_eq!(part2(input), 240162699605221);
    }
}
