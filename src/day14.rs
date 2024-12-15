use std::cmp::Ordering;
use std::io;

use super::{SplitOnceArr as _, Vec2, read_lines};

struct Robot {
    position: Vec2<i64>,
    velocity: Vec2<i64>,
}

impl Robot {
    const AREA_SIZE: Vec2<i64> = Vec2 { x: 101, y: 103 };

    // const AREA_SIZE: Vec2<i64> = Vec2 { x: 11, y: 7 };

    fn parse(line: &str) -> Self {
        line.split_once_arr(' ')
            .unwrap()
            .map(|s| {
                s.split_once('=')
                    .unwrap()
                    .1
                    .split_once_arr(',')
                    .unwrap()
                    .map(|s| s.parse().unwrap())
                    .into()
            })
            .into()
    }

    fn step(&mut self, steps: i64) {
        self.position += self.velocity * steps;
        self.position = self.position.rem_euclid(Self::AREA_SIZE);
    }

    fn quadrant(&self) -> Option<usize> {
        let limit = Self::AREA_SIZE / 2;
        let cmp: [_; 2] = self.position.cmp_each(&limit).into();
        match cmp {
            [_, Ordering::Equal] | [Ordering::Equal, _] => None,
            [Ordering::Less, Ordering::Less] => Some(0),
            [Ordering::Less, Ordering::Greater] => Some(1),
            [Ordering::Greater, Ordering::Less] => Some(2),
            [Ordering::Greater, Ordering::Greater] => Some(3),
        }
    }

    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn display_robots(robots: &[Self], min_neighbours: usize) -> bool {
        let mut grid = [[b'.'; Self::AREA_SIZE.y as _]; Self::AREA_SIZE.x as _];
        for r in robots {
            match &mut grid[r.position.x as usize][r.position.y as usize] {
                b @ b'.' => *b = b'1',
                b @ b'1'..=b'8' => *b += 1,
                b @ b'9' => *b = b'+',
                _ => {},
            }
        }
        let mut neighbours = 0;
        for i in 1..Self::AREA_SIZE.x as usize - 1 {
            for j in 1..Self::AREA_SIZE.y as usize - 1 {
                if grid[i][j] != b'.' {
                    neighbours += usize::from(grid[i - 1][j] != b'.');
                    neighbours += usize::from(grid[i + 1][j] != b'.');
                    neighbours += usize::from(grid[i][j - 1] != b'.');
                    neighbours += usize::from(grid[i][j + 1] != b'.');
                }
            }
        }
        if neighbours >= min_neighbours {
            let v: Vec<_> = grid.iter().map(|b| std::str::from_utf8(b).unwrap()).collect();
            let content = v.join("\n");
            println!("{content}");
            true
        } else {
            false
        }
    }
}

impl From<[Vec2<i64>; 2]> for Robot {
    fn from([position, velocity]: [Vec2<i64>; 2]) -> Self { Self { position, velocity } }
}

fn get_robots() -> impl Iterator<Item = Robot> { read_lines(14).map(|s| Robot::parse(&s)) }

pub fn day14_a() {
    let mut quadrant_counts = [0; 4];
    get_robots()
        .filter_map(|mut r| {
            r.step(100);
            r.quadrant()
        })
        .for_each(|q| quadrant_counts[q] += 1);
    let result: i32 = quadrant_counts.into_iter().product();
    println!("{result}");
}

pub fn day14_b() {
    let mut robots: Vec<_> = get_robots().collect();
    let mut step = 0;
    let stdin = io::stdin();
    let mut content = String::new();
    loop {
        if Robot::display_robots(&robots, 500) {
            println!("Step: {step}");
            stdin.read_line(&mut content).unwrap();
        }
        if !content.trim().is_empty() {
            break;
        }
        robots.iter_mut().for_each(|r| r.step(1));
        step += 1;
    }
}
