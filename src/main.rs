#![allow(dead_code)]

use quack::{Graph, Print};

fn main() {
    let mut graph = Graph::new();

    graph.insert(Box::new(Print()));
    graph.evaluate();
}
