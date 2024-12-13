use std::collections::HashSet;

use super::read_grid_bytes;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    const fn new(x: usize, y: usize) -> Self { Self { x, y } }

    fn top(&self) -> Option<Self> { (self.y > 0).then(|| (Self { x: self.x, y: self.y - 1 })) }

    fn right(&self, max: usize) -> Option<Self> {
        (self.x + 1 < max).then(|| (Self { x: self.x + 1, y: self.y }))
    }

    fn bottom(&self, max: usize) -> Option<Self> {
        (self.y + 1 < max).then(|| (Self { x: self.x, y: self.y + 1 }))
    }

    fn left(&self) -> Option<Self> { (self.x > 0).then(|| (Self { x: self.x - 1, y: self.y })) }

    fn neighbours(&self, grid: &[Box<[u8]>]) -> [Option<(Self, u8)>; 4] {
        [self.top(), self.right(grid[0].len()), self.bottom(grid.len()), self.left()]
            .map(|o| o.map(|p| (p, grid[p.y][p.x])))
    }

    fn neighbours_corners(&self, grid: &[Box<[u8]>]) -> [[Option<(Self, u8)>; 3]; 3] {
        let top = self.top();
        let top_left = top.and_then(|p| p.left());
        let top_right = top.and_then(|p| p.right(grid[0].len()));
        let bottom = self.bottom(grid.len());
        let bottom_left = bottom.and_then(|p| p.left());
        let bottom_right = bottom.and_then(|p| p.right(grid[0].len()));
        let left = self.left();
        let right = self.right(grid[0].len());
        [[top_left, top, top_right], [left, Some(*self), right], [
            bottom_left, bottom, bottom_right,
        ]]
        .map(|a| a.map(|o| o.map(|p| (p, grid[p.y][p.x]))))
    }

    fn iter_positions(grid: &[Box<[u8]>]) -> impl Iterator<Item = (Self, u8)> {
        grid.iter()
            .enumerate()
            .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, &b)| (Self { x, y }, b)))
    }
}

fn count(
    f: impl Fn(Position, u8, &[Box<[u8]>], &mut HashSet<Position>) -> (usize, usize),
) -> usize {
    let farm = read_grid_bytes(12);
    let mut positions_used: HashSet<Position> = HashSet::new();
    let mut result = 0;
    for (p, b) in Position::iter_positions(&farm) {
        if positions_used.insert(p) {
            let (a, p) = f(p, b, &farm, &mut positions_used);
            result += a * p;
        }
    }
    result
}

pub fn day12_a() {
    let result = count(flood_a);
    println!("{result}");
}

pub fn day12_b() {
    let result = count(flood_b);
    println!("{result}");
}

fn flood_a(
    position: Position, plot: u8, farm: &[Box<[u8]>], used: &mut HashSet<Position>,
) -> (usize, usize) {
    let mut area = 1;
    let mut perimeter = 0;
    for o in position.neighbours(farm) {
        match o {
            Some((p, b)) if b == plot =>
                if used.insert(p) {
                    let (a, p) = flood_a(p, plot, farm, used);
                    area += a;
                    perimeter += p;
                },
            _ => perimeter += 1,
        }
    }
    (area, perimeter)
}

fn flood_b(
    position: Position, plot: u8, farm: &[Box<[u8]>], used: &mut HashSet<Position>,
) -> (usize, usize) {
    used.insert(position);
    let mut area = 1;
    let mut sides = 0;
    let [[top_left, top, top_right], [left, _, right], [bottom_left, bottom, bottom_right]] =
        position.neighbours_corners(farm);
    let checks = [
        (top, left, top_left),
        (right, top, top_right),
        (bottom, right, bottom_right),
        (left, bottom, bottom_left),
    ];
    for (main, neighbor, corner) in checks {
        match main {
            Some((p, b)) if b == plot =>
                if used.insert(p) {
                    let (a, s) = flood_b(p, plot, farm, used);
                    area += a;
                    sides += s;
                },
            _ =>
                sides += usize::from(
                    neighbor.is_none_or(|(_, b)| b != plot)
                        || corner.is_some_and(|(_, b)| b == plot),
                ),
        }
    }
    (area, sides)
}
