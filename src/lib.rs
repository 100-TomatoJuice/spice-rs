use std::ops::{Deref, DerefMut};

use elements::Element;
use petgraph::graph::UnGraph;

pub mod elements;
pub mod runners;

pub struct Circuit(pub UnGraph<bool, Box<dyn Element>>);

impl Deref for Circuit {
    type Target = UnGraph<bool, Box<dyn Element>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Circuit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
