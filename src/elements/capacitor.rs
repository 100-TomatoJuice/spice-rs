use nalgebra::Complex;

use crate::NodeId;

use super::{resistor::Resistor, Element, Terminal};

#[derive(Clone, Copy)]
pub struct Capacitor {
    capacitance: f32,
    terminals: [Terminal; 2],
}

impl Capacitor {
    pub fn new(capacitance: f32, positive_node: NodeId, negative_node: NodeId) -> Self {
        Self {
            capacitance,
            terminals: [
                Terminal::new(positive_node, super::Polarity::Positive),
                Terminal::new(negative_node, super::Polarity::Negative),
            ],
        }
    }
}

impl Element for Capacitor {
    fn terminals(&self) -> &[Terminal] {
        &self.terminals
    }

    fn stamp(&self, a_matrix: &mut Vec<f32>, z_vector: &mut Vec<f32>, n: usize, m: usize) {
        let nodes: Vec<NodeId> = self.terminals().iter().map(|x| x.node).collect();
        Resistor::new(f32::MAX, nodes[0], nodes[1]).stamp(a_matrix, z_vector, n, m);
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
        f32::MAX
    }

    /// The rectangular impedence of a capacitor is equal to `0 - j/Cω`,
    /// where `C` is the capacitance in Farads and `ω` is the frequency in Hertz.
    fn impedance(&self, frequency: f32) -> Complex<f32> {
        Complex::new(0.0, -1.0 / (self.capacitance * frequency))
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Complex;

    use crate::{elements::Element, NodeId};

    use super::Capacitor;

    /// Test if the impedance for the capacitor is correctly calculated.
    #[test]
    fn impedance() {
        let capacitor = Capacitor::new(10.0, NodeId(0), NodeId(1));
        assert_eq!(
            capacitor.impedance(1000.0),
            Complex::<f32>::new(0.0, -0.0001)
        )
    }
}
