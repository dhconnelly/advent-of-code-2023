use advent_of_code_2023::static_queue::StaticQueue;
use advent_of_code_2023::static_vec::StaticVec;
use heapless::FnvIndexSet;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::thread;
use tetra::graphics::{self, Color};
use tetra::graphics::{
    mesh::{Mesh, ShapeStyle},
    DrawParams, Rectangle,
};
use tetra::math;
use tetra::{Context, ContextBuilder, State};

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 768;
const SLEEP_MICROS: u64 = 100;
const TILE_WIDTH: f32 = 3.;
const TILE_HEIGHT: f32 = 3.;

#[derive(Clone, Copy)]
enum Sprite {
    Blank,
    Empty,
    Loop(u8),
    Walk(u8),
    Interior,
    Transient,
}

impl Sprite {
    fn color(self) -> Color {
        match self {
            Sprite::Blank => Color::WHITE,
            Sprite::Empty => Color::BLACK,
            Sprite::Loop(_) => Color::GREEN,
            Sprite::Walk(_) => Color::BLUE,
            Sprite::Interior => Color::RED,
            Sprite::Transient => Color::rgb(1., 0., 1.),
        }
    }

    fn draw(self, ctx: &mut Context, tile: &Mesh, pos: math::Vec2<f32>, rect: Rectangle) {
        match self {
            Sprite::Walk(b'-') | Sprite::Loop(b'-') => {
                for i in 0..3 {
                    let pos = pos
                        .with_x(pos.x + i as f32 * rect.width)
                        .with_y(pos.y + 1 as f32 * rect.height);
                    tile.draw(ctx, DrawParams::new().position(pos).color(self.color()));
                }
            }
            Sprite::Walk(b'|') | Sprite::Loop(b'|') => {
                for j in 0..3 {
                    let pos = pos
                        .with_x(pos.x + 1 as f32 * rect.width)
                        .with_y(pos.y + j as f32 * rect.height);
                    tile.draw(ctx, DrawParams::new().position(pos).color(self.color()));
                }
            }
            Sprite::Walk(b'7') | Sprite::Loop(b'7') => {
                let pos1 = pos.with_y(pos.y + rect.height);
                let pos2 = pos1.with_x(pos1.x + rect.width);
                let pos3 = pos2.with_y(pos2.y + rect.height);
                for pos in [pos1, pos2, pos3] {
                    tile.draw(ctx, DrawParams::new().position(pos).color(self.color()));
                }
            }
            Sprite::Walk(b'L') | Sprite::Loop(b'L') => {
                let pos1 = pos.with_x(pos.x + rect.width);
                let pos2 = pos1.with_y(pos1.y + rect.height);
                let pos3 = pos2.with_x(pos2.x + rect.width);
                for pos in [pos1, pos2, pos3] {
                    tile.draw(ctx, DrawParams::new().position(pos).color(self.color()));
                }
            }
            Sprite::Walk(b'F') | Sprite::Loop(b'F') => {
                let pos1 = pos.with_y(pos.y + rect.height).with_x(pos.x + 2. * rect.width);
                let pos2 = pos1.with_x(pos1.x - rect.width);
                let pos3 = pos2.with_y(pos2.y + rect.height);
                for pos in [pos1, pos2, pos3] {
                    tile.draw(ctx, DrawParams::new().position(pos).color(self.color()));
                }
            }
            Sprite::Walk(b'J') | Sprite::Loop(b'J') => {
                let pos1 = pos.with_y(pos.y + rect.height);
                let pos2 = pos1.with_x(pos1.x + rect.width);
                let pos3 = pos2.with_y(pos2.y - rect.height);
                for pos in [pos1, pos2, pos3] {
                    tile.draw(ctx, DrawParams::new().position(pos).color(self.color()));
                }
            }
            Sprite::Walk(b'S')
            | Sprite::Loop(b'S')
            | Sprite::Empty
            | Sprite::Interior
            | Sprite::Transient => {
                for i in 0..3 {
                    for j in 0..3 {
                        let pos = pos
                            .with_x(pos.x + i as f32 * rect.width)
                            .with_y(pos.y + j as f32 * rect.height);
                        tile.draw(ctx, DrawParams::new().position(pos).color(self.color()));
                    }
                }
            }
            _ => panic!("invalid loop"),
        }
    }
}

type Buffer = Vec<Vec<Sprite>>;

struct Visualizer {
    data: Arc<Mutex<Buffer>>,
    transient: Vec<(Pt2, Sprite)>,
    in_transient: HashSet<Pt2>,
}

impl Visualizer {
    fn new(rows: i32, cols: i32) -> Self {
        let transient = Vec::new();
        let in_transient = HashSet::new();
        let data = Arc::new(Mutex::new(vec![vec![Sprite::Empty; cols as usize]; rows as usize]));
        let viz = Self { data, transient, in_transient };
        viz
    }

