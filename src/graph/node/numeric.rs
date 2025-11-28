use std::{any::Any, collections::HashMap, str::FromStr};
use strum::IntoEnumIterator;

use crate::{InoutId, Meta, Node, NodeInoutId};

#[derive(Debug, PartialEq, Eq, Hash, strum::EnumIter, strum::EnumString)]
pub enum NumericValueInout {
    Output
}

#[derive(Debug)]
pub struct NumericValue {
    value: f32,

    inout_ids: HashMap<NumericValueInout, NodeInoutId>,
}

impl Node for NumericValue {
    fn new() -> Self {
        let mut inout_ids = HashMap::new();

        for inout in NumericValueInout::iter() {
            dbg!(&inout);
            inout_ids.insert(inout, NodeInoutId::new());
        };

        Self {
            value: Default::default(),
            inout_ids: inout_ids,
        }
    }

    fn id_for(&self, inout_name: &str) -> Option<NodeInoutId> {
        if let Ok(inout_enum) = NumericValueInout::from_str(inout_name) {
            self.inout_ids.get(&inout_enum).cloned()
        } else {
            None
        }
    }

    fn title(&self) -> &str {
        "String Value"
    }

    fn evaluate(&self, output_id: Option<InoutId>, input: Box<dyn Any>, meta: Meta) {
        dbg!(self.title());

        dbg!(output_id);
        dbg!(meta);

    }
}
