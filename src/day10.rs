use std::collections::{HashMap, HashSet};
use std::mem;
use super::read_grid_digits;

fn directions(x: usize, y: usize, mountain: &[Box<[u8]>]) -> [Option<(usize, usize)>; 4] {
    [
        (y > 0).then(|| (x, y - 1)),
        (x + 1 < mountain[0].len()).then(|| (x + 1, y)),
        (y + 1 < mountain.len()).then(|| (x, y + 1)),
        (x > 0).then(|| (x - 1, y)),
    ]
}

fn count<T: Default>(
    init: impl Fn(usize, usize) -> T, sum: impl Fn(&mut T, &T), value: impl Fn(T) -> usize,
) -> usize {
    let mountain = read_grid_digits(10);
    let mut positions = HashMap::new();
    for (y, r) in mountain.iter().enumerate() {
        for (x, b) in r.iter().enumerate() {
            if *b == 0 {
                positions.insert((x, y), init(x, y));
            }
        }
    }
    let mut positions_next = HashMap::new();
    for next in 1..=9 {
        for ((x, y), v) in positions {
            for c in directions(x, y, &mountain) {
                if let Some((x_next, y_next)) = c && mountain[y_next][x_next] == next {
                    sum(positions_next.entry((x_next, y_next)).or_default(), &v);
                }
            }
        }
        positions = mem::take(&mut positions_next);
    }
    positions.into_values().map(value).sum()
}

pub fn day10_b() {
    #[expect(clippy::cast_sign_loss)]
    let result = count(|_, _| 1, |v1, v2| *v1 += *v2, |v| v as _);
    println!("{result}");
}

pub fn day10_a() {
    let result = count(|x, y| HashSet::from([50 * y + x]), |v1, v2| v1.extend(v2), |v| v.len());
    println!("{result}");
}
