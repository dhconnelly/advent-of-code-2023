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

type Memory<'a> = StaticVec<Lens<'a>, 32768>;
type Boxes = StaticVec<Option<u16>, 256>;

// find the ptr to the element before the element labeled |label|. if not found,
// returns the pointer to the last element in the linked list.
fn find_pred<'a, 'b>(mem: &'a [Lens<'b>], mut ptr: u16, label: &str) -> u16 {
    while let Some(i) = mem[ptr as usize].next {
        let next = &mem[i as usize];
        if next.label == label {
            return ptr;
        }
        ptr = i;
    }
    ptr
}

// remove the element labeled |label| from the list pointed to by |mptr|
fn remove(mem: &mut Memory, mptr: &mut Option<u16>, label: &str) {
    if let Some(ptr) = *mptr {
        if mem[ptr as usize].label == label {
            *mptr = mem[ptr as usize].next;
        } else {
            let pred = find_pred(&mem[..], ptr, label) as usize;
            match mem[pred].next {
                Some(j) => mem[pred].next = mem[j as usize].next,
                None => {}
            }
        }
    }
}

// updates the value of the element labeled |label|, if it exists, in the
// list pointed to by |mptr|. if no such element exists, adds a new element
// the list.
fn insert<'a>(mem: &mut Memory<'a>, mptr: &mut Option<u16>, label: &'a str, value: u8) {
    match *mptr {
        None => {
            *mptr = Some(mem.len() as u16);
            mem.push(Lens { label, value, next: None });
        }
        Some(ptr) => {
            if mem[ptr as usize].label == label {
                mem[ptr as usize].value = value;
            } else {
                let pred = find_pred(&mem[..], ptr, label) as usize;
                match mem[pred].next {
                    Some(j) => mem[j as usize].value = value,
                    None => {
                        mem[pred].next = Some(mem.len() as u16);
                        mem.push(Lens { label, value, next: None });
                    }
                }
            }
        }
    }
}

fn apply<'a>(mem: &mut Memory<'a>, boxes: &mut Boxes, op: Op<'a>) {
    match op {
        Op::Insert(label, value) => {
            insert(mem, &mut boxes[hash(label) as usize], label, value);
        }
        Op::Remove(label) => {
            remove(mem, &mut boxes[hash(label) as usize], label);
        }
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
