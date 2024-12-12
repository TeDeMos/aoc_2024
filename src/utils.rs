use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::str::FromStr;
use std::str::pattern::Pattern;

fn get_path(day: usize) -> String {
    format!("/home/tedem/RustroverProjects/advent_of_code/data/{day}.txt")
}

pub fn read_lines(day: usize) -> impl Iterator<Item = String> {
    BufReader::new(File::open(get_path(day)).unwrap()).lines().map_while(Result::ok)
}

pub fn read_string(day: usize) -> String { fs::read_to_string(get_path(day)).unwrap() }

pub fn read_numbers<T: FromStr<Err: Debug>>(day: usize) -> Vec<T> {
    read_string(day).split_whitespace().map(|s| s.parse::<T>().unwrap()).collect()
}

pub fn read_digits(day: usize) -> Box<[u8]> { 
    let mut bytes = read_string(day).into_boxed_str().into_boxed_bytes();
    bytes.iter_mut().for_each(|b| *b -= b'0');
    bytes
}

pub fn read_grid_digits(day: usize) -> Box<[Box<[u8]>]> {
    read_lines(day).map(|s| {
        let mut bytes = s.into_boxed_str().into_boxed_bytes();
        bytes.iter_mut().for_each(|b| *b -= b'0');
        bytes
    }).collect()
}

pub fn read_grid_bytes(day: usize) -> Box<[Box<[u8]>]> {
    read_lines(day).map(|s| s.into_boxed_str().into_boxed_bytes()).collect()
}

pub trait SplitOnceArr {
    fn split_once_arr<P: Pattern>(&self, pattern: P) -> Option<[&str; 2]>;
}

impl SplitOnceArr for &str {
    fn split_once_arr<P: Pattern>(&self, pattern: P) -> Option<[&str; 2]> {
        self.split_once(pattern).map(<[&str; 2]>::from)
    }
}

impl SplitOnceArr for String {
    fn split_once_arr<P: Pattern>(&self, pattern: P) -> Option<[&str; 2]> {
        self.split_once(pattern).map(<[&str; 2]>::from)
    }
}

pub trait IterPairs {
    type Item;

    fn iter_pairs(&self) -> impl Iterator<Item = (&Self::Item, &Self::Item)>;
}

impl<T> IterPairs for Vec<T> {
    type Item = T;

    fn iter_pairs(&self) -> impl Iterator<Item = (&T, &T)> {
        self.iter().enumerate().flat_map(|(i, x)| self[i + 1..].iter().map(move |y| (x, y)))
    }
}
