use std::collections::HashSet;
use super::read_grid_bytes;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const fn turn(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    direction: Direction
}

impl Position {
    const fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self { x, y, direction }
    }
    
    fn forward(&self, room: &[Box<[u8]>], direction: Direction) -> Option<(usize, usize)> {
        match direction {
            Direction::Up => (self.y > 0).then(|| (self.x, self.y - 1)),
            Direction::Right => (self.x + 1 < room[0].len()).then(|| (self.x + 1, self.y)),
            Direction::Down => (self.y + 1 < room.len()).then(|| (self.x, self.y + 1)),
            Direction::Left => (self.x > 0).then(|| (self.x - 1, self.y)),
        }
    }
    
    fn next_position(&self, room: &[Box<[u8]>]) -> Option<Self> {
        let mut next_direction = self.direction;
        loop {
            let (next_x, next_y) = self.forward(room, next_direction)?;
            if room[next_y][next_x] == b'#' {
                next_direction = next_direction.turn();
            } else {
                return Some(Self::new(next_x, next_y, next_direction))
            }
        }
    }
    
    fn step(&mut self, room: &[Box<[u8]>]) -> bool {
        self.next_position(room).inspect(|p| *self = *p).is_some()
    }
}

struct TestGuard<'a> {
    position: Position,
    visited: HashSet<Position>,
    room: &'a [Box<[u8]>]
}

impl<'a> TestGuard<'a> {
    fn new(guard: &'a Guard) -> Self {
        Self { position: guard.position, visited: HashSet::new(), room: &guard.room }
    }
    
    fn run(mut self) -> bool {
        self.visited.insert(self.position);
        while self.position.step(self.room) {
            if !self.visited.insert(self.position) {
                return true
            }
        }
        false
    }
}

struct Guard {
    position: Position,
    counter: usize,
    room: Box<[Box<[u8]>]>,
}

impl Guard {
    fn new(room: Box<[Box<[u8]>]>) -> Self {
        let (x, y) = room
            .iter()
            .enumerate()
            .find_map(|(y, r)| r.iter().position(|&b| b == b'^').map(|x| (x, y)))
            .unwrap();
        Self { position: Position::new(x, y, Direction::Up), counter: 0, room }
    }
    
    fn count_unique(mut self) -> usize {
        self.room[self.position.y][self.position.x] = b'#';
        self.counter += 1;
        while self.position.step(&self.room) {
            if self.room[self.position.y][self.position.x] != b'+' {
                self.room[self.position.y][self.position.x] = b'+';
                self.counter += 1;
            }
        }
        self.counter
    }
    
    fn count_possible_loops(mut self) -> usize {
        while let Some(next) = self.position.next_position(&self.room) {
            if self.room[next.y][next.x] != b'+' {
                self.room[next.y][next.x] = b'#';
                let test = TestGuard::new(&self);
                self.counter += usize::from(test.run());
                self.room[next.y][next.x] = b'+';
            }
            self.position = next;
        }
        self.counter
    }
}

pub fn day6_a() {
    let room = read_grid_bytes(6);
    let guard = Guard::new(room);
    let result = guard.count_unique();
    println!("{result}");
}

pub fn day6_b() {
    let room = read_grid_bytes(6);
    let guard = Guard::new(room);
    let result = guard.count_possible_loops();
    println!("{result}");
}
