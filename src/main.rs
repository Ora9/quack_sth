use quack::{Graph, Node, node::NumericValue, };

fn main() {
    let mut graph = Graph::new();

    let numvalue = graph.insert(Box::new(NumericValue::new()));

    dbg!(&graph);
    dbg!(numvalue);

    // graph.evaluate();
}
