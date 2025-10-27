use std::{any::Any, hash::{DefaultHasher, Hash, Hasher}};

use crate::Meta;

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
