use std::borrow::Cow;

use super::read_lines;

#[derive(Debug)]
struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    instructions: Vec<u8>,
    pointer: usize,
    result: Vec<u8>,
}

impl Computer {
    fn get_literal(&self) -> i64 { self.instructions[self.pointer + 1].into() }

    fn get_combo(&self) -> i64 {
        #[expect(clippy::match_on_vec_items)]
        match self.instructions[self.pointer + 1] {
            o @ 0..=3 => o.into(),
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => todo!(),
            _ => unreachable!(),
        }
    }

    fn divide(&self) -> i64 {
        let numerator = self.reg_a;
        let power = self.get_combo();
        if power > 64 {
            0
        } else {
            let denominator = 1 << power;
            numerator / denominator
        }
    }

    fn adv(&mut self) {
        self.reg_a = self.divide();
        self.pointer += 2;
    }

    fn bxl(&mut self) {
        let left = self.reg_b;
        let right = self.get_literal();
        self.reg_b = left ^ right;
        self.pointer += 2;
    }

    fn bst(&mut self) {
        self.reg_b = self.get_combo() & 0b111;
        self.pointer += 2;
    }

    fn jnz(&mut self) {
        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        if self.reg_a == 0 {
            self.pointer += 2;
        } else {
            self.pointer = self.get_literal() as usize;
        }
    }

    fn bxc(&mut self) {
        self.reg_b ^= self.reg_c;
        self.pointer += 2;
    }

    fn out(&mut self) -> bool {
        #[expect(clippy::cast_sign_loss)]
        self.result.push((self.get_combo() & 0b111) as u8);
        self.pointer += 2;
        self.result == self.instructions[0..self.result.len()]
    }

    fn bdv(&mut self) {
        self.reg_b = self.divide();
        self.pointer += 2;
    }

    fn cdv(&mut self) {
        self.reg_c = self.divide();
        self.pointer += 2;
    }

    fn run(&mut self, finish: bool) -> bool {
        while let Some(&i) = self.instructions.get(self.pointer) {
            match i {
                0 => self.adv(),
                1 => self.bxl(),
                2 => self.bst(),
                3 => self.jnz(),
                4 => self.bxc(),
                5 =>
                    if !self.out() && !finish {
                        return false;
                    },
                6 => self.bdv(),
                7 => self.cdv(),
                _ => unreachable!(),
            }
        }
        self.instructions == self.result
    }

    fn get_str(&self) -> String {
        let strings: Vec<_> = self.result.iter().map(u8::to_string).collect();
        strings.join(",")
    }

    fn parse(mut lines: impl Iterator<Item = String>) -> Self {
        let [reg_a, reg_b, reg_c] = lines
            .by_ref()
            .take(3)
            .map(|s| s.split_once(':').unwrap().1.trim().parse().unwrap())
            .array_chunks()
            .next()
            .unwrap();
        let instructions = lines
            .nth(1)
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        Self { reg_a, reg_b, reg_c, instructions, pointer: 0, result: Vec::new() }
    }
}

pub fn day17_a() {
    let mut computer = Computer::parse(read_lines(17));
    computer.run(true);
    let result = computer.get_str();
    println!("{result}");
}

const EXPECTED: [i64; 16] = [2, 4, 1, 1, 7, 5, 4, 6, 1, 4, 0, 3, 5, 5, 3, 0];

fn find() -> i64 {
    let mut potential: Vec<_> = (0..=7i64.pow(1)).filter(|&a| simulate(a).last() == Some(&0)).collect();
    for count in 2..=EXPECTED.len() {
        let mut next = Vec::new();
        potential.into_iter().for_each(|a| check(a, count, &mut next));
        potential = next;
    }
    potential.into_iter().min().unwrap()
}

fn check(previous: i64, count: usize, results: &mut Vec<i64>) {
    for end in 0..=7 {
        let a = (previous << 3) | end;
        let result = simulate(a);
        if result.len() >= count && result[result.len() - count..] == EXPECTED[16 - count..] {
            results.push(a);
        }
    }
}

fn simulate(start: i64) -> Vec<i64> {
    let mut result = Vec::new();
    let mut a = start;
    while a != 0 {
        let mut b = a & 0b111;
        b ^= 0b1;
        b ^= (a >> b) & 0b111;
        b ^= 0b100;
        a >>= 3;
        result.push(b & 0b111);
    }
    result
}

fn test() {
    for test in 0o10..0o100 {
        let base = simulate(test);
        let moved = test << 3;
        print!("{test:o}: {base:?} | ");
        for x in 0..=7 {
            let new = moved |x;
            let result = simulate(new);
            print!("{new:o}: {result:?} | ");
            if base != result[result.len() - base.len()..] {
                println!("{test:o}, {x}");
            }
        }
        println!();
    }
}

pub fn day17_b() {
    // test();
    let result = find();
    println!("{result}");
}
