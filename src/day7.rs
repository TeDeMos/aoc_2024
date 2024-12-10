use super::read_lines;

fn get_values() -> impl Iterator<Item = (u64, Vec<u64>)> {
    read_lines(7).map(|s| {
        let (l, r) = s.split_once(": ").unwrap();
        let target = l.parse().unwrap();
        let numbers = r.split_whitespace().map(|s| s.parse().unwrap()).collect();
        (target, numbers)
    })
}

pub fn day7_a() {
    let result: u64 = get_values().filter_map(|(t, n)| test_a(t, &n).then_some(t)).sum();
    println!("{result}");
}

pub fn day7_b() {
    let result: u64 = get_values().filter_map(|(t, n)| test_b(t, &n).then_some(t)).sum();
    println!("{result}");
}

fn test_a(target: u64, numbers: &[u64]) -> bool {
    let [first, rest @ ..] = numbers else { return false };
    test_a_r(target, *first, rest)
}

fn test_a_r(target: u64, total: u64, remainder: &[u64]) -> bool {
    let [first, rest @ ..] = remainder else { return total == target };
    test_a_r(target, total * *first, rest) || test_a_r(target, total + *first, rest)
}

const fn concat(l: u64, r: u64) -> u64 {
    l * 10u64.pow(r.ilog10() + 1) + r
}

fn test_b(target: u64, numbers: &[u64]) -> bool {
    let [first, rest @ ..] = numbers else { return false };
    test_b_r(target, *first, rest)
}

fn test_b_r(target: u64, total: u64, remainder: &[u64]) -> bool {
    let [first, rest @ ..] = remainder else { return total == target };
    test_b_r(target, total * *first, rest) || test_b_r(target, total + *first, rest) || test_b_r(target, concat(total, *first), rest)
}
