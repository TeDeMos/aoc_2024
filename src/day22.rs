use std::collections::HashMap;

use super::read_lines;

struct Memo {
    memo: HashMap<isize, isize>,
}

struct NumberIter<'a> {
    memo: &'a mut Memo,
    current: isize,
    n: usize,
}

impl Iterator for NumberIter<'_> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        (self.n <= 2000).then(|| {
            let result = self.current;
            self.n += 1;
            self.current = self.memo.next(self.current);
            result
        })
    }
}

impl Memo {
    fn new() -> Self { Self { memo: HashMap::new() } }

    fn iter(&mut self, start: isize) -> NumberIter {
        NumberIter { memo: self, current: start, n: 0 }
    }

    fn next(&mut self, current: isize) -> isize {
        *self.memo.entry(current).or_insert_with(|| Self::step(current))
    }

    const fn mix_prune(current: isize, other: isize) -> isize { (current ^ other) & 0x00ff_ffff }

    const fn step(mut current: isize) -> isize {
        current = Self::mix_prune(current, current << 6);
        current = Self::mix_prune(current, current >> 5);
        current = Self::mix_prune(current, current << 11);
        current
    }
}

struct ChangesCounter {
    counts: HashMap<[isize; 4], isize>,
}

impl ChangesCounter {
    fn new() -> Self { Self { counts: HashMap::new() } }

    fn consume(&mut self, iter: NumberIter) {
        let mut current = HashMap::new();
        iter.map(|p| p.rem_euclid(10))
            .map_windows(|&[l, r]| (r - l, r))
            .map_windows(|&[(a, _), (b, _), (c, _), (d, p)]| {
                current.entry([a, b, c, d]).or_insert(p);
            })
            .last();
        for (k, v) in current {
            *self.counts.entry(k).or_default() += v;
        }
    }

    fn find_best(self) -> ([isize; 4], isize) { self.counts.into_iter().max_by_key(|(_, v)| *v).unwrap() }
}

pub fn day22_a() {
    let mut memo = Memo::new();
    let result: isize = read_lines(22).map(|s| memo.iter(s.parse().unwrap()).last().unwrap()).sum();
    println!("{result}");
}

pub fn day22_b() { 
    let mut changes = ChangesCounter::new();
    let mut memo = Memo::new();
    read_lines(22).for_each(|s| changes.consume(memo.iter(s.parse().unwrap())));
    let (a, result) = changes.find_best();
    println!("{a:?}, {result}");
}
