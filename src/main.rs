#![feature(iter_map_windows, pattern)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod utils;

use utils::{read_lines, read_string, read_grid_bytes, SplitOnceArr};

fn main() { day6::day6_b() }
