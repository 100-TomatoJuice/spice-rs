use nalgebra::Complex;

use crate::NodeId;

use super::{Element, Terminal};

#[derive(Default, Debug, Clone, Copy)]
pub struct Resistor {
    resistance: f32,
    terminals: [Terminal; 2],
}

impl Resistor {
    pub fn new(resistance: f32, node1: NodeId, node2: NodeId) -> Self {
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

    /// Stamps itself onto the G matrix, which is apart of the A matrix.
    fn stamp(&self, a_matrix: &mut Vec<f32>, _z_vector: &mut Vec<f32>, n: usize, m: usize) {
        let node_1 = self.terminals()[0].node.0;
        let node_2 = self.terminals()[1].node.0;

        match (node_1 > 0, node_2 > 0) {
            // Neither node is ground
            (true, true) => {
                a_matrix[(node_1 - 1) * (n + m)] += self.conductance();
                a_matrix[(node_2 - 1) + (node_1 - 1) * (n - 1 + m)] -= self.conductance();

                a_matrix[(node_2 - 1) * (n + m)] += self.conductance();
                a_matrix[(node_1 - 1) + (node_2 - 1) * (n - 1 + m)] -= self.conductance();
            }
            // Only node 2 is ground
            (true, false) => {
                a_matrix[(node_1 - 1) * (n + m)] += self.conductance();
            }
            // Only node 1 is ground
            (false, true) => {
                a_matrix[(node_2 - 1) * (n + m)] += self.conductance();
            }
            // Both nodes are ground
            _ => (),
        }
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
