use std::{any::Any, collections::{HashMap, HashSet}, sync::{Arc, Mutex}};

use crate::{node::{EdgepointId, Node, NodeId}};

mod meta;
pub use meta::{Meta, Quality};

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
                    quality: Quality::Balanced,
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
