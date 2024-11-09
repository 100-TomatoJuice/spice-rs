use nalgebra::Complex;

use crate::NodeId;

use super::{Element, Terminal};

#[derive(Default, Debug, Clone, Copy)]
pub struct ACVoltageSource {
    voltage: Complex<f32>,
    terminals: [Terminal; 2],
}

impl ACVoltageSource {
    pub fn new(voltage: Complex<f32>, positive_node: NodeId, negative_node: NodeId) -> Self {
        Self {
            voltage,
            terminals: [
                Terminal::new(positive_node, super::Polarity::Positive),
                Terminal::new(negative_node, super::Polarity::Negative),
            ],
        }
    }
}

impl Element for ACVoltageSource {
    fn terminals(&self) -> &[Terminal] {
        &self.terminals
    }

    fn stamp(&self, _a_matrix: &mut Vec<f32>, _z_vector: &mut Vec<f32>, _n: usize, _m: usize) {}

    fn dc_voltage(&self) -> f32 {
        0.0
    }

    fn ac_voltage(&self) -> Complex<f32> {
        self.voltage
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
