use nalgebra::Complex;
use petgraph::graph::NodeIndex;

use super::{Element, Terminal, MIN_RESISTANCE};

#[derive(Default, Clone)]
pub struct Inductor {
    inductance: f32,
    terminals: [Terminal; 2],
}

impl Inductor {
    pub fn new(inductance: f32, positive_node: NodeIndex, negative_node: NodeIndex) -> Self {
        Self {
            inductance,
            terminals: [
                Terminal::new(positive_node, super::Polarity::Positive),
                Terminal::new(negative_node, super::Polarity::Negative),
            ],
        }
    }
}

impl Element for Inductor {
    fn terminals(&self) -> &[Terminal] {
        &self.terminals
    }

    fn dc_voltage(&self) -> f32 {
        0.0
    }

    fn ac_voltage(&self) -> Complex<f32> {
        Complex::ZERO
    }

    fn dc_current(&self) -> f32 {
        0.0
    }

    fn ac_current(&self) -> Complex<f32> {
        Complex::ZERO
    }

    fn resistance(&self) -> f32 {
        MIN_RESISTANCE
    }

    /// The rectangular impedence of an inductor is equal to `0 + jωL`,
    /// where `L` is the inductance in Henries and `ω` is the frequency in Hertz.
    fn impedance(&self, frequency: f32) -> Complex<f32> {
        Complex::new(0.0, self.inductance * frequency)
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Complex;
    use petgraph::graph::NodeIndex;

    use crate::elements::Element;

    use super::Inductor;

    /// Test if the impedance for the inductor is correctly calculated.
    #[test]
    fn impedance() {
        let inductor = Inductor::new(10.0, NodeIndex::new(0), NodeIndex::new(1));
        assert_eq!(
            inductor.impedance(1000.0),
            Complex::<f32>::new(0.0, 10000.0)
        )
    }
}
