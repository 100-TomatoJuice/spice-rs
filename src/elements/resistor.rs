use nalgebra::Complex;
use petgraph::graph::NodeIndex;

use super::{Element, Terminal};

#[derive(Default, Clone, Copy)]
pub struct Resistor {
    resistance: f32,
    terminals: [Terminal; 2],
}

impl Resistor {
    pub fn new(resistance: f32, node1: NodeIndex, node2: NodeIndex) -> Self {
        Self {
            resistance,
            terminals: [
                Terminal::new(node1, super::Polarity::Neutral),
                Terminal::new(node2, super::Polarity::Neutral),
            ],
        }
    }
}

impl Element for Resistor {
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
        self.resistance
    }

    /// The rectangular impedence of a resistor is equal to `R + j0`,
    /// where `R` is the resistance in Ohms.
    fn impedance(&self, _frequency: f32) -> Complex<f32> {
        Complex::new(self.resistance, 0.0)
    }
}
