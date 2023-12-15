use crate::static_vec::StaticVec;

fn hash(s: &str) -> u8 {
    s.bytes().fold(0u16, |cur, b| (cur + b as u16) * 17 % 256) as u8
}

pub fn part1(input: &str) -> i64 {
    input.split(',').map(str::trim).map(hash).map(|h| h as i64).sum()
}

#[derive(Debug)]
enum Op<'a> {
    Insert(&'a str, u8),
    Remove(&'a str),
}

fn parse_op(s: &str) -> Op {
    let i = s.find(|c| c == '-' || c == '=').unwrap();
    let (lens, tail) = (&s[..i], &s[i..]);
    if tail.starts_with('-') {
        Op::Remove(lens)
    } else {
        Op::Insert(lens, tail[1..].parse().unwrap())
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Lens<'a> {
    label: &'a str,
    value: u8,
    next: Option<u16>,
}

type Memory<'a> = StaticVec<Lens<'a>, 2048>;
type Boxes = StaticVec<Option<u16>, 256>;

// returns pointers to the element *before* the element labeled |label| as well
// as to the element itself in the list pointed to by |ptr|.
fn find<'a, 'b>(mem: &'a Memory, ptr: Option<u16>, label: &str) -> (Option<u16>, Option<u16>) {
    let (mut prev, mut next) = (None, ptr);
    while let Some(ptr) = next {
        if mem[ptr as usize].label == label {
            break;
        }
        (prev, next) = (next, mem[ptr as usize].next);
    }
    (prev, next)
}

// remove the element labeled |label| from the list pointed to by |mptr|
fn remove(mem: &mut Memory, mptr: &mut Option<u16>, label: &str) {
    match find(mem, *mptr, label) {
        (_, None) => {}
        (None, Some(ptr)) => *mptr = mem[ptr as usize].next,
        (Some(prev), Some(next)) => mem[prev as usize].next = mem[next as usize].next,
    }
}

// appends a new element to memory and returns its pointer
fn make<'a>(mem: &mut Memory<'a>, label: &'a str, value: u8) -> u16 {
    mem.push(Lens { label, value, next: None });
    mem.len() as u16 - 1
}

// updates the value of the element labeled |label|, if it exists, in the
// list pointed to by |mptr|. if no such element exists, adds a new element
// the list.
fn insert<'a>(mem: &mut Memory<'a>, mptr: &mut Option<u16>, label: &'a str, value: u8) {
    match find(mem, *mptr, label) {
        (_, Some(next)) => mem[next as usize].value = value,
        (None, None) => *mptr = Some(make(mem, label, value)),
        (Some(prev), None) => mem[prev as usize].next = Some(make(mem, label, value)),
    }
}

fn apply<'a>(mem: &mut Memory<'a>, boxes: &mut Boxes, op: Op<'a>) {
    match op {
        Op::Insert(label, value) => insert(mem, &mut boxes[hash(label) as usize], label, value),
        Op::Remove(label) => remove(mem, &mut boxes[hash(label) as usize], label),
    }
}

pub fn part2(input: &str) -> usize {
    let mut mem = Memory::empty();
    let mut boxes = Boxes::of(None);

    // apply all operations
    for tok in input.split(',').map(str::trim) {
        let op = parse_op(tok);
        apply(&mut mem, &mut boxes, op);
    }

    // get the power
    let mut sum = 0;
    for (i, mut b) in boxes.iter().enumerate() {
        let mut j = 1;
        while let Some(ptr) = b {
            let lens = &mem[*ptr as usize];
            sum += (i + 1) * j * lens.value as usize;
            b = &lens.next;
            j += 1;
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(input), 1320);
        assert_eq!(part2(input), 145);
    }

    #[test]
    fn test_real() {
        let input = include_str!("../inputs/day15.txt");
        assert_eq!(part1(input), 508498);
        assert_eq!(part2(input), 279116);
    }
}
