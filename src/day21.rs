use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter, Result as FmtResult, Write};
use std::iter;

use super::{Vec2, read_lines};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Move {
    Up,
    Right,
    Down,
    Left,
    Activate,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct AToA(Box<[Move]>);

impl AToA {
    fn iter(&self) -> impl Iterator<Item = Move> { self.0.iter().copied() }

    const fn len(&self) -> usize { self.0.len() }
}

#[derive(Debug)]
struct AToAInfo {
    count: usize,
    cached_result: Option<Box<[AToA]>>,
}

impl AToAInfo {
    const fn new() -> Self { Self { count: 0, cached_result: None } }
}

#[derive(Debug)]
struct Finder {
    current: HashMap<AToA, AToAInfo>,
}

impl Finder {
    fn new() -> Self { Self { current: HashMap::new() } }

    fn find(&mut self, sequence: &str, robot_layers: usize) -> usize {
        self.clear_counts();
        Self::shortest_sequence(sequence).into_iter().for_each(|a| self.add(a, 1));
        for _ in 0..robot_layers {
            self.step();
        }
        self.current.iter().map(|(a, i)| a.len() * i.count).sum()
    }

    fn step(&mut self) {
        let mut counts_to_change: HashMap<AToA, isize> = HashMap::new();
        for (a, i) in &mut self.current {
            if i.count == 0 {
                continue;
            }
            i.cached_result.get_or_insert_with(|| Self::shortest_atoa(a)).into_iter().for_each(
                |a| {
                    *counts_to_change.entry(a.clone()).or_default() +=
                        isize::try_from(i.count).unwrap();
                },
            );
            *counts_to_change.entry(a.clone()).or_default() -= isize::try_from(i.count).unwrap();
        }
        counts_to_change.into_iter().for_each(|(a, c)| self.add(a, c));
    }

    fn add(&mut self, result: AToA, count: isize) {
        let current = &mut self.current.entry(result).or_insert_with(AToAInfo::new).count;
        if count >= 0 {
            *current += usize::try_from(count).unwrap();
        } else {
            *current -= usize::try_from(-count).unwrap();
        }
    }

    fn clear_counts(&mut self) { self.current.values_mut().for_each(|i| i.count = 0); }

    const fn numpad_position(c: char) -> Vec2<usize> {
        match c {
            '7' => Vec2::new(0, 0),
            '8' => Vec2::new(1, 0),
            '9' => Vec2::new(2, 0),
            '4' => Vec2::new(0, 1),
            '5' => Vec2::new(1, 1),
            '6' => Vec2::new(2, 1),
            '1' => Vec2::new(0, 2),
            '2' => Vec2::new(1, 2),
            '3' => Vec2::new(2, 2),
            '0' => Vec2::new(1, 3),
            'A' => Vec2::new(2, 3),
            _ => panic!(),
        }
    }

    const fn arrpad_position(m: Move) -> Vec2<usize> {
        match m {
            Move::Up => Vec2::new(1, 0),
            Move::Activate => Vec2::new(2, 0),
            Move::Left => Vec2::new(0, 1),
            Move::Down => Vec2::new(1, 1),
            Move::Right => Vec2::new(2, 1),
        }
    }

    fn shortest_atoa(atoa: &AToA) -> Box<[AToA]> {
        iter::once(Move::Activate)
            .chain(atoa.iter())
            .map_windows(|&[l, r]| {
                Self::shortest_between(
                    Self::arrpad_position(l),
                    Self::arrpad_position(r),
                    Vec2::new(0, 0),
                )
            })
            .collect()
    }

    fn shortest_sequence(sequence: &str) -> Box<[AToA]> {
        iter::once('A')
            .chain(sequence.chars())
            .map_windows(|&[l, r]| {
                Self::shortest_between(
                    Self::numpad_position(l),
                    Self::numpad_position(r),
                    Vec2::new(0, 3),
                )
            })
            .collect()
    }

    fn shortest_between(from: Vec2<usize>, to: Vec2<usize>, gap: Vec2<usize>) -> AToA {
        let cmp = to.cmp_each(&from);
        let mut result = Vec::new();
        let x_m = |o| if o == Ordering::Greater { Move::Right } else { Move::Left };
        let y_m = |o| if o == Ordering::Greater { Move::Down } else { Move::Up };
        match (cmp.x, cmp.y) {
            (Ordering::Equal, Ordering::Equal) => {},
            (Ordering::Equal, c) => result.extend(iter::repeat_n(y_m(c), from.y.abs_diff(to.y))),
            (c, Ordering::Equal) => result.extend(iter::repeat_n(x_m(c), from.x.abs_diff(to.x))),
            (x_c, y_c) => {
                let can_start_h = !(from.y == gap.y && to.x == gap.x);
                let can_start_v = !(from.x == gap.x && to.y == gap.y);
                let horizontal: bool;
                if can_start_h && !can_start_v {
                    horizontal = true;
                } else if can_start_v && !can_start_h {
                    horizontal = false;
                } else {
                    let x = x_m(x_c);
                    let y = y_m(x_c);
                    // Furthest move has to first
                    match (x, y) {
                        (Move::Left, _) => horizontal = true,
                        (_, Move::Down) => horizontal = false,
                        _ => horizontal = true,
                    }
                }
                if horizontal {
                    result.extend(iter::repeat_n(x_m(x_c), from.x.abs_diff(to.x)));
                    result.extend(iter::repeat_n(y_m(y_c), from.y.abs_diff(to.y)));
                } else {
                    result.extend(iter::repeat_n(y_m(y_c), from.y.abs_diff(to.y)));
                    result.extend(iter::repeat_n(x_m(x_c), from.x.abs_diff(to.x)));
                }
            }
        }
        result.push(Move::Activate);
        AToA(result.into_boxed_slice())
    }
}

fn count(n: usize) -> usize {
    let mut finder = Finder::new();
    read_lines(21).map(|s| finder.find(&s, n) * s[..s.len() - 1].parse::<usize>().unwrap()).sum()
}

pub fn day21_a() {
    let result = count(2);
    println!("{result}");
}

pub fn day21_b() {
    let result = count(25);
    println!("{result}");
}

// Furthest first (idk just works)
//v<A
// 	v<A<A>>^A | V<A<A>>^A V<<A>>^A VAA<^A<A
// 	<vA<A>>^A | V<<A>A>^A V<<A>>^A VAA<^A<A 
// 	v<A<A^>>A | V<A<A>>^A V<<A>>^A <A>VAA^A
// 	<vA<A^>>A | V<<A<A<^A V<<A>>^A <A<VAA^A
//<vA
// 	V<<A>A>^A | <VA<AA>>^A VA^A VA<^A>A
// 	<<VA>A>^A | V<<AA>A>^A VA^A V>A^A>A
// 	V<<A>A^>A | 
// 	>>VA>A^>A |
// 	
// <<vA | v<<AA>A^>A | <vA<AA>>^AAvA^A<A>VA^A
// v<<A | v<A<AA>>^A | <vA
