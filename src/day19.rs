use std::collections::{HashMap, HashSet};

use super::read_lines;

type Known = HashMap<Box<[u8]>, bool>;
type Counts = HashMap<Box<[u8]>, usize>;
type Towels = HashSet<Box<[u8]>>;

fn get_known() -> (Known, impl Iterator<Item = Box<[u8]>>) {
    let mut lines = read_lines(19);
    let towels = lines.next().unwrap().split(", ").map(|s| (s.as_bytes().into(), true)).collect();
    let rest = lines.skip(1).map(|s| s.into_boxed_str().into_boxed_bytes());
    (towels, rest)
}

fn get_counts() -> (Towels, impl Iterator<Item = Box<[u8]>>) {
    let mut lines = read_lines(19);
    let towels = lines.next().unwrap().split(", ").map(|s| s.as_bytes().into()).collect();
    let rest = lines.skip(1).map(|s| s.into_boxed_str().into_boxed_bytes());
    (towels, rest)
}

fn check(pattern: &[u8], known: &mut Known) -> bool {
    if let Some(&r) = known.get(pattern) {
        return r;
    }
    for i in 1..pattern.len() {
        if check(&pattern[..i], known) && check(&pattern[i..], known) {
            known.insert(pattern.into(), true);
            return true;
        }
    }
    known.insert(pattern.into(), false);
    false
}

fn count(pattern: &[u8], towels: &Towels, counts: &mut Counts) -> usize {
    if let Some(&r) = counts.get(pattern) {
        return r;
    }
    let mut possibilities = usize::from(towels.get(pattern).is_some());
    for i in 1..pattern.len() {
        if towels.contains(&pattern[..i]) {
            possibilities += count(&pattern[i..], towels, counts);
        }
    }
    counts.insert(pattern.into(), possibilities);
    possibilities
}

pub fn day19_a() {
    let (mut known, rest) = get_known();
    let result = rest.filter(|p| check(p, &mut known)).count();
    println!("{result}");
}

pub fn day19_b() {
    let (towels, rest) = get_counts();
    let mut counts = Counts::new();
    let result: usize = rest.map(|p| count(&p, &towels, &mut counts)).sum();
    println!("{result}");
}
