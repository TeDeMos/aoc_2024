use std::cmp::Ordering;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::ops::{Add, AddAssign, Div, Mul, Sub};
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
    read_lines(day)
        .map(|s| {
            let mut bytes = s.into_boxed_str().into_boxed_bytes();
            bytes.iter_mut().for_each(|b| *b -= b'0');
            bytes
        })
        .collect()
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

pub trait IntDivide<T = Self> {
    type Output;

    fn int_divide(self, rhs: T) -> Option<Self::Output>;
}

impl IntDivide for i64 {
    type Output = Self;

    fn int_divide(self, rhs: Self) -> Option<Self::Output> { (self % rhs == 0).then(|| self / rhs) }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self { Self { x, y } }
}

impl<T> From<[T; 2]> for Vec2<T> {
    fn from([x, y]: [T; 2]) -> Self { Self { x, y } }
}

impl<T> From<Vec2<T>> for [T; 2] {
    fn from(value: Vec2<T>) -> Self { [value.x, value.y] }
}

impl Vec2<i64> {
    pub const fn rem_euclid(self, other: Self) -> Self {
        Self { x: self.x.rem_euclid(other.x), y: self.y.rem_euclid(other.y) }
    }
}

impl<T> Vec2<T>
where T: Mul<Output = T> + Sub<Output = T> + Copy
{
    pub fn cross_product(self, rhs: Self) -> T { self.x * rhs.y - self.y * rhs.x }
}

impl<T> AddAssign for Vec2<T>
where T: AddAssign
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> AddAssign<T> for Vec2<T>
where T: AddAssign + Copy
{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl<T> Mul<T> for Vec2<T>
where T: Mul + Copy
{
    type Output = Vec2<T::Output>;

    fn mul(self, rhs: T) -> Self::Output { Vec2 { x: self.x * rhs, y: self.y * rhs } }
}

impl<T> Div<T> for Vec2<T>
where T: Div + Copy
{
    type Output = Vec2<T::Output>;

    fn div(self, rhs: T) -> Self::Output { Vec2 { x: self.x / rhs, y: self.y / rhs } }
}

impl<T> Vec2<T>
where T: Ord
{
    pub fn cmp_each(&self, other: &Self) -> Vec2<Ordering> {
        Vec2 { x: self.x.cmp(&other.x), y: self.y.cmp(&other.y) }
    }
}

impl Vec2<usize> {
    pub const fn up(self) -> Self { Self { x: self.x, y: self.y - 1 } }

    pub const fn right(self) -> Self { Self { x: self.x + 1, y: self.y } }

    pub const fn down(self) -> Self { Self { x: self.x, y: self.y + 1 } }

    pub const fn left(self) -> Self { Self { x: self.x - 1, y: self.y } }
}
