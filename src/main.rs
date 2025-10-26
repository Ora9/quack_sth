#![allow(dead_code)]

use crate::{graph::Graph, node::{Osc, Print}};

mod graph {
    use std::{any::Any, collections::{HashMap, HashSet}, sync::{Arc, Mutex}};

    use crate::{meta::Meta, node::{EdgepointId, Node, NodeId}};

    /// A graph contains nodes,
    pub struct Graph {
        nodes: HashMap<NodeId, Box<dyn Node>>,
        edges: HashSet<(EdgepointId, EdgepointId)>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: HashMap::new(),
                edges: HashSet::new(),
            }
        }

        pub fn insert(&mut self, node: Box<dyn Node>) -> NodeId {
            let id = NodeId::new();
            self.nodes.insert(id, node);
            id
        }

        pub fn patch(&mut self, output_edgepoint: EdgepointId, input_edgepoint: EdgepointId) {

            // self.edges.insert()

        }

        pub fn evaluate(&self) {
            for (id, node) in &self.nodes {
                if node.should_run_if_leaf() {
                    node.evaluate(None, Box::new("oui!".to_string()), Meta {
                        quality: crate::meta::Quality::Balanced,
                        tick: 5
                    });
                }
            }
        }
    }

    struct LasyInputs {
        node_id: NodeId,
        graph: Arc<Mutex<Graph>>,
        inputs: HashMap<EdgepointId, Box<dyn Any>>,
    }

    impl LasyInputs {
        fn get() {

        }
    }
}

mod node {
    use std::{any::Any, hash::{DefaultHasher, Hash, Hasher}};

    use crate::meta::Meta;

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct NodeId(u64);

    impl NodeId {
        pub fn new() -> Self {
            let mut hasher = DefaultHasher::new();
            std::time::Instant::now().hash(&mut hasher);
            Self (hasher.finish())
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct EdgepointId(NodeId, u64);

    impl EdgepointId {
        pub fn new(node_id: NodeId, source: impl Hash) -> Self {
            let mut hasher = DefaultHasher::new();
            source.hash(&mut hasher);
            Self (node_id, hasher.finish())
        }

        pub fn node_id(&self) -> NodeId {
            self.0
        }
    }

    pub trait Node {
        fn title(&self) -> &str;
        fn evaluate(&self, output_id: Option<EdgepointId>, input: Box<dyn Any>, meta: Meta);

        fn should_run_if_leaf(&self) -> bool;
    }

    pub enum PrintEdgepoint {
        Input,
    }

    pub struct Print();
    impl Node for Print {
        fn title(&self) -> &str {
            "Print"
        }

        fn should_run_if_leaf(&self) -> bool {
            true
        }

        fn evaluate(&self, output_id: Option<EdgepointId>, input: Box<dyn Any>, meta: Meta) {
            dbg!(self.title());

            dbg!(output_id);
            dbg!(meta);

            if let Some(str) = input.downcast_ref::<String>() {
                println!("a string! : {str}");
            } else {
                println!("not a string!");
            }
        }
    }

    #[derive(Debug, Hash)]
    pub enum OscEdgepoints {
        Frequency,
        Amplitude,
        Out,
    }

    pub struct Osc();

    impl Node for Osc {
        fn title(&self) -> &str {
            "Osc"
        }

        fn should_run_if_leaf(&self) -> bool {
            false
        }

        fn evaluate(&self, output_id: Option<EdgepointId>, input: Box<dyn Any>, meta: Meta) {
            dbg!(self.title());

            dbg!(output_id);
            dbg!(meta);

        }
    }
}

mod meta {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Quality {
        Highest,
        Balanced,
        Performance,
        Lowest,
    }

    #[derive(Debug)]
    pub struct Meta {
        pub tick: u64,
        pub quality: Quality,
    }

    impl Meta {
        fn tick(&self) -> u64 {
            self.tick
        }
    }
}

fn main() {
    let mut graph = Graph::new();

    graph.insert(Box::new(Print()));
    graph.evaluate();
}
