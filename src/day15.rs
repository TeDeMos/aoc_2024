use std::collections::HashMap;

use super::{Vec2, read_lines};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl Move {
    fn parse(b: u8) -> Self {
        match b {
            b'^' => Self::Up,
            b'>' => Self::Right,
            b'v' => Self::Down,
            b'<' => Self::Left,
            _ => unreachable!(),
        }
    }

    const fn offset(self, mut vec: Vec2<usize>) -> Vec2<usize> {
        match self {
            Self::Up => vec.y -= 1,
            Self::Right => vec.x += 1,
            Self::Down => vec.y += 1,
            Self::Left => vec.x -= 1,
        }
        vec
    }
}

fn get_data(thick: bool) -> (World, impl Iterator<Item = Move>) {
    let mut lines = read_lines(15);
    let world = if thick { World::new_thick(&mut lines) } else { World::new_thin(&mut lines) };
    let moves = lines.flat_map(|s| s.into_bytes().into_iter().map(Move::parse));
    (world, moves)
}

struct World {
    grid: Box<[Box<[u8]>]>,
    robot: Vec2<usize>,
}

impl World {
    fn new(lines: &mut impl Iterator<Item = String>, row: impl Fn(String) -> Box<[u8]>) -> Self {
        let mut grid: Box<[Box<[u8]>]> =
            lines.by_ref().take_while(|s| !s.is_empty()).map(row).collect();
        let robot = grid
            .iter()
            .enumerate()
            .find_map(|(y, r)| r.iter().position(|&b| b == b'@').map(|x| Vec2 { x, y }))
            .unwrap();
        grid[robot.y][robot.x] = b'.';
        Self { grid, robot }
    }

    fn new_thin(lines: &mut impl Iterator<Item = String>) -> Self {
        Self::new(lines, |s| s.into_boxed_str().into_boxed_bytes())
    }

    fn new_thick(lines: &mut impl Iterator<Item = String>) -> Self {
        Self::new(lines, |s| {
            s.bytes()
                .flat_map(|b| match b {
                    b'@' => [b'@', b'.'],
                    b'O' => [b'[', b']'],
                    b => [b, b],
                })
                .collect()
        })
    }

    fn simulate_thin(&mut self, moves: impl Iterator<Item = Move>) {
        for m in moves {
            let first = m.offset(self.robot);
            match self.grid[first.y][first.x] {
                b'.' => self.robot = first,
                b'O' => self.push_thin(m, first),
                b'#' => {},
                _ => unreachable!(),
            }
        }
    }

    fn simulate_thick(&mut self, moves: impl Iterator<Item = Move>) {
        for m in moves {
            let first = m.offset(self.robot);
            match self.grid[first.y][first.x] {
                b'.' => self.robot = first,
                b @ (b'[' | b']') => match m {
                    Move::Left | Move::Right => self.push_thick_horizontal(m, first),
                    Move::Up | Move::Down => self.push_thick_vertical(m, first, b),
                },
                b'#' => {},
                _ => unreachable!(),
            }
        }
    }

    fn push_thin(&mut self, m: Move, first: Vec2<usize>) {
        let mut last = first;
        loop {
            last = m.offset(last);
            match self.grid[last.y][last.x] {
                b'.' => {
                    self.grid[last.y][last.x] = b'O';
                    self.grid[first.y][first.x] = b'.';
                    self.robot = first;
                    break;
                },
                b'#' => break,
                b'O' => continue,
                _ => unreachable!(),
            }
        }
    }

    fn push_thick_horizontal(&mut self, m: Move, first: Vec2<usize>) {
        let mut last = first;
        loop {
            last = m.offset(m.offset(last));
            match self.grid[last.y][last.x] {
                b'.' => {
                    if m == Move::Left {
                        self.grid[first.y][last.x..=first.x].rotate_left(1);
                    } else {
                        self.grid[first.y][first.x..=last.x].rotate_right(1);
                    }
                    self.robot = first;
                    break;
                },
                b'[' | b']' => continue,
                b'#' => break,
                _ => unreachable!(),
            }
        }
    }

    fn push_thick_vertical(&mut self, m: Move, first: Vec2<usize>, b: u8) {
        let mut pushes = Vec::new();
        let mut current = first;
        let other = if b == b'[' { first.x + 1 } else { first.x - 1 };
        let mut potential = HashMap::from([(first.x, first.y), (other, first.y)]);
        while !potential.is_empty() {
            current = m.offset(current);
            let mut to_delete = Vec::new();
            let mut to_add = Vec::new();
            for (&column, &start) in &potential {
                match self.grid[current.y][column] {
                    b'.' => {
                        pushes.push((column, start, current.y));
                        to_delete.push(column);
                    },
                    b'#' => return,
                    b @ (b'[' | b']') => {
                        let other = if b == b'[' { column + 1 } else { column - 1 };
                        if !potential.contains_key(&other) {
                            to_add.push(other);
                        }
                    },
                    _ => unreachable!(),
                }
            }
            for c in to_delete {
                potential.remove(&c);
            }
            for c in to_add {
                potential.insert(c, current.y);
            }
        }
        pushes.into_iter().for_each(|(x, s, e)| self.move_vertical(x, s, e, m));
        self.robot = first;
    }

    fn move_vertical(&mut self, x: usize, y_start: usize, y_end: usize, m: Move) {
        if m == Move::Up {
            for y in y_end..y_start {
                self.grid[y][x] = self.grid[y + 1][x];
            }
        } else {
            for y in (y_start + 1..=y_end).rev() {
                self.grid[y][x] = self.grid[y - 1][x];
            }
        }
        self.grid[y_start][x] = b'.';
    }

    fn count(self, b: u8) -> usize {
        self.grid
            .into_iter()
            .enumerate()
            .flat_map(|(y, r)| {
                r.into_iter().enumerate().filter(|&(_, c)| c == b).map(move |(x, _)| 100 * y + x)
            })
            .sum()
    }
}

pub fn day15_a() {
    let (mut world, moves) = get_data(false);
    world.simulate_thin(moves);
    let result = world.count(b'O');
    println!("{result}");
}

pub fn day15_b() {
    let (mut world, moves) = get_data(true);
    world.simulate_thick(moves);
    let result = world.count(b'[');
    println!("{result}");
}
