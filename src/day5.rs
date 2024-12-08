use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use super::{read_lines, SplitOnceArr};

pub fn get_rules() -> (HashMap<usize, HashSet<usize>>, impl Iterator<Item = Vec<usize>>) {
    let mut lines = read_lines(5);
    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    lines.by_ref().take_while(|l| !l.is_empty()).for_each(|i| {
        let [left, right] = i.split_once_arr('|').unwrap().map(|x| x.parse().unwrap());
        rules.entry(right).or_default().insert(left);
    });
    (rules, lines.map(|s| s.split(',').map(|s| s.parse().unwrap()).collect()))
}

fn check_order(numbers: &[usize], rules: &HashMap<usize, HashSet<usize>>) -> bool {
    let mut available = [true; 100];
    for &n in numbers {
        if available[n] {
            rules.get(&n).into_iter().flat_map(|v| v.iter()).for_each(|&i| available[i] = false);
        } else {
            return false;
        }
    }
    true
}

pub fn day5_a() {
    let (rules, lines) = get_rules();
    let result: usize =
        lines.filter_map(|l| check_order(&l, &rules).then_some(l[l.len() / 2])).sum();
    println!("{result}");
}

pub fn day5_b() {
    let (rules, lines) = get_rules();
    let order = |l: &'_ usize, r: &'_ usize| {
        rules
            .get(l)
            .and_then(|x| x.contains(r).then_some(Ordering::Less))
            .or_else(|| rules.get(r).and_then(|x| x.contains(l).then_some(Ordering::Greater)))
            .unwrap_or(Ordering::Equal)
    };
    let result: usize = lines.filter(|l| !check_order(l, &rules)).map(|mut l| {
        l.sort_by(order);
        l[l.len() / 2]
    }).sum();
    println!("{result}");
}
