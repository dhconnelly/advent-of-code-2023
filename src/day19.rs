use heapless::{FnvIndexMap, Vec};

type Workflows<'a> = FnvIndexMap<&'a str, Workflow<'a>, 1024>;
type Parts = Vec<Part, 1024>;
type Part = [i16; 4];

type Range = (i16, i16);
type AbstractPart = [Range; 4];

fn update(mut part: AbstractPart, var: u8, with: Range) -> AbstractPart {
    part[var as usize] = with;
    part
}

fn sub(outer: Range, inner: Range) -> Range {
    if outer.0 == inner.0 {
        (inner.1 + 1, outer.1)
    } else {
        (outer.0, inner.0 - 1)
    }
}

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

    fn simulate(self, lhs: Range, rhs: i16) -> Option<Range> {
        match self {
            Op::Lt if lhs.0 >= rhs => None,
            Op::Lt => Some((lhs.0, lhs.1.min(rhs - 1))),
            Op::Gt if lhs.1 <= rhs => None,
            Op::Gt => Some((lhs.0.max(rhs + 1), lhs.1)),
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
    conds: Vec<Rule<'a>, 4>,
    alt: &'a str,
}

impl<'a> Workflow<'a> {
    fn apply(&'a self, part: &Part) -> &'a str {
        for rule in &self.conds {
            if rule.op.apply(part[rule.var as usize], rule.arg) {
                return rule.target;
            }
        }
        self.alt
    }

    fn simulate<const N: usize>(
        &'a self,
        mut part: AbstractPart,
        workflows: &Workflows,
        valids: &mut Vec<AbstractPart, N>,
    ) {
        let mut resolve = |label, part| match label {
            "R" => {}
            "A" => valids.push(part).unwrap(),
            target => workflows.get(target).unwrap().simulate(part, workflows, valids),
        };
        for Rule { var, op, arg, target } in &self.conds {
            let x = part[*var as usize];
            let y = match op.simulate(x, *arg) {
                None => continue,
                Some(y) => y,
            };
            resolve(*target, update(part.clone(), *var, y));
            part[*var as usize] = sub(x, y);
        }
        resolve(self.alt, part);
    }
}

fn process(workflows: &Workflows, part: &Part) -> bool {
    let mut label = "in";
    while label != "A" && label != "R" {
        label = workflows.get(&label).unwrap().apply(&part);
    }
    label == "A"
}

fn valid_combos(part: &AbstractPart) -> i64 {
    part.iter().map(|(a, b)| (b - a + 1) as i64).product()
}

fn all_valid(workflows: &Workflows) -> i64 {
    let part = [(1, 4000); 4];
    let mut valids: Vec<AbstractPart, 1024> = Vec::new();
    workflows.get("in").unwrap().simulate(part, workflows, &mut valids);
    valids.iter().map(valid_combos).sum()
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
    (label, Workflow { conds: ifs, alt: els })
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
    let sum = |part: &Part| -> i64 { part.iter().map(|x| *x as i64).sum() };
    parts.iter().filter(|part| process(&workflows, *part)).map(sum).sum()
}

pub fn part2(input: &str) -> i64 {
    let workflows = parse(input).0;
    all_valid(&workflows)
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
        assert_eq!(part2(input), 167409079868000);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day19.txt");
        assert_eq!(part1(input), 367602);
        assert_eq!(part2(input), 125317461667458);
    }
}
