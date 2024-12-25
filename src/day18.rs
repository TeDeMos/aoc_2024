use std::collections::HashSet;

use super::{SplitOnceArr as _, read_lines};

const SIZE: usize = 71;

fn display(grid: &[[bool; SIZE]; SIZE]) {
    let chars = grid.each_ref().map(|r| r.map(|b| if b { b'#' } else { b'.' }));
    let rows = chars.each_ref().map(|r| std::str::from_utf8(r).unwrap());
    let display = rows.join("\n");
    println!("{display}");
}

fn get_data() -> ([[bool; SIZE]; SIZE], impl Iterator<Item = [usize; 2]>) {
    let mut grid = [[false; SIZE]; SIZE];
    let mut coords = read_lines(18)
        .map(|l| l.split_once_arr(',').unwrap().map(|n| n.parse::<usize>().unwrap()));
    coords.by_ref().take(1024) .for_each(|[x, y]| grid[y][x] = true);
    (grid, coords)
}

fn check_grid(grid: &[[bool; SIZE]; SIZE]) -> Option<u16> {
    let mut scores = [[u16::MAX; SIZE]; SIZE];
    scores[0][0] = 0;
    let mut to_check = HashSet::from([(0, 0)]);
    while !to_check.is_empty() {
        let mut next = HashSet::new();
        for (x, y) in to_check {
            let s = scores[y][x];
            if x > 0 && !grid[y][x - 1] && scores[y][x - 1] > s + 1 {
                scores[y][x - 1] = s + 1;
                next.insert((x - 1, y));
            }
            if y > 0 && !grid[y - 1][x] && scores[y - 1][x] > s + 1 {
                scores[y - 1][x] = s + 1;
                next.insert((x, y - 1));
            }
            if x < SIZE - 1 && !grid[y][x + 1] && scores[y][x + 1] > s + 1 {
                scores[y][x + 1] = s + 1;
                next.insert((x + 1, y));
            }
            if y < SIZE - 1 && !grid[y + 1][x] && scores[y + 1][x] > s + 1 {
                scores[y + 1][x] = s + 1;
                next.insert((x, y + 1));
            }
        }
        to_check = next;
    }
    let result = scores[SIZE - 1][SIZE - 1];
    (result != u16::MAX).then_some(result)
}

pub fn day18_a() { 
    let (grid, _) = get_data();
    let result = check_grid(&grid).unwrap(); 
    println!("{result}");
}

pub fn day18_b() {
    let (mut grid, mut rest) = get_data();
    loop {
        let [x, y] = rest.next().unwrap();
        grid[y][x] = true;
        if check_grid(&grid).is_none() {
            println!("{x},{y}");
            return;
        }
    }
}
