use std::cmp::Ordering;

use super::read_lines;

enum Mistake {
    Before,
    Now(u32),
    After,
}

impl Mistake {
    fn check(&mut self, l: u32, r: u32, order: Ordering) -> bool {
        match self {
            Self::Before => {
                if !test_values(l, r, order) {
                    *self = Self::Now(l);
                }
                true
            },
            Self::Now(b) =>
                if test_values(*b, r, order) {
                    *self = Self::After;
                    true
                } else {
                    false
                },
            Self::After => test_values(l, r, order),
        }
    }
}

fn without_nth(iter: impl Iterator<Item = u32>, n: usize) -> impl Iterator<Item = u32> {
    iter.enumerate().filter_map(move |(i, x)| (i != n).then_some(x))
}

fn test_values(l: u32, r: u32, order: Ordering) -> bool {
    (1..=3).contains(&l.abs_diff(r)) && l.cmp(&r) == order
}

fn check_iter_one_mistake(iter: impl Iterator<Item = u32> + Clone) -> bool {
    let mut test_a = iter.clone().peekable();
    let first = test_a.next().unwrap();
    let second = test_a.peek().unwrap();
    if (1..=3).contains(&first.abs_diff(*second)) {
        let order = first.cmp(second);
        let mut mistake = Mistake::Before;
        if test_a.map_windows(|&[l, r]| mistake.check(l, r, order)).all(|b| b) {
            return true;
        }
    }
    let mut test_b = iter.clone();
    test_b.next();
    if check_iter(test_b) {
        return true;
    }
    let test_c = without_nth(iter, 1);
    check_iter(test_c)
}

fn check_iter(iter: impl Iterator<Item = u32>) -> bool {
    let mut iter = iter.peekable();
    let first = iter.next().unwrap();
    let second = iter.peek().unwrap();
    if (1..=3).contains(&first.abs_diff(*second)) {
        let order = first.cmp(second);
        iter.map_windows(|&[l, r]| test_values(l, r, order)).all(|b| b)
    } else {
        false
    }
}

pub fn day2_b() {
    let result = read_lines(2)
        .filter(|s| check_iter_one_mistake(s.split_whitespace().map(|s| s.parse::<u32>().unwrap())))
        .count();
    println!("{result}");
}

pub fn day2_a() {
    let result = read_lines(2)
        .filter(|s| check_iter(s.split_whitespace().map(|s| s.parse::<u32>().unwrap())))
        .count();
    println!("{result}");
}