    fn mark_loop(&mut self, (row, col): Pt2, c: u8) {
        {
            self.data.lock().unwrap()[row as usize][col as usize] = Sprite::Loop(c);
        }
    }

    fn mark_walk(&mut self, (row, col): Pt2, c: u8) {
        {
            self.data.lock().unwrap()[row as usize][col as usize] = Sprite::Walk(c);
        }
    }

    fn mark_interior(&mut self, (row, col): Pt2) {
        {
            self.data.lock().unwrap()[row as usize][col as usize] = Sprite::Interior;
        }
    }

    fn mark_transient(&mut self, pt @ (row, col): Pt2) {
        if self.in_transient.contains(&pt) {
            return;
        }
        {
            let cur = &mut self.data.lock().unwrap()[row as usize][col as usize];
            self.transient.push((pt, *cur));
            self.in_transient.insert(pt);
            *cur = Sprite::Transient;
        }
    }

    fn clear_transient(&mut self) {
        {
            for ((row, col), prev) in &self.transient {
                self.data.lock().unwrap()[*row as usize][*col as usize] = *prev;
            }
            self.transient.clear();
            self.in_transient.clear();
        }
    }
}

struct VizState {
    data: Arc<Mutex<Buffer>>,
    rect: Rectangle,
    tile: Mesh,
}

impl VizState {
    fn new(ctx: &mut Context, data: Arc<Mutex<Buffer>>) -> Self {
        let tile;
        let rect;
        {
            let locked = data.lock().unwrap();
            let tile_height = HEIGHT as f32 / locked.len() as f32 / TILE_HEIGHT;
            let tile_width = WIDTH as f32 / locked[0].len() as f32 / TILE_WIDTH;
            rect = Rectangle::new(0., 0., tile_width, tile_height);
            tile = Mesh::rectangle(ctx, ShapeStyle::Fill, rect).unwrap();
        }
        VizState { data, tile, rect }
    }
}

impl State for VizState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        let data = self.data.lock().unwrap();
        graphics::clear(ctx, Color::rgb(0., 0., 0.));
        for row in 0..data.len() {
            for col in 0..data[row].len() {
                let pos = math::Vec2::new(
                    col as f32 * self.rect.width * TILE_WIDTH,
                    row as f32 * self.rect.height * TILE_HEIGHT,
                );
                data[row][col].draw(ctx, &self.tile, pos, self.rect);
            }
        }
        Ok(())
    }
}

type Tile = u8;
type Pt2 = (i32, i32);
type Set<T> = FnvIndexSet<T, 16384>;
type Queue<T> = StaticQueue<T, 16384>;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum Dir {
    #[default]
    Above,
    Below,
    Left,
    Right,
}

fn go((row, col): Pt2, dir: Dir) -> Pt2 {
    match dir {
        Dir::Left => (row, col - 1),
        Dir::Right => (row, col + 1),
        Dir::Above => (row - 1, col),
        Dir::Below => (row + 1, col),
    }
}

#[derive(Debug)]
struct Grid<'a> {
    data: &'a [u8],
    width: i32,
    height: i32,
}

impl Grid<'_> {
    fn at(&self, (row, col): Pt2) -> Option<Tile> {
        if row < 0 || row >= self.height {
            None
        } else if col < 0 || col >= self.width {
            None
        } else {
            Some(self.data[(row * (self.width + 1) + col) as usize])
        }
    }
}

fn tube_directions(from: Tile) -> StaticVec<Dir, 4> {
    match from {
        b'|' => [Dir::Above, Dir::Below].into_iter().collect(),
        b'-' => [Dir::Left, Dir::Right].into_iter().collect(),
        b'L' => [Dir::Above, Dir::Right].into_iter().collect(),
        b'J' => [Dir::Above, Dir::Left].into_iter().collect(),
        b'7' => [Dir::Left, Dir::Below].into_iter().collect(),
        b'F' => [Dir::Right, Dir::Below].into_iter().collect(),
        b'S' => [Dir::Left, Dir::Right, Dir::Above, Dir::Below].into_iter().collect(),
        _ => StaticVec::empty(),
    }
}

fn tube_neighbors(grid: &Grid, from: Pt2, viz: &mut Visualizer) -> StaticVec<Pt2, 4> {
    grid.at(from)
        .map(tube_directions)
        .into_iter()
        .flatten()
        .map(|dir| go(from, dir))
        .filter(|pt| grid.at(*pt).is_some())
        .collect()
}

fn tube_connections(grid: &Grid, from: Pt2, viz: &mut Visualizer) -> StaticVec<Pt2, 4> {
    tube_neighbors(grid, from, viz)
        .into_iter()
        .filter(|nbr| tube_neighbors(grid, *nbr, viz).contains(&from))
        .collect()
}

