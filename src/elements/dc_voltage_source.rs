use nalgebra::Complex;

use crate::NodeId;

use super::{Element, Terminal};

#[derive(Clone)]
pub struct DCVoltageSource {
    voltage: f32,
    terminals: [Terminal; 2],
    index: usize,
}

impl DCVoltageSource {
    pub fn new(voltage: f32, positive_node: NodeId, negative_node: NodeId, index: usize) -> Self {
        Self {
            voltage,
            terminals: [
                Terminal::new(positive_node, super::Polarity::Positive),
                Terminal::new(negative_node, super::Polarity::Negative),
            ],
            index,
        }
    }
}

impl Element for DCVoltageSource {
    fn terminals(&self) -> &[Terminal] {
        &self.terminals
    }

    /// Stamps itself onto the B and C matrix, which are both apart of the A matrix,
    /// and onto the z_vector.
    fn stamp(&self, a_matrix: &mut Vec<f32>, z_vector: &mut Vec<f32>, n: usize, m: usize) {
        let terminal_1 = self.terminals()[0];
        let terminal_2 = self.terminals()[1];
        let node_1 = terminal_1.node.0;
        let node_2 = terminal_2.node.0;

        let n = n - 1;
        match (node_1 > 0, node_2 > 0) {
            (true, true) => {
                // B matrix
                a_matrix[(n + m) * (n + self.index) + node_1 - 1] += terminal_1.sign();
                a_matrix[(n + m) * (n + self.index) + node_2 - 1] += terminal_2.sign();

                // C matrix
                a_matrix[(n + m) * (node_1 - 1) + n + self.index] += terminal_1.sign();
                a_matrix[(n + m) * (node_2 - 1) + n + self.index] += terminal_2.sign();

                // z vector
                z_vector[n + self.index] = self.dc_voltage();
            }
            (true, false) => {
                // B matrix
                a_matrix[(n + m) * (n + self.index) + node_1 - 1] += terminal_1.sign();

                // C matrix
                a_matrix[(n + m) * (node_1 - 1) + n + self.index] += terminal_1.sign();

                // z vector
                z_vector[n + self.index] = self.dc_voltage();
            }
            (false, true) => {
                // B matrix
                a_matrix[(n + m) * (n + self.index) + node_2 - 1] += terminal_2.sign();

                // C matrix
                a_matrix[(n + m) * (node_2 - 1) + n + self.index] += terminal_2.sign();

                // z vector
                z_vector[n + self.index] = self.dc_voltage();
            }
            _ => (),
        }
    }

    fn is_b_c_element(&self) -> bool {
        true
    }

    fn dc_voltage(&self) -> f32 {
        self.voltage
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
        0.0
    }

    fn impedance(&self, _frequency: f32) -> Complex<f32> {
        Complex::ZERO
    }
}
