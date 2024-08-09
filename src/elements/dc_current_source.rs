use nalgebra::Complex;

use crate::NodeId;

use super::{Element, Terminal};

#[derive(Clone, Copy)]
pub struct DCCurrentSource {
    current: f32,
    terminals: [Terminal; 2],
    resistance: f32,
}

impl DCCurrentSource {
    pub fn new(current: f32, positive_node: NodeId, negative_node: NodeId) -> Self {
        Self {
            current,
            terminals: [
                Terminal::new(positive_node, super::Polarity::Positive),
                Terminal::new(negative_node, super::Polarity::Negative),
            ],
            resistance: 0.0,
        }
    }

    #[must_use]
    pub fn with_resistance(mut self, resistance: f32) -> Self {
        self.resistance = resistance;
        self
    }
}

impl Element for DCCurrentSource {
    fn terminals(&self) -> &[Terminal] {
        &self.terminals
    }

    fn stamp(&self, _a_matrix: &mut Vec<f32>, z_vector: &mut Vec<f32>, _n: usize, _m: usize) {
        let terminal_1 = self.terminals()[0];
        let terminal_2 = self.terminals()[1];
        let node_1 = terminal_1.node.0;
        let node_2 = terminal_2.node.0;

        if node_1 > 0 {
            z_vector[node_1 - 1] += terminal_1.sign() * self.dc_current();
        }
        if node_2 > 0 {
            z_vector[node_2 - 1] += terminal_2.sign() * self.dc_current();
        }
    }

    fn dc_voltage(&self) -> f32 {
        0.0
    }

    fn ac_voltage(&self) -> Complex<f32> {
        Complex::ZERO
    }

    fn dc_current(&self) -> f32 {
        self.current
    }

    fn ac_current(&self) -> Complex<f32> {
        Complex::ZERO
    }

    fn resistance(&self) -> f32 {
        self.resistance
    }

    fn impedance(&self, _frequency: f32) -> Complex<f32> {
        Complex::ZERO
    }
}