fn find(grid: &Grid, tile: Tile) -> Option<Pt2> {
    for row in 0..grid.height {
        for col in 0..grid.width {
            if grid.at((row, col)) == Some(tile) {
                return Some((row, col));
            }
        }
    }
    None
}

fn find_loop(grid: &Grid, start: Pt2, v: &mut Set<Pt2>, viz: &mut Visualizer) {
    let mut q = Queue::new();
    q.push_back((start, 0));
    v.insert(start).unwrap();
    viz.mark_loop(start, grid.at(start).unwrap());
    while let Some(front @ (cur, dist)) = q.pop_front() {
        let nbrs = tube_connections(&grid, cur, viz);
        for nbr in nbrs {
            if q.front() == Some(&front) {
                v.insert(nbr).unwrap();
                return;
            }
            if v.contains(&nbr) {
                continue;
            }
            v.insert(nbr).unwrap();
            viz.mark_loop(nbr, grid.at(nbr).unwrap());
            q.push_back((nbr, dist + 1));
        }
        thread::sleep(std::time::Duration::from_micros(SLEEP_MICROS));
    }
}

fn interior_neighbors(grid: &Grid, prev: Pt2, cur: Pt2) -> StaticVec<Pt2, 4> {
    use Dir::*;
    match grid.at(cur) {
        Some(b'F') if prev.1 > cur.1 => StaticVec::from([go(cur, Above), go(cur, Left)]),
        Some(b'J') if prev.1 < cur.1 => StaticVec::from([go(cur, Below), go(cur, Right)]),
        Some(b'7') if prev.0 > cur.0 => StaticVec::from([go(cur, Above), go(cur, Right)]),
        Some(b'L') if prev.0 < cur.0 => StaticVec::from([go(cur, Left), go(cur, Below)]),
        Some(b'|') if prev.0 < cur.0 => StaticVec::from([go(cur, Left)]),
        Some(b'|') if prev.0 > cur.0 => StaticVec::from([go(cur, Right)]),
        Some(b'-') if prev.1 < cur.1 => StaticVec::from([go(cur, Below)]),
        Some(b'-') if prev.1 > cur.1 => StaticVec::from([go(cur, Above)]),
        _ => StaticVec::empty(),
    }
}

fn explore(grid: &Grid, looop: &Set<Pt2>, from: Pt2, v: &mut Set<Pt2>, viz: &mut Visualizer) {
    for dir in [Dir::Left, Dir::Right, Dir::Above, Dir::Below] {
        let nbr = go(from, dir);
        if v.contains(&nbr) || looop.contains(&nbr) {
            continue;
        }
        v.insert(nbr).unwrap();
        viz.mark_interior(nbr);
        thread::sleep(std::time::Duration::from_micros(SLEEP_MICROS));
        explore(grid, looop, nbr, v, viz);
    }
}

fn interior_area(grid: &Grid, looop: &Set<Pt2>, viz: &mut Visualizer) -> i32 {
    let start = *looop.iter().min_by(|(r1, c1), (r2, c2)| r1.cmp(r2).then(c1.cmp(c2))).unwrap();
    let mut v = Set::new();
    let (mut prev, mut cur) = (start, start);
    while cur != start || prev == start {
        viz.mark_walk(cur, grid.at(cur).unwrap());
        thread::sleep(std::time::Duration::from_micros(SLEEP_MICROS));
        for pt in interior_neighbors(grid, prev, cur) {
            if !v.contains(&pt) && !looop.contains(&pt) {
                v.insert(pt).unwrap();
                explore(grid, looop, pt, &mut v, viz);
            }
        }
        let nbrs = tube_connections(grid, cur, viz);
        let next = nbrs.into_iter().filter(|nbr| *nbr != prev).next().unwrap();
        (prev, cur) = (cur, next);
    }
    v.len() as i32
}

fn parse(input: &str) -> Grid {
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    Grid { data: input.as_bytes(), width, height }
}

fn part1(grid: &Grid, viz: &mut Visualizer) -> Set<Pt2> {
    let start = find(&grid, b'S').unwrap();
    let mut looop = Set::new();
    find_loop(&grid, start, &mut looop, viz);
    looop
}

fn part2(grid: &Grid, looop: Set<Pt2>, viz: &mut Visualizer) -> i32 {
    interior_area(&grid, &looop, viz)
}

fn main() -> tetra::Result {
    let text = include_str!("../../inputs/day10.txt");
    let grid = parse(&text);
    let mut viz = Visualizer::new(grid.height, grid.width);
    let state = viz.data.clone();
    thread::spawn(move || {
        thread::sleep_ms(2000);
        let looop = part1(&grid, &mut viz);
        part2(&grid, looop, &mut viz);
    });
    ContextBuilder::new("day10viz", WIDTH, HEIGHT).build()?.run(|ctx| Ok(VizState::new(ctx, state)))
}
