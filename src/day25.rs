use super::read_lines;

type Pins = [usize; 5];
type Keys = Vec<Pins>;
type Locks = Vec<Pins>;

fn get_data() -> (Keys, Locks) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    read_lines(25).array_chunks::<8>().for_each(|a| {
        let mut result = [0; 5];
        let check_row = |(s, i): (&String, _)| {
            s.chars().enumerate().for_each(|(n, c)| {
                if c == '#' && result[n] < i {
                    result[n] = i;
                }
            });
        };
        match a[0].as_str() {
            "#####" => {
                a[1..=5].iter().rev().zip((1..=5).rev()).for_each(check_row);
                locks.push(result);
            },
            "....." => {
                a[1..=5].iter().zip((1..=5).rev()).for_each(check_row);
                keys.push(result);
            },
            _ => unreachable!(),
        }
    });
    (keys, locks)
}

fn get_potential(keys: &Keys, locks: &Locks) -> impl Iterator<Item = (Pins, Pins)> {
    keys.iter().copied().flat_map(|k| {
        locks
            .iter()
            .copied()
            .filter(move |&l| k.into_iter().zip(l).all(|(k_p, l_p)| k_p + l_p <= 5))
            .map(move |l| (k, l))
    })
}

pub fn day25_a() {
    let (keys, locks) = get_data();
    let result = get_potential(&keys, &locks).count();
    println!("{result}");
}
