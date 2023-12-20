use heapless::{FnvIndexMap, Vec};
use libc_print::std_name::*;

type Connections<'a> = FnvIndexMap<&'a str, Vec<&'a str, 8>, 64>;
type System<'a> = FnvIndexMap<&'a str, Machine<'a>, 64>;

#[derive(Debug)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
enum Machine<'a> {
    FlipFlop { on: bool },
    Conjunction { recent: FnvIndexMap<&'a str, Pulse, 8> },
}

// run the system and return the number of pulses sent
fn run(sys: &mut System, conns: &Connections) -> usize {
    0
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
    // need to iterate over mut first
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
    println!("{:?}", sys);
    println!("{:?}", conns);
    let mut pulses = 0;
    for i in 0..1000 {
        pulses += run(&mut sys, &conns);
    }
    pulses
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
}
