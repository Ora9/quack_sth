use std::{any::Any, fmt::Debug};

use crate::{InoutId, Meta, NodeInoutId};

// pub mod audio;
pub mod numeric;
pub use numeric::*;
// pub mod textual;

// pub use textual::*;

pub trait Node: Debug {
    fn title(&self) -> &str;
    fn evaluate(&self, output_id: Option<InoutId>, input: Box<dyn Any>, meta: Meta);

    fn id_for(&self, inout_name: &str) -> Option<NodeInoutId>;

    fn new() -> Self where Self: Sized;
}
