use std::collections::{HashMap, HashSet};

use super::read_grid_bytes;

type Grid<T> = Box<[Box<[T]>]>;
type Path = Box<[(usize, usize)]>;

fn get_maze() -> (Grid<u8>, Grid<usize>, Path) {
    let grid = read_grid_bytes(20);
    let (mut x, mut y) = grid
        .iter()
        .enumerate()
        .find_map(|(y, r)| r.iter().position(|&b| b == b'S').map(|x| (x, y)))
        .unwrap();
    let mut scores: Box<[Box<[usize]>]> =
        grid.iter().map(|r| r.iter().map(|_| 0).collect()).collect();
    let mut path = Vec::from([(x, y)]);
    while grid[y][x] != b'E' {
        let current = scores[y][x];
        for (x_next, y_next) in [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
            if grid[y_next][x_next] != b'#' && scores[y_next][x_next] == 0 {
                (x, y) = (x_next, y_next);
                break;
            }
        }
        scores[y][x] = current + 1;
        path.push((x, y));
    }
    (grid, scores, path.into_boxed_slice())
}

fn get_possible_saves(grid: &Grid<u8>, scores: &Grid<usize>, path: Path, n: usize) -> usize {
    let mut possible_saves = 0;
    for (x, y) in path {
        let current = scores[y][x];
        let mut check = |x: usize, y: usize, n| {
            possible_saves += usize::from(scores[y][x].saturating_sub(current) >= 100 + n);
        };
        for o in 1..=n {
            if x + o < grid[0].len() {
                check(x + o, y, o);
            }
            if let Some(x) = x.checked_sub(o) {
                check(x, y, o);
            }
            if y + o < grid.len() {
                check(x, y + o, o);
                for o2 in 1..=n - o {
                    if x + o2 < grid[0].len() {
                        check(x + o2, y + o, o + o2);
                    }
                    if let Some(x) = x.checked_sub(o2) {
                        check(x, y + o, o + o2);
                    }
                }
            }
            if let Some(y) = y.checked_sub(o) {
                check(x, y, o);
                for o2 in 1..=n - o {
                    if x + o2 < grid[0].len() {
                        check(x + o2, y, o + o2);
                    }
                    if let Some(x) = x.checked_sub(o2) {
                        check(x, y, o + o2);
                    }
                }
            }
        }
    }
    possible_saves
}

pub fn day20_a() {
    let (grid, scores, start) = get_maze();
    let result = get_possible_saves(&grid, &scores, start, 2);
    println!("{result}");
}

pub fn day20_b() {
    let (grid, scores, start) = get_maze();
    let result = get_possible_saves(&grid, &scores, start, 20);
    println!("{result}");
}
