use super::{IntDivide as _, SplitOnceArr as _, Vec2, read_lines};

struct Machine {
    a: Vec2<i64>,
    b: Vec2<i64>,
    prize: Vec2<i64>,
}

impl From<[Vec2<i64>; 3]> for Machine {
    fn from([a, b, prize]: [Vec2<i64>; 3]) -> Self { Self { a, b, prize } }
}

impl Machine {
    fn parse(lines: &[String; 3]) -> Self {
        Self::from(lines.each_ref().map(|l| {
            Vec2::<i64>::from(
                l.split_once(':')
                    .unwrap()
                    .1
                    .split_once_arr(',')
                    .unwrap()
                    .map(|v| v.split_once(['+', '=']).unwrap().1.parse().unwrap()),
            )
        }))
    }

    fn count(self) -> Option<i64> {
        // p_x - n * a_x = m * b_x => m = (p_x - n * a_x) / b_x
        // p_y - n * a_y = m * b_y => m = (p_y - n * a_y) / b_y
        // (p_x - n * a_x) / b_x = (p_y - n * a_y) / b_y
        // b_y * p_x - n * a_x * b_y = b_x * p_y - n * a_y * b_x
        // n * a_y * b_x - n * a_x * b_y = b_x * p_y - b_y * p_x
        // n * (a_y * b_x - a_x * b_y) = b_x * p_y - b_y * p_x
        // n = (b_x * p_y - b_y * p_x) / (a_y * b_x - a_x * b_y)
        let n = self.b.cross_product(self.prize).int_divide(self.b.cross_product(self.a))?;
        let m = (self.prize.x - n * self.a.x).int_divide(self.b.x)?;
        Some(3 * n + m)
    }
}

fn read_machines() -> impl Iterator<Item = Machine> {
    read_lines(13).filter(|s| !s.is_empty()).array_chunks().map(|m| Machine::parse(&m))
}

pub fn day13_a() {
    let result: i64 = read_machines().filter_map(Machine::count).sum();
    println!("{result}");
}

pub fn day13_b() {
    let result: i64 = read_machines()
        .filter_map(|mut m| {
            m.prize += 10_000_000_000_000;
            m.count()
        })
        .sum();
    println!("{result}");
}
