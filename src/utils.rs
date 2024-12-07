use std::fs::File;
use std::io::{BufRead as _, BufReader};

pub fn read_lines(day: usize) -> impl Iterator<Item=String> {
    let file = File::open(format!("/home/tedem/RustroverProjects/advent_of_code/data/{day}.txt")).unwrap();
    BufReader::new(file)
        .lines()
        .map_while(Result::ok)
}
