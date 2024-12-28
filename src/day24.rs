use std::collections::HashMap;

use super::read_lines;

#[derive(Debug, Copy, Clone)]
enum Value {
    Known(bool),
    Unknown(Gate),
}

type Name = [u8; 3];

fn get_name(s: &str) -> Name { s.bytes().array_chunks().next().unwrap() }

fn display_name(n: Name) -> String { String::from_utf8(n.to_vec()).unwrap() }

#[derive(Copy, Clone, Debug)]
struct Gate {
    left: Name,
    right: Name,
    operation: Operation,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Operation {
    Xor,
    Or,
    And,
}

impl Operation {
    const fn calculate(self, left: bool, right: bool) -> bool {
        match self {
            Self::Xor => left ^ right,
            Self::Or => left | right,
            Self::And => left & right,
        }
    }
}

fn get_data() -> HashMap<Name, Value> {
    let mut lines = read_lines(24);
    let mut result: HashMap<_, _> = lines
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(|s| {
            let (name, value) = s.split_once(": ").unwrap();
            let value = match value {
                "1" => true,
                "0" => false,
                _ => unreachable!(),
            };
            (get_name(name), Value::Known(value))
        })
        .collect();
    result.extend(lines.map(|s| {
        let (gate, name) = s.split_once(" -> ").unwrap();
        let [left, op, right] = gate.split(' ').next_chunk().unwrap();
        let operation = match op {
            "XOR" => Operation::Xor,
            "AND" => Operation::And,
            "OR" => Operation::Or,
            _ => unreachable!(),
        };
        (
            get_name(name),
            Value::Unknown(Gate { left: get_name(left), operation, right: get_name(right) }),
        )
    }));
    result
}

fn get_value(values: &mut HashMap<Name, Value>, name: Name) -> bool {
    match *values.get(&name).unwrap() {
        Value::Known(b) => b,
        Value::Unknown(g) => {
            let left = get_value(values, g.left);
            let right = get_value(values, g.right);
            let value = g.operation.calculate(left, right);
            values.insert(name, Value::Known(value));
            value
        },
    }
}

fn get_zs(values: &HashMap<Name, Value>) -> Vec<Name> {
    let mut result: Vec<_> = values.keys().copied().filter(|&n| n[0] == b'z').collect();
    result.sort_unstable_by(|a, b| b.cmp(a));
    result
}

pub fn day24_a() {
    let mut gates = get_data();
    let zs = get_zs(&gates);
    let mut result = 0i64;
    zs.into_iter()
        .map(|z| get_value(&mut gates, z))
        .for_each(|b| result = (result << 1) | i64::from(b));
    println!("{result}");
}

fn swap_gates(gates: &mut HashMap<Name, Value>, a: Name, b: Name) {
    println!("Before: {:?}, {:?}", gates.get(&a), gates.get(&b));
    let value_a = *gates.get(&a).unwrap();
    let value_b = *gates.get_mut(&b).unwrap();
    gates.insert(a, value_b);
    gates.insert(b, value_a);
    println!("After: {:?}, {:?}", gates.get(&a), gates.get(&b));
}

fn check_if_normal_adder() {
    let mut gates = get_data();
    // Layer 12 XOR kth, OR z12
    swap_gates(&mut gates, get_name("kth"), get_name("z12"));
    // z26 = x26 and y26 and gsd because zmf = ksd or gsd
    swap_gates(&mut gates, get_name("gsd"), get_name("z26"));
    // z32 = layer 32 second AND and tbt second XOR
    swap_gates(&mut gates, get_name("tbt"), get_name("z32"));
    // z36 = qnf XOR previous_carry, wrong
    swap_gates(&mut gates, get_name("vpm"), get_name("qnf"));
    let x_y: Vec<_> = gates
        .iter()
        .filter_map(|(&n, g)| match g {
            Value::Unknown(Gate {
                left: [b'x', xn0, xn1],
                operation: o,
                right: [b'y', yn0, yn1],
            }) if xn0 == yn0 && xn1 == yn1 => Some((n, (xn0 - b'0') * 10 + (xn1 - b'0'), *o)),
            Value::Unknown(Gate {
                left: [b'y', yn0, yn1],
                operation: o,
                right: [b'x', xn0, xn1],
            }) if xn0 == yn0 && xn1 == yn1 => Some((n, (xn0 - b'0') * 10 + (xn1 - b'0'), *o)),
            _ => None,
        })
        .collect();
    let mut x_and_y: Vec<_> = x_y
        .iter()
        .filter_map(|&(name, idx, op)| (op == Operation::And).then_some((idx, name)))
        .collect();
    x_and_y.sort_unstable_by_key(|x| x.0);
    println!("{x_and_y:?}");
    let x_and_y: Vec<_> = x_and_y.into_iter().map(|x| x.1).collect();
    let mut x_xor_y: Vec<_> = x_y
        .iter()
        .filter_map(|&(name, idx, op)| (op == Operation::Xor).then_some((idx, name)))
        .collect();
    x_xor_y.sort_unstable_by_key(|x| x.0);
    println!("{x_xor_y:?}");
    let x_xor_y: Vec<_> = x_xor_y.into_iter().map(|x| x.1).collect();
    let find_names = |a: Name, b: Name, o: Operation| {
        gates.iter().find_map(|(&n, g)| match g {
            Value::Unknown(Gate { left, operation, right })
                if *operation == o && (*left == a && *right == b || *left == b && *right == a) =>
                Some(n),
            _ => None,
        })
    };
    // let find_name = |na: Name, o: Operation| -> Vec<_> {
    //     gates
    //         .iter()
    //         .filter_map(|(&n, g)| match g {
    //             Value::Unknown(Gate { left, operation, right })
    //                 if *operation == o && *left == na =>
    //                 Some((n, *right)),
    //             Value::Unknown(Gate { left, operation, right })
    //                 if *operation == o && *right == na =>
    //                 Some((n, *left)),
    //             _ => None,
    //         })
    //         .collect()
    // };
    let mut previous_carry = x_and_y[0];
    let mut layer = 1;
    loop {
        println!("Layer {layer} previous carry: {}", display_name(previous_carry));
        // if layer == 36 {
        //     println!("{}", display_name(previous_carry));
        //     println!("{}", display_name(x_xor_y[layer]));
        //     let potential = find_name(previous_carry, Operation::Xor);
        //     for p in potential {
        //         println!("Gate: {}, other input: {}", display_name(p.0), display_name(p.1));
        //     }
        // }
        let Some(second_xor) = find_names(previous_carry, x_xor_y[layer], Operation::Xor) else {
            println!("Layer {layer} second xor not found!");
            break;
        };
        let Some(second_and) = find_names(previous_carry, x_xor_y[layer], Operation::And) else {
            println!("Layer {layer} second and not found!");
            break;
        };
        // if layer == 26 {
        // if layer == 32 {
        //     println!("{}", display_name(second_and));
        //     println!("{}", display_name(second_xor));
        //     println!("{}", display_name(x_and_y[layer]));
        //     let potential = find_name(second_and, Operation::Or);
        //     let potential = find_name(x_and_y[layer], Operation::Or);
        //     for p in potential {
        //         println!("Gate: {}, other input: {}", display_name(p.0), display_name(p.1));
        //     }
        // }
        let Some(or) = find_names(x_and_y[layer], second_and, Operation::Or) else {
            println!("Layer {layer} or not found!");
            break;
        };
        println!(
            "Layer {layer} - 2nd XOR: {}, 2nd AND: {}, OR: {}",
            display_name(second_xor),
            display_name(second_and),
            display_name(or)
        );
        layer += 1;
        previous_carry = or;
        if layer > 44 {
            break;
        }
    }
    let mut swapped = ["kth", "z12", "gsd", "z26", "tbt", "z32", "vpm", "qnf"];
    swapped.sort_unstable();
    println!("{}", swapped.join(","));
}

pub fn day24_b() { check_if_normal_adder(); }
