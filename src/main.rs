#![feature(iter_map_windows, pattern)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

use utils::{read_lines, read_string, read_digits, read_grid_bytes, SplitOnceArr, IterPairs};

fn main() { day9::day9_b() }
