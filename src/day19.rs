use heapless::{FnvIndexMap, Vec};
use libc_print::std_name::*;

type Workflows<'a> = FnvIndexMap<&'a str, Workflow<'a>, 1024>;
type Parts = Vec<Part, 1024>;
type Part = [i16; 4];

#[derive(Clone, Copy, Debug)]
enum Op {
    Lt,
    Gt,
}

impl Op {
    fn apply(self, lhs: i16, rhs: i16) -> bool {
        match self {
            Op::Lt => lhs < rhs,
            Op::Gt => lhs > rhs,
        }
    }
}

#[derive(Clone, Debug)]
struct Rule<'a> {
    var: u8,
    op: Op,
    arg: i16,
    target: &'a str,
}

#[derive(Clone, Debug)]
struct Workflow<'a> {
    ifs: Vec<Rule<'a>, 4>,
    els: &'a str,
}

impl<'a> Workflow<'a> {
    fn apply(&'a self, part: &Part) -> &'a str {
        for rule in &self.ifs {
            if rule.op.apply(part[rule.var as usize], rule.arg) {
                return rule.target;
            }
        }
        self.els
    }
}

fn process(workflows: &Workflows, part: &Part) -> bool {
    let mut label = "in";
    loop {
        let workflow = workflows.get(&label).unwrap();
        label = workflow.apply(&part);
        if label == "A" || label == "R" {
            break;
        }
    }
    label == "A"
}

fn parse_workflow(line: &str) -> (&str, Workflow) {
    let (label, rest) = line.split_once('{').unwrap();
    let mut rules = rest[..rest.len() - 1].split(',').rev();
    let els = rules.next().unwrap();
    let ifs = rules
        .map(|rule| {
            let var = match rule.as_bytes()[0] {
                b'x' => 0,
                b'm' => 1,
                b'a' => 2,
                b's' => 3,
                _ => unreachable!(),
            };
            let op = match rule.as_bytes()[1] {
                b'<' => Op::Lt,
                b'>' => Op::Gt,
                _ => unreachable!(),
            };
            let (arg, target) = rule[2..].split_once(':').unwrap();
            let arg = arg.parse().unwrap();
            Rule { var, op, arg, target }
        })
        .rev()
        .collect();
    (label, Workflow { ifs, els })
}

fn parse_part(line: &str) -> Part {
    let line = &line[1..line.len() - 1];
    let part: Vec<i16, 4> = line.split(',').map(|tok| tok[2..].parse().unwrap()).take(4).collect();
    part.into_array().unwrap()
}

fn parse(input: &str) -> (Workflows, Parts) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows.lines().map(parse_workflow).collect();
    let parts = parts.lines().map(parse_part).collect();
    (workflows, parts)
}

pub fn part1(input: &str) -> i64 {
    let (workflows, parts) = parse(input);
    let part_sum = |part: &Part| part[0] as i64 + part[1] as i64 + part[2] as i64 + part[3] as i64;
    parts.iter().filter(|part| process(&workflows, *part)).map(part_sum).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";
        assert_eq!(part1(input), 19114);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day19.txt");
        assert_eq!(part1(input), 0);
    }
}
