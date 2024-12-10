use std::collections::{HashMap, HashSet};

use super::{IterPairs as _, read_grid_bytes};

#[expect(clippy::type_complexity)]
fn get_antennas() -> (HashMap<u8, Vec<(isize, isize)>>, impl Fn(isize, isize) -> bool) {
    let grid = read_grid_bytes(8);
    let mut antennas: HashMap<u8, Vec<(isize, isize)>> = HashMap::new();
    for (y, r) in grid.iter().enumerate() {
        for (x, b) in r.iter().enumerate() {
            if b.is_ascii_alphanumeric() {
                #[expect(clippy::cast_possible_wrap)]
                antennas.entry(*b).or_default().push((x as _, y as _));
            }
        }
    }
    #[expect(clippy::cast_possible_wrap)]
    let check_range =
        move |x, y| (0..grid[0].len() as _).contains(&x) && (0..grid.len() as _).contains(&y);
    (antennas, check_range)
}

pub fn day8_a() {
    let (antennas, check_range) = get_antennas();
    let mut positions = HashSet::new();
    for v in antennas.into_values() {
        for (&(x1, y1), &(x2, y2)) in v.iter_pairs() {
            let (xd, yd) = (x2 - x1, y2 - y1);
            let (xa, ya) = (x1 - xd, y1 - yd);
            let (xb, yb) = (x2 + xd, y2 + yd);
            if check_range(xa, ya) {
                positions.insert((xa, ya));
            }
            if check_range(xb, yb) {
                positions.insert((xb, yb));
            }
        }
    }
    let result = positions.len();
    println!("{result}");
}

pub fn day8_b() {
    let (antennas, check_range) = get_antennas();
    let mut positions = HashSet::new();
    for v in antennas.into_values() {
        for (&(x1, y1), &(x2, y2)) in v.iter_pairs() {
            let (xd, yd) = (x2 - x1, y2 - y1);
            let (mut xp, mut yp) = (x1, y1);
            while check_range(xp, yp) {
                positions.insert((xp, yp));
                (xp, yp) = (xp - xd, yp - yd);
            }
            (xp, yp) = (x2, y2);
            while check_range(xp, yp) {
                positions.insert((xp, yp));
                (xp, yp) = (xp + xd, yp + yd);
            }
        }
    }
    let result = positions.len();
    println!("{result}");
}