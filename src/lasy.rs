use std::sync::{Arc, Mutex};

use anyhow::Context;

use crate::{Graph, Meta};

/// `LasyExecutor` is the executor of Quack, constructed with a reference to the graph, it is responsible for
/// evaluating all needed nodes, handling their ins and outs
///
/// The inner working of `LasyExecutor` is subject to change, to allow for more performance
///
/// Currently, `LasyExecutor` :
/// - Is cheaply cloned
/// - Recursively traverse the graph
#[derive(Debug, Clone)]
pub struct LasyExecutor {
    graph: Arc<Mutex<Graph>>,
}

impl LasyExecutor {
    pub fn new(graph: Arc<Mutex<Graph>>) -> Self {
        Self { graph }
    }

    pub(crate) fn evaluate_for(&self, out_name: &str, meta: Meta) -> Result<(), anyhow::Error> {
        dbg!(out_name, meta);

        let graph = self.graph.lock().expect("the graph is inaccessible");
        let out_id = graph
            .graph_out_id_for(out_name)
            .context("out name not found for this graph")?
            .inout_id();

        graph.evaluate(out_id, self.clone(), meta);

        Ok(())
    }

    fn get() {}
}
