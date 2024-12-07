#![feature(iter_map_windows)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod utils;

use utils::{read_lines, read_string, read_grid_bytes};

fn main() { day4::day4_b(); }



