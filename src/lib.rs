use elements::Element;

pub mod elements;
pub mod runners;

#[derive(Default)]
pub struct Circuit {
    nodes: Vec<NodeId>,
    elements: Vec<Box<dyn Element>>,
}

impl Circuit {
    /// Adds a new [`NodeId`] to the node list.
    ///
    /// Starts at 0, which is ground, with every subsequent node having the id of `n + 1`.
    ///
    /// ```
    /// use spice_rs::Circuit;
    ///
    /// let mut circuit = Circuit::default();
    ///
    /// let ground = circuit.add_node();
    /// let v1 = circuit.add_node();
    /// let v2 = circuit.add_node();
    ///
    /// assert_eq!(ground.0, 0);
    /// assert_eq!(v1.0, 1);
    /// assert_eq!(v2.0, 2);
    /// ```
    pub fn add_node(&mut self) -> NodeId {
        let next_node = NodeId(self.nodes.len());
        self.nodes.push(next_node);

        next_node
    }

    /// Adds a new element to the circuit.
    pub fn add_element(&mut self, element: Box<dyn Element>) {
        self.elements.push(element);
    }

    /// The number of nodes in the circuit.
    pub fn node_count(&self) -> usize {
        self.nodes.iter().len()
    }

    /// All of the elements in the circuit.
    pub fn elements(&self) -> &[Box<dyn Element>] {
        &self.elements
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NodeId(pub usize);
