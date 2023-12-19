use heapless::{FnvIndexMap, Vec};

// =============================================================================
// workflows

#[derive(Clone, Copy, Debug)]
enum Op {
    Lt,
    Gt,
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

type Workflows<'a> = FnvIndexMap<&'a str, Workflow<'a>, 1024>;

// =============================================================================
// workflow application

type Parts = Vec<Part, 1024>;
type Part = [i16; 4];

fn apply_op(op: Op, lhs: i16, rhs: i16) -> bool {
    match op {
        Op::Lt => lhs < rhs,
        Op::Gt => lhs > rhs,
    }
}

fn apply_workflow<'a>(workflow: &'a Workflow, part: &Part) -> &'a str {
    for rule in &workflow.conds {
        if apply_op(rule.op, part[rule.var as usize], rule.arg) {
            return rule.target;
        }
    }
    workflow.alt
}

fn is_valid(workflows: &Workflows, part: &Part) -> bool {
    let mut label = "in";
    while label != "A" && label != "R" {
        label = apply_workflow(workflows.get(&label).unwrap(), part);
    }
    label == "A"
}

fn sum_ratings(workflows: &Workflows, parts: &Parts) -> i64 {
    let sum = |part: &Part| -> i64 { part.iter().map(|x| *x as i64).sum() };
    parts.iter().filter(|part| is_valid(&workflows, *part)).map(sum).sum()
}

// =============================================================================
// workflow simulation

type Range = (i16, i16);
type AbstractPart = [Range; 4];

fn update_part(mut part: AbstractPart, var: u8, with: Range) -> AbstractPart {
    part[var as usize] = with;
    part
}

fn remove_range(outer: Range, inner: Range) -> Range {
    if outer.0 == inner.0 {
        (inner.1 + 1, outer.1)
    } else {
        (outer.0, inner.0 - 1)
    }
}

fn satisfy_op(op: Op, lhs: Range, rhs: i16) -> Option<Range> {
    match op {
        Op::Lt if lhs.0 >= rhs => None,
        Op::Lt => Some((lhs.0, lhs.1.min(rhs - 1))),
        Op::Gt if lhs.1 <= rhs => None,
        Op::Gt => Some((lhs.0.max(rhs + 1), lhs.1)),
    }
}

fn satisfy_workflow<'a, const N: usize>(
    workflow: &'a Workflow,
    mut part: AbstractPart,
    workflows: &Workflows,
    valids: &mut Vec<AbstractPart, N>,
) {
    let mut satisfy = |label, part| match label {
        "R" => {}
        "A" => valids.push(part).unwrap(),
        target => satisfy_workflow(workflows.get(target).unwrap(), part, workflows, valids),
    };
    for Rule { var, op, arg, target } in &workflow.conds {
        let x = part[*var as usize];
        let y = match satisfy_op(*op, x, *arg) {
            None => continue,
            Some(y) => y,
        };
        satisfy(*target, update_part(part.clone(), *var, y));
        part[*var as usize] = remove_range(x, y);
    }
    satisfy(workflow.alt, part);
}

fn count_valid(part: &AbstractPart) -> i64 {
    part.iter().map(|(a, b)| (b - a + 1) as i64).product()
}

fn total_valid(workflows: &Workflows) -> i64 {
    let part = [(1, 4000); 4];
    let mut valids: Vec<AbstractPart, 1024> = Vec::new();
    satisfy_workflow(workflows.get("in").unwrap(), part, workflows, &mut valids);
    valids.iter().map(count_valid).sum()
}

// =============================================================================
// parsing

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

// =============================================================================
// solutions

pub fn part1(input: &str) -> i64 {
    let (workflows, parts) = parse(input);
    sum_ratings(&workflows, &parts)
}

pub fn part2(input: &str) -> i64 {
    let workflows = parse(input).0;
    total_valid(&workflows)
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
