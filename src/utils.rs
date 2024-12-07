use std::fs;
use std::fs::File;
use std::io::{BufRead as _, BufReader};

fn get_path(day: usize) -> String {
    format!("/home/tedem/RustroverProjects/advent_of_code/data/{day}.txt")
}

pub fn read_lines(day: usize) -> impl Iterator<Item = String> {
    BufReader::new(File::open(get_path(day)).unwrap()).lines().map_while(Result::ok)
}

pub fn read_string(day: usize) -> String { fs::read_to_string(get_path(day)).unwrap() }

pub fn read_grid_bytes(day: usize) -> Box<[Box<[u8]>]> {
    read_lines(day).map(|s| s.into_boxed_str().into_boxed_bytes()).collect()
}