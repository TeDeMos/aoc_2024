use std::collections::HashMap;
use super::read_numbers;

fn count(limit: usize) -> usize {
    let mut numbers: HashMap<u64, usize> = read_numbers(11).into_iter().map(|v| (v, 1)).collect();
    for i in 0..limit {
        let mut numbers_next = HashMap::new();
        let mut add = |v, c| *numbers_next.entry(v).or_default() += c;
        for (n, c) in numbers {
            if n == 0 {
                add(1, c);
            } else {
                let digits = n.ilog10() + 1;
                if digits % 2 == 0 {
                    let split = 10u64.pow(digits / 2);
                    add(n / split, c);
                    add(n % split, c);
                } else {
                    add(n * 2024, c);
                }
            }
        }
        numbers = numbers_next;
    }
    numbers.into_values().sum()
}

pub fn day11_a() {
    let result = count(25);
    println!("{result}");
}

pub fn day11_b() {
    let result = count(75);
    println!("{result}");

}
