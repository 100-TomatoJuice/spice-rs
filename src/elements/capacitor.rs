use nalgebra::Complex;
use petgraph::graph::NodeIndex;

use super::{Element, Terminal};

#[derive(Default, Clone)]
pub struct Capacitor {
    capacitance: f32,
    terminals: [Terminal; 2],
}

impl Capacitor {
    pub fn new(capacitance: f32, positive_node: NodeIndex, negative_node: NodeIndex) -> Self {
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
    use petgraph::graph::NodeIndex;

    use crate::elements::Element;

    use super::Capacitor;

    /// Test if the impedance for the capacitor is correctly calculated.
    #[test]
    fn impedance() {
        let capacitor = Capacitor::new(10.0, NodeIndex::new(0), NodeIndex::new(1));
        assert_eq!(
            capacitor.impedance(1000.0),
            Complex::<f32>::new(0.0, -0.0001)
        )
    }
}
