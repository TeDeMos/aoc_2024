use std::cmp::Ordering;
use std::collections::HashSet;

use super::{Vec2, read_grid_bytes};

#[repr(usize)]
#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    East = 0,
    North = 1,
    West = 2,
    South = 3,
}

impl Direction {
    const fn clockwise(self) -> Self {
        match self {
            Self::East => Self::North,
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
        }
    }

    const fn counterclockwise(self) -> Self {
        match self {
            Self::East => Self::South,
            Self::North => Self::East,
            Self::West => Self::North,
            Self::South => Self::West,
        }
    }

    const fn forward(self, position: Vec2<usize>) -> Vec2<usize> {
        match self {
            Self::East => Vec2 { x: position.x - 1, y: position.y },
            Self::North => Vec2 { x: position.x, y: position.y - 1 },
            Self::West => Vec2 { x: position.x + 1, y: position.y },
            Self::South => Vec2 { x: position.x, y: position.y + 1 },
        }
    }
}

struct CellScores {
    direction_scores: [Option<DirectionScore>; 4],
}

impl CellScores {
    fn set(&mut self, direction: Direction, previous: PosDir, score: usize) -> bool {
        let current = &mut self.direction_scores[direction as usize];
        if let Some(p) = current.as_mut() {
            match p.score.cmp(&score) {
                Ordering::Greater => {
                    p.previous.clear();
                    p.previous.push(previous);
                    p.score = score;
                    true
                },
                Ordering::Equal => {
                    p.previous.push(previous);
                    false
                },
                Ordering::Less => false,
            }
        } else {
            *current = Some(DirectionScore::new(vec![previous], score));
            true
        }
    }

    fn set_start(&mut self, direction: Direction) {
        self.direction_scores[direction as usize] = Some(DirectionScore::new(Vec::new(), 0));
    }

    const fn get(&self, direction: Direction) -> Option<&DirectionScore> {
        self.direction_scores[direction as usize].as_ref()
    }

    fn get_min(&self) -> (usize, Vec<PosDir>) {
        let min_score =
            self.direction_scores.iter().flatten().min_by_key(|s| s.score).unwrap().score;
        let previous: Vec<_> = self
            .direction_scores
            .iter()
            .flatten()
            .filter(|s| s.score == min_score)
            .flat_map(|s| s.previous.iter())
            .copied()
            .collect();
        (min_score, previous)
    }
}

impl CellScores {
    const fn new() -> Self { Self { direction_scores: [const { None }; 4] } }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct PosDir {
    cell: Vec2<usize>,
    direction: Direction,
}

impl PosDir {
    const fn new(cell: Vec2<usize>, direction: Direction) -> Self { Self { cell, direction } }

    const fn clockwise(self) -> Self {
        Self { cell: self.cell, direction: self.direction.clockwise() }
    }

    const fn counterclockwise(self) -> Self {
        Self { cell: self.cell, direction: self.direction.counterclockwise() }
    }

    const fn forward(self) -> Self {
        Self { cell: self.direction.forward(self.cell), direction: self.direction }
    }
}

struct DirectionScore {
    previous: Vec<PosDir>,
    score: usize,
}

impl DirectionScore {
    const fn new(previous: Vec<PosDir>, score: usize) -> Self { Self { previous, score } }
}

struct Maze {
    grid: Box<[Box<[u8]>]>,
    scores: Box<[Box<[CellScores]>]>,
    start: Vec2<usize>,
    end: Vec2<usize>,
}

impl Maze {
    fn new(grid: Box<[Box<[u8]>]>) -> Self {
        let mut start = Vec2 { x: 0, y: 0 };
        let mut end = Vec2 { x: 0, y: 0 };
        let scores = grid
            .iter()
            .enumerate()
            .map(|(y, r)| {
                r.iter()
                    .enumerate()
                    .inspect(|&(x, b)| match b {
                        b'S' => start = Vec2 { x, y },
                        b'E' => end = Vec2 { x, y },
                        _ => {},
                    })
                    .map(|_| CellScores::new())
                    .collect()
            })
            .collect();
        Self { grid, scores, start, end }
    }

    const fn get_score(&self, pos_dir: PosDir) -> Option<&DirectionScore> {
        self.scores[pos_dir.cell.y][pos_dir.cell.x].get(pos_dir.direction)
    }

    fn set_score(&mut self, pos_dir: PosDir, from: PosDir, score: usize) -> bool {
        self.scores[pos_dir.cell.y][pos_dir.cell.x].set(pos_dir.direction, from, score)
    }

    fn calculate(&mut self) {
        self.scores[self.start.y][self.start.x].set_start(Direction::East);
        let mut new = HashSet::from([PosDir::new(self.start, Direction::East)]);
        while !new.is_empty() {
            let mut next = HashSet::new();
            new.into_iter().for_each(|p| self.flood(p, &mut next));
            new = next;
        }
    }

    fn get_result_a(&self) -> usize { self.scores[self.end.y][self.end.x].get_min().0 }

    fn get_result_b(&self) -> usize {
        let previous = self.scores[self.end.y][self.end.x].get_min().1;
        let mut positions = HashSet::from([self.end]);
        let mut pos_dirs = HashSet::new();
        self.count_optimal(&previous, &mut pos_dirs);
        positions.extend(pos_dirs.into_iter().map(|p| p.cell));
        positions.len()
    }

    fn count_optimal(&self, to_check: &[PosDir], results: &mut HashSet<PosDir>) {
        for &p in to_check {
            if results.insert(p) {
                self.count_optimal(&self.get_score(p).unwrap().previous, results);
            }
        }
    }

    fn flood(&mut self, pos_dir: PosDir, to_check: &mut HashSet<PosDir>) {
        let score = self.get_score(pos_dir).unwrap().score;
        for p in [pos_dir.clockwise(), pos_dir.counterclockwise()] {
            if self.set_score(p, pos_dir, score + 1000) {
                to_check.insert(p);
            }
        }
        let p = pos_dir.forward();
        if self.grid[p.cell.y][p.cell.x] != b'#' && self.set_score(p, pos_dir, score + 1) {
            to_check.insert(p);
        }
    }
}

pub fn day16_a() {
    let grid = read_grid_bytes(16);
    let mut maze = Maze::new(grid);
    maze.calculate();
    let result = maze.get_result_a();
    println!("{result}");
}

pub fn day16_b() {
    let grid = read_grid_bytes(16);
    let mut maze = Maze::new(grid);
    maze.calculate();
    let result = maze.get_result_b();
    println!("{result}");
}
