use std::collections::HashMap;

use super::read_lines;

fn get_values() -> (Vec<u32>, Vec<u32>) {
    read_lines(1)
        .map(|s| {
            let mut iter = s.split_whitespace().map(|s| s.parse::<u32>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .unzip()
}

pub fn day1_a() {
    let (mut left, mut right) = get_values();
    left.sort();
    right.sort();
    let result: u32 = left.iter().zip(right.iter()).map(|(l, r)| l.abs_diff(*r)).sum();
    println!("{result}");
}

pub fn day1_b() {
    let (left, right) = get_values();
    let mut counts = HashMap::new();
    right.iter().fold(&mut counts, |h, i| {
        *h.entry(i).or_default() += 1;
        h
    });
    let result: u32 = left.into_iter().map(|n| n * counts.get(&n).unwrap_or(&0)).sum();
    println!("{result}");
}
