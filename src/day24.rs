use core::ops::Range;

use heapless::Vec;
use libc_print::std_name::*;

const DIMS: usize = 3;
type Systems = Vec<System, 512>;
type Ratio = (i128, i128);

#[derive(Debug)]
struct System {
    x0: [i128; DIMS],
    v0: [i128; DIMS],
}

impl From<&str> for System {
    fn from(value: &str) -> Self {
        let (x0, v0) = value.split_once(" @ ").unwrap();
        let (x0, v0) = (x0.split(", "), v0.split(", "));
        let parse_int = |x: &str| x.trim().parse::<i128>().unwrap();
        let (x0, v0) = (x0.map(parse_int), v0.map(parse_int));
        let x0 = x0.collect::<Vec<i128, 3>>().into_array().unwrap();
        let v0 = v0.collect::<Vec<i128, 3>>().into_array().unwrap();
        System { x0, v0 }
    }
}

fn equivalent((a, b): Ratio, (c, d): Ratio) -> bool {
    if b == 0 || d == 0 {
        false
    } else {
        a * d == b * c
    }
}

fn intersection_between(
    lhs: &System,
    rhs: &System,
    lo: i128,
    hi: i128,
    dim_lo: usize,
    dim_hi: usize,
) -> bool {
    // solve the following system of |dim| equations in two variables (tl and tr):
    // (xl-xr) = tr * (vxr) + tl * (-vxl)
    // (yl-yr) = tr * (vyr) + tl * (-vyl)
    // (zl-zr) = tr * (vzr) + tl * (-vzl)
    // call this:
    // A = tr * B1 + tl * B2 where A is a vector of the left hand sides
    // and B1 and B2 are the vectors scaled by tr and tl respectively.
    let mut a: Vec<i128, 3> = (dim_lo..=dim_hi).map(|i| lhs.x0[i] - rhs.x0[i]).collect();
    let mut b1: Vec<i128, 3> = (dim_lo..=dim_hi).map(|i| rhs.v0[i]).collect();
    let mut b2: Vec<i128, 3> = (dim_lo..=dim_hi).map(|i| -lhs.v0[i]).collect();

    // solve for tl by eliminating tr
    // (xl-xr)*vyr*vzr = tr * (vxr)*vyr*vzr + tl * (-vxl)*vyr*vzr
    // (yl-yr)*vxr*vzr = tr * (vyr)*vxr*vzr + tl * (-vyl)*vxr*vzr
    // (zl-zr)*vxr*vyr = tr * (vzr)*vxr*vyr + tl * (-vzl)*vxr*vyr
    for i in dim_lo..dim_hi {
        for j in i + 1..=dim_hi {
            let scale1 = b1[j];
            let scale2 = b1[i];
            a[i] *= scale1;
            b1[i] *= scale1;
            b2[i] *= scale1;
            a[j] *= scale2;
            b1[j] *= scale2;
            b2[j] *= scale2;
        }
    }

    // now subtract:
    // (xl-xr)*vyr*vzr - (yl-yr)*vxr*vzr = tl * ((-vxl)*vyr*vzr - (-vyl)*vxr*vzr)
    // (xl-xr)*vyr*vzr - (zl-zr)*vxr*vzr = tl * ((-vxl)*vyr*vzr - (-vzl)*vxr*vyr)
    // (yl-yr)*vxr*vzr - (zl-zr)*vxr*vyr = tl * ((-vyl)*vxr*vzr - (-vzl)*vxr*vyr)
    let mut tl = Vec::<Ratio, 3>::new();
    for i in dim_lo..dim_hi {
        for j in i + 1..=dim_hi {
            let dividend = a[i] - a[j];
            let divisor = b2[i] - b2[j];
            tl.push((dividend, divisor)).unwrap();
        }
    }
    if tl[1..].iter().any(|r| !equivalent(*r, tl[0])) {
        return false;
    }
    let (tl_top, tl_bottom) = tl[0];
    if tl_bottom == 0 {
        return false;
    }
    if tl_bottom.signum() != tl_top.signum() {
        return false;
    }

    // now substitute for tl and solve for tr
    // (xl-xr)*vyr*vzr = tr * (vxr)*vyr*vzr + (tl_top/tl_bottom) * (-vxl)*vyr*vzr
    // (yl-yr)*vxr*vzr = tr * (vyr)*vxr*vzr + (tl_top/tl_bottom) * (-vyl)*vxr*vzr
    // (zl-zr)*vxr*vyr = tr * (vzr)*vxr*vyr + (tl_top/tl_bottom) * (-vzl)*vxr*vyr
    // multiply through by tl_bottom
    // ((xl-xr)*vyr*vzr)*tl_bottom = tr * (vxr)*vyr*vzr*tl_bottom + tl_top * (-vxl)*vyr*vzr
    // ((yl-yr)*vxr*vzr)*tl_bottom= tr * (vyr)*vxr*vzr*tl_bottom + tl_top * (-vyl)*vxr*vzr
    // ((zl-zr)*vxr*vyr)*tl_bottom= tr * (vzr)*vxr*vyr*tl_bottom + tl_top * (-vzl)*vxr*vyr
    let mut tr = Vec::<Ratio, 3>::new();
    for i in dim_lo..dim_hi {
        let dividend = (a[i] * tl_bottom) - (tl_top * b2[i]);
        let divisor = b1[i] * tl_bottom;
        tr.push((dividend, divisor)).unwrap();
    }
    if tr[1..].iter().any(|r| !equivalent(*r, tr[0])) {
        return false;
    }
    let (tr_top, tr_bottom) = tr[0];
    if tr_bottom == 0 {
        return false;
    }
    if tr_bottom.signum() != tr_top.signum() {
        return false;
    }

    for i in dim_lo..=dim_hi {
        let t = tl_top as f64 / tl_bottom as f64;
    }

    // substitute and enforce the contraints
    // lo <= xl + (tl_top / tl_bottom) * vxl <= hi
    // lo <= yl + (tl_top / tl_bottom) * vyl <= hi
    // lo <= zl + (tl_top / tl_bottom) * vzl <= hi
    (dim_lo..=dim_hi).all(|i| {
        let left = (lo - lhs.x0[i]) * tl_bottom;
        let right = (hi - lhs.x0[i]) * tl_bottom;
        let mid = tl_top * lhs.v0[i];
        if tl_bottom > 0 {
            left <= mid && mid <= right
        } else {
            left >= mid && mid >= right
        }
    })
}

fn parse(input: &str) -> Systems {
    input.lines().map(System::from).collect()
}

fn count_intersections(input: &str, lo: i128, hi: i128, dim_lo: usize, dim_hi: usize) -> usize {
    let sys = parse(input);
    let mut intersections = 0;
    for i in 0..sys.len() - 1 {
        for j in i + 1..sys.len() {
            if intersection_between(&sys[i], &sys[j], lo, hi, dim_lo, dim_hi) {
                intersections += 1;
            }
        }
    }
    intersections
}

pub fn part1(input: &str) -> usize {
    let (lo, hi) = (200000000000000, 400000000000000);
    count_intersections(input, lo, hi, 0, 1)
}

pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    const REAL_INPUT: &str = include_str!("../inputs/day24.txt");

    #[test]
    fn test_example() {
        assert_eq!(count_intersections(TEST_INPUT, 7, 27, 0, 1), 2);
        assert_eq!(part2(TEST_INPUT), 0);
    }

    #[test]
    fn test_real() {
        assert_eq!(part1(REAL_INPUT), 15593);
        assert_eq!(part2(REAL_INPUT), 0);
    }
}
