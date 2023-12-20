use heapless::{Deque, FnvIndexMap, Vec};
use libc_print::std_name::*;

type Connections<'a> = FnvIndexMap<&'a str, Vec<&'a str, 8>, 64>;
type System<'a> = FnvIndexMap<&'a str, Machine<'a>, 64>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    Low,
    High,
}

impl From<bool> for Pulse {
    fn from(value: bool) -> Self {
        if value {
            Pulse::High
        } else {
            Pulse::Low
        }
    }
}

#[derive(Debug)]
enum Machine<'a> {
    FlipFlop { on: bool },
    Conjunction { recent: FnvIndexMap<&'a str, Pulse, 16> },
}

impl Machine<'_> {
    fn state(&self) -> usize {
        match self {
            Machine::FlipFlop { on } => *on as usize,
            Machine::Conjunction { recent } => {
                recent.values().enumerate().fold(0usize, |b, (i, p)| b | ((*p as usize) << i))
            }
        }
    }
}

type State = Vec<usize, 64>;

fn state(sys: &System) -> State {
    sys.values().map(Machine::state).collect()
}

#[derive(Debug)]
struct Message<'a> {
    from: &'a str,
    to: &'a str,
    pulse: Pulse,
}

enum Outcome {
    LowRx,
    Pulses(usize, usize),
}

// run the system and return the number of pulses sent
fn run<'a>(sys: &mut System<'a>, conns: &Connections<'a>) -> Outcome {
    use Pulse::*;
    let mut q: Deque<Message, 64> = Deque::new();
    let (mut low, mut high) = (0, 0);
    q.push_back(Message { from: "", to: "broadcaster", pulse: Low }).unwrap();
    while let Some(Message { from, to, pulse: input }) = q.pop_front() {
        if to == "rx" && input == Low {
            return Outcome::LowRx;
        }
        match input {
            Low => low += 1,
            High => high += 1,
        }
        let output = match (input, sys.get_mut(to)) {
            (Low, Some(Machine::FlipFlop { on })) => {
                *on = !*on;
                Some(Pulse::from(*on))
            }
            (input, Some(Machine::Conjunction { recent })) => {
                recent.insert(from, input).unwrap();
                if recent.values().all(|pulse| *pulse == High) {
                    Some(Low)
                } else {
                    Some(High)
                }
            }
            (input, None) => Some(input),
            _ => None,
        };
        if let Some(pulse) = output {
            if let Some(dests) = conns.get(to) {
                for dest in dests {
                    q.push_back(Message { from: to, to: dest, pulse }).unwrap();
                }
            }
        }
    }
    Outcome::Pulses(low, high)
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
    for (target, machine) in sys.iter_mut() {
        if let Machine::Conjunction { recent } = machine {
            for (source, outs) in conns.iter() {
                if outs.contains(target) {
                    recent.insert(source, Pulse::Low).unwrap();
                }
            }
        }
    }
    (sys, conns)
}

pub fn part1(input: &str) -> usize {
    let (mut sys, conns) = parse(input);
    let (mut low, mut high) = (0, 0);
    for _ in 0..1000 {
        match run(&mut sys, &conns) {
            Outcome::LowRx => unreachable!(),
            Outcome::Pulses(x, y) => (low, high) = (low + x, high + y),
        }
    }
    low * high
}

pub fn part2(input: &str) -> usize {
    let (mut sys, conns) = parse(input);
    for i in 1..1000 {
        if let Outcome::LowRx = run(&mut sys, &conns) {
            return i;
        }
        println!("{:?}", state(&sys));
    }
    unreachable!()
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
        assert_eq!(part2(input), 832957356);
    }
}
