#![feature(iter_array_chunks, iter_map_windows, let_chains, pattern)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

use utils::{
    IntDivide, IterPairs, SplitOnceArr, Vec2, read_digits, read_grid_bytes, read_grid_digits,
    read_lines, read_numbers, read_string,
};

fn main() { day13::day13_b() }
