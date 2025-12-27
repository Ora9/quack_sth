use std::{any::Any, fmt::Debug};

use crate::{HashId, InoutId, LasyExecutor, Meta, NodeId, NodeInoutId};

pub mod numeric;
pub use numeric::*;

pub trait Node: Debug {
    fn new() -> Self
    where
        Self: Sized;

    /// The node "title" when displayed
    fn title(&self) -> &str;

    fn evaluate(&self, out_id: InoutId, lasy_executor: LasyExecutor, meta: Meta) -> f32;

    fn node_inout_id_for(&self, inout_name: &str, node_id: NodeId) -> Option<NodeInoutId> {
        self.id_for(inout_name)
            .and_then(|inout_id| Some(NodeInoutId::new(node_id, inout_id)))
    }

    fn id_for(&self, inout_name: &str) -> Option<InoutId>;
}
