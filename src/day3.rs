use super::read_string;

struct ConditionalCounter {
    counter: MulCounter,
    enabled: bool,
    toggle_counter: usize,
}

impl ConditionalCounter {
    const fn new() -> Self { Self { counter: MulCounter::new(), enabled: true, toggle_counter: 0 } }

    fn next(&mut self, c: char) {
        if self.enabled {
            self.counter.next(c);
        }
        self.check_toggle(c);
    }

    fn check_toggle(&mut self, c: char) {
        match (self.toggle_counter, self.enabled, c) {
            (0, _, 'd') | (1, _, 'o') | (2, true, 'n') | (2, false, '(') | (3, true, '\'') | (4, true, 't') | (5, true, '(') =>
                self.toggle_counter += 1,
            (3, false, ')') | (6, true, ')') => {
                self.toggle_counter = 0;
                self.enabled = !self.enabled;
            },
            _ => self.toggle_counter = 0,
        }
    }
}

struct MulCounter {
    sum: usize,
    state: MulState,
}

impl MulCounter {
    const fn new() -> Self { Self { sum: 0, state: MulState::None } }

    fn next(&mut self, c: char) { self.state.next(c, &mut self.sum); }
}

enum MulState {
    None,
    Name(usize),
    FirstDigit(usize),
    SecondDigit(usize, usize),
}

impl MulState {
    fn next(&mut self, c: char, sum: &mut usize) {
        match self {
            Self::None =>
                if c == 'm' {
                    *self = Self::Name(0);
                },
            Self::Name(u) => match (*u, c) {
                (0, 'u') | (1, 'l') => *u += 1,
                (2, '(') => *self = Self::FirstDigit(0),
                _ => *self = Self::None,
            },
            Self::FirstDigit(u) => match (*u, c) {
                (..100, '0'..='9') => {
                    *u *= 10;
                    *u += c as usize - '0' as usize;
                },
                (1.., ',') => *self = Self::SecondDigit(*u, 0),
                _ => *self = Self::None,
            },
            Self::SecondDigit(p, u) => match (*u, c) {
                (..100, '0'..='9') => {
                    *u *= 10;
                    *u += c as usize - '0' as usize;
                },
                (1.., ')') => {
                    *sum += *p * *u;
                    *self = Self::None;
                },
                _ => *self = Self::None,
            },
        }
    }
}

pub fn day3_b() {
    let content = read_string(3);
    let mut result = ConditionalCounter::new();
    content.chars().for_each(|c| result.next(c));
    let result = result.counter.sum;
    println!("{result}");
}

pub fn day3_a() {
    let content = read_string(3);
    let mut result = MulCounter::new();
    content.chars().for_each(|c| result.next(c));
    let result = result.sum;
    println!("{result}");
}

