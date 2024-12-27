use std::collections::HashMap;

use super::{IterPairs as _, SplitOnceArr as _, read_lines};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
struct Node([u8; 2]);

#[expect(clippy::fallible_impl_from)]
impl From<&str> for Node {
    fn from(value: &str) -> Self { Self(value.bytes().array_chunks().next().unwrap()) }
}

impl From<Node> for String {
    fn from(value: Node) -> Self {
        Self::from_utf8(value.0.to_vec()).unwrap()
    }
}

impl Node {
    const fn starts_with(self, b: u8) -> bool { self.0[0] == b }

    const fn second(self) -> u8 { self.0[1] }
}

struct Graph {
    connections: HashMap<Node, Vec<Node>>,
}

impl Graph {
    fn new() -> Self { Self { connections: HashMap::new() } }

    fn connected(&self, left: Node, right: Node) -> bool {
        self.connections.get(&left).is_some_and(|v| v.contains(&right))
    }

    fn add(&mut self, left: Node, right: Node) {
        self.connections.entry(left).or_default().push(right);
        self.connections.entry(right).or_default().push(left);
    }

    fn find_pairs(&self) -> impl Iterator<Item=[Node; 3]> {
        self.connections.iter().filter(|(k, _)| k.starts_with(b't')).flat_map(move |(&k, v)| {
            v.iter_pairs()
                .filter(move |&(&l, &r)| {
                    (!l.starts_with(b't') || l.second() < k.second())
                        && (!r.starts_with(b't') || r.second() < k.second())
                        && self.connected(l, r)
                })
                .map(move |(&l, &r)| [k, l, r])
        })
    }

    fn sort(&mut self) -> Vec<Node> {
        let mut nodes: Vec<_> = self
            .connections
            .iter_mut()
            .map(|(k, v)| {
                v.sort();
                *k
            })
            .collect();
        nodes.sort();
        nodes
    }
}

fn find_recursive(current: Vec<Node>, potential: Vec<Node>, g: &Graph) -> Vec<Node> {
    let mut largest = current.clone();
    for p in potential.clone() {
        let mut new_current = current.clone();
        new_current.push(p);
        let new_potential = potential
            .iter()
            .copied()
            .skip_while(|&p2| p2 <= p)
            .filter(|&p2| new_current.iter().all(|&c| g.connected(c, p2)))
            .collect();
        let result = find_recursive(new_current, new_potential, g);
        if result.len() > largest.len() {
            largest = result;
        }
    }
    largest
}

fn find_largest_clique(mut g: Graph) -> Vec<Node> {
    let nodes = g.sort();
    let mut largest = Vec::new();
    for n in nodes {
        let current = vec![n];
        let potential: Vec<_> =
            g.connections.get(&n).unwrap().iter().copied().skip_while(|&m| m < n).collect();
        let result = find_recursive(current, potential, &g);
        if result.len() > largest.len() {
            largest = result;
        }
    }
    largest
}

fn get_graph() -> Graph {
    let mut graph = Graph::new();
    read_lines(23)
        .map(|s| s.split_once_arr('-').unwrap().map(Node::from))
        .for_each(|[l, r]| graph.add(l, r));
    graph
}

pub fn day23_a() {
    let graph = get_graph();
    let result = graph.find_pairs().count();
    println!("{result}");
}

pub fn day23_b() {
    let graph = get_graph();
    let clique = find_largest_clique(graph);
    let mut strings: Vec<_> = clique.into_iter().map(String::from).collect();
    strings.sort();
    let result = strings.join(",");
    println!("{result}");
}
